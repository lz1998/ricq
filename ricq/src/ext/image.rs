use std::future::Future;
use std::pin::Pin;

use ricq_core::command::img_store::GroupImageStoreResp;
use ricq_core::command::long_conn::OffPicUpResp;
use ricq_core::highway::BdhInput;
use ricq_core::msg::elem::{FriendImage, GroupImage};
use ricq_core::{RQError, RQResult};

use crate::structs::ImageInfo;
use crate::Client;

pub async fn upload_group_image_ext<F>(
    cli: &Client,
    group_code: i64,
    image_info: ImageInfo,
    f: F,
) -> RQResult<GroupImage>
where
    F: for<'a> FnOnce(
        &'a ImageInfo,
    ) -> Pin<Box<dyn Future<Output = RQResult<Vec<u8>>> + Send + 'a>>,
{
    let sign = cli.get_highway_session_key().await;
    let group_image = match cli.get_group_image_store(group_code, &image_info).await? {
        GroupImageStoreResp::Exist { file_id, addrs } => {
            image_info.into_group_image(file_id, addrs.first().cloned().unwrap_or_default(), sign)
        }
        GroupImageStoreResp::NotExist {
            file_id,
            upload_key,
            mut upload_addrs,
        } => {
            let data = f(&image_info).await?;
            let addr = upload_addrs
                .pop()
                .ok_or(RQError::EmptyField("upload_addrs"))?;
            cli.highway_upload_bdh(
                addr.clone().into(),
                BdhInput {
                    command_id: 2,
                    ticket: upload_key,
                    ext: vec![],
                    encrypt: false,
                    chunk_size: 256 * 1024,
                    send_echo: true,
                },
                &data,
            )
            .await
            .map(|_| image_info.into_group_image(file_id, addr, sign))?
        }
    };
    Ok(group_image)
}

pub async fn upload_friend_image_ext<F>(
    cli: &Client,
    target: i64,
    image_info: ImageInfo,
    f: F,
) -> RQResult<FriendImage>
where
    F: for<'a> FnOnce(
        &'a ImageInfo,
    ) -> Pin<Box<dyn Future<Output = RQResult<Vec<u8>>> + Send + 'a>>,
{
    let friend_image = match cli.get_off_pic_store(target, &image_info).await? {
        OffPicUpResp::Exist { res_id, uuid } => image_info.into_friend_image(res_id, uuid),
        OffPicUpResp::UploadRequired {
            res_id,
            uuid,
            upload_key,
            mut upload_addrs,
        } => {
            let data = f(&image_info).await?;
            let addr = upload_addrs
                .pop()
                .ok_or(RQError::EmptyField("upload_addrs"))?;
            cli.highway_upload_bdh(
                addr.clone().into(),
                BdhInput {
                    command_id: 1,
                    ticket: upload_key,
                    ext: vec![],
                    encrypt: false,
                    chunk_size: 256 * 1024,
                    send_echo: true,
                },
                &data,
            )
            .await
            .map(|_| image_info.into_friend_image(res_id, uuid))?
        }
    };
    Ok(friend_image)
}

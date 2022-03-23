use std::future::Future;
use std::pin::Pin;

use rq_engine::command::img_store::GroupImageStoreResp;
use rq_engine::command::long_conn::OffPicUpResp;
use rq_engine::msg::elem::{FriendImage, GroupImage};
use rq_engine::{RQError, RQResult};

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
    let sign = cli._get_highway_session_key().await;
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
                .ok_or_else(|| RQError::Other("addrs is empty".into()))?;
            cli._upload_group_image(upload_key, addr.clone().into(), data)
                .await
                .and_then(|_| Ok(image_info.into_group_image(file_id, addr, sign)))?
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
                .ok_or_else(|| RQError::Other("addrs is empty".into()))?;
            cli._upload_friend_image(upload_key, addr.clone().into(), data)
                .await
                .and_then(|_| Ok(image_info.into_friend_image(res_id, uuid)))?
        }
    };
    Ok(friend_image)
}

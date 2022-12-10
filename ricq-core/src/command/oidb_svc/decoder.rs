use std::collections::HashMap;

use bytes::{Bytes, BytesMut};

use crate::command::oidb_svc::GroupAtAllRemainInfo;
use crate::structs::{
    GroupFileCount, GroupFileInfo, GroupFileItem, GroupFileList, GroupFolderInfo, GroupInfo,
    GroupMemberPermission,
};
use crate::{pb, RQResult};
use prost::Message;

use super::OcrResponse;

impl super::super::super::Engine {
    // OidbSvc.0x88d_0
    pub fn decode_group_info_response(&self, payload: Bytes) -> RQResult<Vec<GroupInfo>> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let groups = pb::oidb::D88dRspBody::decode(&*pkg.bodybuffer)?.rsp_group_info;
        Ok(groups
            .into_iter()
            .filter_map(|g| {
                let code = g.group_code? as i64;
                let info = g.group_info?;
                Some(GroupInfo {
                    uin: info.group_uin? as i64,
                    code,
                    name: String::from_utf8_lossy(&info.group_name?).into_owned(),
                    memo: String::from_utf8_lossy(&info.group_memo?).into_owned(),
                    owner_uin: info.group_owner? as i64,
                    group_create_time: info.group_create_time.unwrap_or_default(),
                    group_level: info.group_level.unwrap_or_default(),
                    member_count: info.group_member_num? as u16,
                    max_member_count: info.group_member_max_num? as u16,
                    shut_up_timestamp: info.shutup_timestamp.unwrap_or_default() as i64,
                    my_shut_up_timestamp: info.shutup_timestamp_me.unwrap_or_default() as i64,
                    last_msg_seq: info.group_cur_msg_seq.unwrap_or_default() as i64,
                })
            })
            .collect())
    }

    // // OidbSvc.0x8a7_0
    pub fn decode_group_at_all_remain_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GroupAtAllRemainInfo> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let rsp = pb::oidb::D8a7RspBody::decode(&*pkg.bodybuffer)?;
        Ok(GroupAtAllRemainInfo {
            can_at_all: rsp.can_at_all(),
            remain_at_all_count_for_group: rsp.remain_at_all_count_for_group(),
            remain_at_all_count_for_uin: rsp.remain_at_all_count_for_uin(),
        })
    }

    // OidbSvc.0x990
    pub fn decode_translate_response(&self, payload: Bytes) -> RQResult<Vec<String>> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let rsp = pb::oidb::TranslateRspBody::decode(&*pkg.bodybuffer)?;
        Ok(rsp.batch_translate_rsp.unwrap_or_default().dst_text_list)
    }

    // OidbSvc.0xeac_1/2
    pub fn decode_essence_msg_response(&self, payload: Bytes) -> RQResult<pb::oidb::EacRspBody> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::oidb::EacRspBody::decode(&*pkg.bodybuffer)?;
        Ok(resp)
    }

    // OidbSvc.0xe07_0
    pub fn decode_image_ocr_response(&self, payload: Bytes) -> RQResult<OcrResponse> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::oidb::De07RspBody::decode(&*pkg.bodybuffer)?;
        Ok(OcrResponse {
            texts: resp
                .ocr_rsp_body
                .clone()
                .unwrap_or_default()
                .text_detections,
            language: resp.ocr_rsp_body.unwrap_or_default().language,
        })
    }

    // OidbSvc.0x899_0
    pub fn decode_get_group_admin_list_response(
        &self,
        payload: Bytes,
    ) -> RQResult<HashMap<i64, GroupMemberPermission>> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::cmd0x899::RspBody::decode(&*pkg.bodybuffer)?;
        Ok(resp
            .memberlist
            .into_iter()
            .map(|mem| {
                (
                    mem.member_uin.unwrap_or_default() as i64,
                    if mem.privilege == Some(1) {
                        GroupMemberPermission::Owner
                    } else if mem.privilege == Some(2) {
                        GroupMemberPermission::Administrator
                    } else {
                        GroupMemberPermission::Member
                    },
                )
            })
            .collect())
    }
    // OidbSvc.0x6d8_1
    pub fn decode_group_file_list_response(&self, payload: Bytes) -> RQResult<GroupFileList> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::oidb::D6d8RspBody::decode(&*pkg.bodybuffer)?;
        let resp = &resp.file_list_info_rsp.unwrap_or_default();
        Ok(GroupFileList {
            all_file_count: resp.all_file_count(),
            is_end: resp.is_end(),
            items: resp
                .item_list
                .clone()
                .into_iter()
                .map(|f| {
                    if let Some(fi) = f.file_info {
                        let folder_info = f.folder_info.unwrap_or_default();
                        GroupFileItem {
                            file_info: GroupFileInfo {
                                file_id: fi.file_id().to_string(),
                                bus_id: fi.bus_id(),
                                file_name: fi.file_name().to_string(),
                                sha: format!("{:x}", BytesMut::from(fi.sha())),
                                dead_time: fi.dead_time(),
                                file_size: fi.file_size(),
                                upload_time: fi.upload_time(),
                                uploader_uin: fi.uploader_uin(),
                                uploader_name: fi.uploader_name().to_string(),
                                parent_folder_id: fi.parent_folder_id().to_string(),
                                local_path: fi.local_path().to_string(),
                                modify_time: fi.modify_time(),
                                download_times: fi.download_times(),
                                md5: Bytes::from(fi.md5.unwrap_or_default()),
                                sha3: Bytes::from(fi.sha3.unwrap_or_default()),
                                uploaded_size: fi.uploaded_size.unwrap_or_default(),
                            },
                            folder_info: GroupFolderInfo {
                                create_time: folder_info.create_time(),
                                create_uin: folder_info.create_uin(),
                                creator_name: folder_info.creator_name.unwrap_or_default(),
                                folder_id: folder_info.folder_id.unwrap_or_default(),
                                folder_name: folder_info.folder_name.unwrap_or_default(),
                                modify_time: folder_info.modify_time.unwrap_or_default(),
                                parent_folder_id: folder_info.parent_folder_id.unwrap_or_default(),
                                total_file_count: folder_info.total_file_count.unwrap_or_default(),
                            },
                            r#type: f.r#type.unwrap_or_default(),
                        }
                    } else {
                        GroupFileItem::default()
                    }
                })
                .collect(),
            next_index: resp.next_index(),
            role: resp.role(),
        })
    }
    // OidbSvc.0x6d6_2
    pub fn decode_group_file_download_response(
        &self,
        payload: Bytes,
        filename: &str,
    ) -> RQResult<String> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::oidb::D6d6RspBody::decode(&*pkg.bodybuffer)?;
        let f_rsp = resp.download_file_rsp.unwrap();
        Ok(format!(
            "http://{}/ftn_handler/{:x}/?fname={}",
            f_rsp.download_ip(),
            BytesMut::from(f_rsp.download_url()),
            filename
        ))
    }
    // OidbSvc.0x6d8_1
    pub fn decode_group_file_count_response(&self, payload: Bytes) -> RQResult<GroupFileCount> {
        let pkg = pb::oidb::OidbssoPkg::decode(&*payload)?;
        let resp = pb::oidb::D6d8RspBody::decode(&*pkg.bodybuffer)?;
        if let Some(file_count_rsp) = resp.file_count_rsp {
            Ok(GroupFileCount {
                is_full: file_count_rsp.is_full.unwrap_or_default(),
                all_file_count: file_count_rsp.all_file_count.unwrap_or_default(),
                limit_count: file_count_rsp.limit_count.unwrap_or_default(),
                file_too_many: file_count_rsp.file_too_many.unwrap_or_default(),
            })
        } else {
            Err(crate::RQError::GetFileCountFailed)
        }
    }
}

use std::collections::HashMap;

use bytes::Bytes;

use crate::command::oidb_svc::GroupAtAllRemainInfo;
use crate::structs::{GroupInfo, GroupMemberPermission};
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
                    name: String::from_utf8_lossy(&info.group_name?).to_string(),
                    memo: String::from_utf8_lossy(&info.group_memo?).to_string(),
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
}

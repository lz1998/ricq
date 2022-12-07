pub struct LinkShare {
    pub title: String,
    pub summary: Option<String>,
    /// 从消息列表中看到的文字,默认为 "[分享]" + title
    pub brief: Option<String>,
    /// 预览图网址, 默认为 QQ 浏览器图标,似乎对域名有限制
    pub picture_url: Option<String>,
    pub url: String,
}

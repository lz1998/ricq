use crate::client::{income::decoder, structs};
use async_trait::async_trait;

/// 所有需要外发的数据的枚举打包
#[derive(Debug)]
pub enum Msgs {
    GroupMessage(decoder::online_push::GroupMessagePart),
    FriendList(decoder::friendlist::FriendListResponse),
    GroupMemberInfo(structs::GroupMemberInfo),
}

/// 处理外发数据的接口，大概会需要写成这个样子
/// 
/// ``` rust
/// use tokio::sync::mpsc;
/// use async_trait::async_trait;
/// 
/// struct NewHandler {
///    sender: mpsc::Sender<T>,
/// }
/// 
/// #[async_trait]
/// impl Handler for NewHandler {
///     async fn handle(&self, msg: Msgs) {
///         let t = match msg {
///         ... /// transform Msgs to T
///         };
///         self.sender.send(t).await.unwrap();
///     }
/// }
#[async_trait]
pub trait Handler {
    async fn handle(&self, msgs: Msgs) -> Result<(), Box<dyn std::error::Error>>;
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, msgs: Msgs) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", msgs);
        Ok(())
    }
}

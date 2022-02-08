use crate::protocol::packet::Packet;

use self::engine_traits::EngineTrait;

 mod engine_traits;
 mod transport_traits;

pub use engine_traits::PacketBuildEngine;

pub mod engine{
    pub use super::engine_traits::{
        EnginePacket,
        EngineTrait,
        GetQQId,
        GetTransport,
        NextSeq
    };
}

pub mod transport{
    pub use super::transport_traits::{
        TransportEncoder,
        TransportDecoder,
        TransportInner,
        TransportTrait
    };
}

/// packet Builder 关联类型Args 的特征
pub trait BuilderArgs {
    /// Args 与 builder 形成双向绑定
    /// 这样可以借助 类型系统推断泛型类型
    type Builder: PacketBuilder;
}

/// Packet 构造器   
/// 通过提供的 `args` 构造出 `body`   
/// 再根据 `packet command` 等信息 构造出 `Packet`  
pub trait PacketBuilder {
    /// Packet Build 所需要的参数
    /// 使用时会取得所有权
    type Args: BuilderArgs<Builder = Self>;

    /// Packet Command 获取PacketCommand
    /// ## Notice
    /// 该方法与 `packet` 方法至少实现一个
    fn packet_command() -> &'static str {
        unimplemented!("Invoke Unimplemented fn")
    }

    /// build 根据 arg 参数生成 packet 的 body 部分
    /// 输出为 `Bytes`
    fn build<E: EngineTrait>(engine: &E, args: Self::Args) -> bytes::Bytes;
    /// packet 将生成的Packet Body 进一步打包为 Packet
    /// ## Notice
    /// 该方法与`packet_command` 方法至少实现一个
    fn packet<E: EngineTrait>(engine: &E, body: bytes::Bytes) -> Packet {
        engine.uin_packet(&Self::packet_command(), body)
    }
}

#[macro_export]
/// 用于快捷构造Args 结构体的Marco
///
/// ## Example
/// ```rust
/// arg_struct!( MockArgs | Mock {
///     a:i32
///     b:u32
/// });
/// ```
/// 其中
/// - `MockArgs` 为生成的Args结构体，
/// - `Mock` 为与Args绑定的Builder，未提供将会进行自我绑定
/// - 大括号内部为Args内部的数据，多个数据之间用 *空格* 区分
macro_rules! arg_struct {
    ($v:vis $name:ident {$( $iv:vis $var:ident : $t:ty )*} )=>{
        arg_struct!($v $name | $name
            {
            $( $iv $var : $t ) *
        }
        );
    };
    ($v:vis $name:ident | $build:ident {$( $iv:vis $var:ident : $t:ty )*} ) => {
        $v struct $name {
            $(
                $iv $var : $t
            ),*
        }

        impl $crate::packet_builder::BuilderArgs for $name{
            type Builder = $build ;
        }

        impl $name {
            $v fn new(
                $(
                    $var : $t
                ),*
            )->Self{
                Self{
                    $(
                        $var
                    ),*
                }
            }
        }

        impl From<(
            $(
                $t
            ),*
        )> for $name{
            fn from(
            (
                $(
                    $var
                ),*
            ):(
                $(
                    $t
                ),*
            ))->Self{
                Self{
                    $(
                        $var
                    ),*
                }
            }
        }
    };
}



#[cfg(test)]
mod test_pkt_build {
    use crate::packet_builder::engine_traits::PacketBuildEngine;
    use crate::protocol::packet::Packet;
    use crate::protocol::transport::Transport;

    use super::engine_traits::{EnginePacket, EngineTrait, GetQQId, GetTransport, NextSeq};
    use super::PacketBuilder;

    struct MockEngine;

    impl GetQQId for MockEngine {
        fn get_id(&self) -> i64 {
            1
        }
    }

    impl NextSeq for MockEngine {
        fn next_seq(&self) -> u16 {
            1
        }

        fn next_packet_seq(&self) -> i32 {
            1
        }

        fn next_group_seq(&self) -> i32 {
            1
        }

        fn next_friend_seq(&self) -> i32 {
            1
        }

        fn next_group_data_trans_seq(&self) -> i32 {
            1
        }

        fn next_highway_apply_seq(&self) -> i32 {
            1
        }
    }

    impl GetTransport for MockEngine {
        type Transport = Transport;
        fn get_transport(&self) -> &Transport {
            unimplemented!()
        }
    }

    impl EnginePacket for MockEngine {
        fn uin_packet(&self, command: &str, body: bytes::Bytes) -> Packet {
            Packet {
                command_name: command.to_string(),
                body,
                message: "".into(),
                ..Default::default()
            }
        }

        fn oicq_request(&self, _uin: i64, _command_id: u16, _body: &[u8]) -> bytes::Bytes {
            todo!()
        }
    }

    impl EngineTrait for MockEngine {}
    struct Mock;

    arg_struct!( MockArgs | Mock {
        a:i32
        b:u32
    });

    impl PacketBuilder for Mock {
        type Args = MockArgs;

        fn build<E: EngineTrait>(_engine: &E, args: Self::Args) -> bytes::Bytes {
            let MockArgs { a, b } = args;
            println!("{}{}", a, b);
            bytes::Bytes::from_static(&[1, 1, 4, 5, 1, 4])
        }

        fn packet_command() -> &'static str {
            "Mock.Command"
        }
    }

    #[test]
    fn test_mock() {
        let engine = MockEngine;
        let args = MockArgs::new(11, 12);
        let pkt = engine.build_packet(args);

        println!("{:?}", pkt)
    }
}

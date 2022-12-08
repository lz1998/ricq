# RICQ

![](https://socialify.git.ci/lz1998/ricq/image?forks=1&issues=1&language=1&owner=1&pattern=Circuit%20Board&pulls=1&stargazers=1&theme=Dark)

QQ Android 协议的 Rust 实现，移植于 [OICQ](https://github.com/takayama-lily/oicq)。

- [ricq](https://crates.io/crates/ricq)：提供异步 API
- [ricq-core](https://crates.io/crates/ricq-core)：不带 IO 的数据包构造器、解析器（通常用于 FFI）
- [ricq-axum-api](https://github.com/lz1998/ricq/tree/master/examples/ricq-axum-api)：提供 HTTP API 形式的登录接口，配合 [ricq-react-ui](https://github.com/lz1998/ricq-react-ui)，只需要开发登录后的逻辑。

## 如何使用

本项目是协议 Lib，如果需要直接使用，可以参考 [examples](https://github.com/lz1998/ricq/tree/master/examples) 中的例子进行开发。

普通开发者推荐使用 SDK、框架进行开发：

| 框架 / SDK | 语言 | 说明 |
| ---- | ---- | ---- |
| [rust_proc_qq](https://github.com/niuhuan/rust_proc_qq) | Rust | 模仿rocket |
| [Walle-Q](https://github.com/abrahum/walle-q) | - | onebot协议 |
| [pbrq](https://github.com/ProtobufBot/pbrq) | - | websocket+protobuf协议（附带[Web-UI](https://github.com/ProtobufBot/pbrq-react-ui)） |
| [atri_qq](https://github.com/LaoLittle/atri_qq) | - | 加载原生动态库插件，高性能低占用 |
| [awr](https://github.com/Wybxc/awr) | Python | 基于 ricq 包装，供 Python 使用的 QQ 无头客户端。 |

> 本项目是一个年轻的项目，请使用 Nightly 工具链构建本项目哦（正经人谁用 Stable 啊）

## 相关项目

| 项目 | 描述 |
| ---- | ---- |
| [lomirus/gtk-qq](https://github.com/lomirus/gtk-qq) | Unofficial Linux QQ client, based on GTK4 and libadwaita, developed with Rust and Relm4. |
| [a1967629423/esp32c3-rs-qq](https://github.com/a1967629423/esp32c3-rs-qq) | 在单片机上运行QQ |

## 已完成功能 / 开发计划

### 登录

- [x] 账号密码登录
- [x] 二维码登录
- [x] 验证码提交
- [x] 设备锁验证
- [x] 错误信息解析

### 消息类型

- [x] 文本
- [x] 表情
- [x] At
- [x] 回复
- [x] 匿名
- [x] 骰子
- [x] 石头剪刀布
- [x] 图片
- [x] 语音
- [x] 长消息（仅支持群聊发送）
- [x] 合并转发（仅支持群聊发送）
- [x] 链接分享
- [ ] 小程序（暂只支持 RAW）
- [ ] 短视频
- [ ] 群文件（上传与接收信息）

### 事件

- [x] 群消息
- [x] 好友消息
- [x] 新好友请求
- [x] 收到其他用户进群请求
- [x] 新好友
- [x] 群禁言
- [x] 好友消息撤回
- [x] 群消息撤回
- [x] 收到邀请进群请求
- [x] 群名称变更
- [x] 好友删除
- [x] 群成员权限变更
- [x] 新成员进群 / 退群
- [x] 登录号加群
- [x] 临时会话消息
- [x] 群解散
- [x] 登录号退群（包含踢出）
- [x] 客户端离线
- [ ] 群提示（戳一戳 / 运气王等）

### 主动操作

> 为防止滥用，将不支持主动邀请新成员进群

- [x] 修改昵称
- [x] 发送群消息
- [x] 获取群列表
- [x] 获取群成员列表
- [x] 获取好友列表 / 分组
- [x] 获取好友个性签名
- [x] 添加 / 删除 / 重命名好友分组
- [x] 群成员禁言 / 解除禁言
- [x] 踢出群成员
- [x] 戳一戳群友
- [x] 戳一戳好友
- [x] 设置群管理员
- [x] 设置群公告
- [x] 设置群名称
- [x] 全员禁言
- [x] 获取群@全体剩余次数
- [x] 翻译
- [x] 修改群成员头衔
- [x] 设置群精华消息
- [x] 发送好友消息
- [x] 发送临时会话消息
- [x] 修改群成员 Card
- [x] 撤回群消息
- [x] 撤回好友消息
- [x] 处理被邀请加群请求
- [x] 处理加群请求
- [x] 处理好友请求
- [x] 删除好友
- [x] 获取陌生人信息
- [x] 设置在线状态
- [x] 修改个人资料
- [x] 修改个性签名
- [x] 获取群文件下载链接
- [ ] 获取群荣誉（龙王 / 群聊火焰等）
- [ ] ~~群成员邀请~~

### 敏感操作

> 由于 [QQ 钱包支付用户服务协议](https://www.tenpay.com/v2/html5/basic/public/agreement/protocol_mqq_pay.shtml), 将不提供一切有关 QQ 钱包的功能。

> 4.13 您不得利用本服务实施下列任一的行为：
> \
> （9） **侵害 QQ 钱包支付服务系統；**

- [ ] ~~QQ 钱包协议（收款 / 付款等）~~

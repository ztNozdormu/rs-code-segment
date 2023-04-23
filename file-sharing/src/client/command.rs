use std::error::Error;

use futures::channel::oneshot;
use libp2p::{Multiaddr, PeerId};

#[derive(Debug)]
pub enum Command {
    // 监听本地端口命令
    StartListening {
    // 本地监听地址
    addr: Multiaddr,
    // 用于发送命令执行状态的通道
    sender: oneshot::Sender<Result<(),Box<dyn Error+Send>>>,
    },
    // 链接给定节点
    Dial {
        // 节点
        peer_id: PeerId,
        peer_addr: Multiaddr,
        sender: oneshot::Sender<Result<(),Box<dyn Error+Send>>>,
    },
    // 宣称本节点提供共享文件命令
    StartProviding{
        file_name: String,
        sender: oneshot::Sender<()>,
    },
    // 获取提供共享文件的节点命令
    GetProviders {
        file_name: String,
        sender: oneshot::Sender<HashSet<PeerId>>,
    },
    // 请求共享文件命令
    RequestFile {
        file_name: String,
        peer_id: PeerId,
        sender: oneshot::Sender<Result<String,Box<dyn Error+Send>>>,
    },
    // 返回共享文件内容命令
    ResponseFile {
        file_name: String,
        channel: ResponseChannel<String>,
    },

}
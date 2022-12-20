参考地址:
>https://mp.weixin.qq.com/s?__biz=MzI1MjAzNDI1MA==&mid=2648216691&idx=3&sn=307bfaeb10ebe67e8b91b830764cb8dd&chksm=f1c5d9d6c6b250c08fbe128ef497f98555f3f493f0d3ec5900bf09b09cb19c054af892550843&scene=27
### 解析命令行参数
```
    我们使用clap库来解析命令行参数，命令行格式如下：

    /// 启动提供文件的网络节点
    file-sharing --listen-address <监听地址> \
                --secret-key-seed <生成密钥对的种子> \
                provide \
                --path <文件全路径> \
                --name <文件名称>

    /// 链接提供文件的节点，并获取文件内容             
    file-sharing --peer <节点地址/PeerId> \
                get \
                --name <文件名称>
```
### 测试命令
1. 上传文件命令
```
cargo run -- \
          --listen-address /ip4/127.0.0.1/tcp/40837 \
          --secret-key-seed 1 \
          provide \
          --path /Users/Justin/sharing_file.txt \
          --name sharing_file
```
2. 获取文件命令
```
cargo run -- \
          --peer /ip4/127.0.0.1/tcp/40837/p2p/12D3KooWPjceQrSwdWXPyLLeABRXmuqt69Rg3sBYbU1Nft9HyQ6X \
          get \
          --name sharing_file
```
use std::error::Error;

pub mod arg;

pub use arg::{CliArgument, Opt};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();
    process_args(opt).await?;

    Ok(())
}

async fn process_args(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.listen_address {
        Some(address) => println!("address: {:?}", address),
        None => println!("address default:/ip4/0.0.0.0/tcp/0"),
    }

    if let Some(address) = opt.peer {
        println!("节点address: {:?}", address);
    }
    match opt.cli_argument {
        CliArgument::Provide { path, name } => {
            print!("上传的文件名称name:{:},文件绝对路径:{:?}", name, path);
        }
        CliArgument::Get { name } => println!("获取文件名称:{:}", name),
    }

    Ok(())
}

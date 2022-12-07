use std::env;
pub mod file;

use file::file_read;

fn main() {

    // let args : Vec<String> = env::args().collect();

    let path =  "/home/learn/rsproject/rustlearn/read-file/src/file/sum_text";//&args[1];

    let fileInfo = file_read::FileIno::new(path);

    let r = fileInfo.read_file();
    
    match r {
        Ok(sum) => println!("读取文件内容累加结果{}",sum) ,
        Err(err) => println!("错误信息{:?}",err),
    }

}

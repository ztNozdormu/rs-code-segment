pub mod parser;

fn main() {
    let result = parser::untyped_examples();
    match result {
        Ok(_) => println!("解析数据成功!"),
        Err(err) => println!("解析数据发生异常{}", err),
    }
    println!("Hello, world!");
}

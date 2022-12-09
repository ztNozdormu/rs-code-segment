use std::fs;
pub mod parser;

fn main() {
    let result = parser::untyped_examples();
    match result {
        Ok(_) => println!("解析数据成功!"),
        Err(err) => println!("解析数据发生异常{}", err),
    }
    let path = "data/test.json".to_owned();
    let data = fs::read_to_string(path).expect("Couldn't find or load that file.");
    let result1 = parser::parser_str(&data);
    match result1 {
        Ok(_) => println!("解析文件内容数据成功!"),
        Err(err) => println!("解析文件内容数据发生异常{}", err),
    }
    let result2 = parser::parser_type(&data);
    match result2 {
        Ok(_) => println!("解析文件内容数据为具体结构对象成功!"),
        Err(err) => println!("解析文件内容数据为具体结构对象发生异常{}", err),
    }

    // 嵌套数据结构
    let test1 = "data/test1.json".to_owned();
    let data_theme = fs::read_to_string(test1).expect("Couldn't find or load that file.");
    let theme = parser::parser_theme(&data_theme);
    match theme {
        Ok(v) => {
            // 遍历theme clors 数据
            for (key, value) in v.colors {
                println!("theme colors's key is {},value is {}", key, value);
            }
            // 可选类型验证
            println!(
                "theme breakpoints widescreen(key)'s value is String {:#?}",
                v.breakpoints.get("widescreen")
            );
            println!(
                "theme breakpoints high(key)'s value is Number {:#?}",
                v.breakpoints.get("high")
            );
        }
        Err(err) => println!("解析文件嵌套内容数据为具体结构对象发生异常{}", err),
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}
// 嵌套类型
#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub animation: HashMap<String, String>,
    pub breakpoints: HashMap<String, Breakpoint>,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>,
    pub font_sizes: Vec<i32>,
    pub font_weights: HashMap<String, i32>,
    pub line_heights: HashMap<String, f32>,
    pub space: Vec<i32>,
    pub sizes: HashMap<String, i32>,
    pub radii: HashMap<String, i32>,
    pub shadows: HashMap<String, HashMap<String, String>>,
    pub gradients: HashMap<String, String>,
}
/**
 * 处理可选类型
* 但如果我们的属性是可选类型(比如“大小”单位可以是数字10或字符串10px)？在Typescript中，
 *  我们可以创建这样的类型Size = string | number。在Rust中，这相当于枚举。
* erde支持Enum类型，如果你传递未标记的宏给他们
* 比如: Theme的 breakpoints下的数据map,其key对应的value既可能是字符串类型也可能是数字类型
 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Breakpoint {
    String(String),
    Number(i32),
}
// example
pub fn untyped_examples() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    // 通过 from_str函数将字符串转换为json结构数据
    let v: Value = serde_json::from_str(data)?;
    println!("please Call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}
// parser by data args
pub fn parser_str(data: &str) -> Result<()> {
    let v: Value = serde_json::from_str(data)?;
    println!("please Call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}
// 数据具体结构(类型)化
pub fn parser_type(data: &str) -> Result<()> {
    let v: Person = serde_json::from_str(data)?;
    println!(
        "please Call {} at the number {:#?}",
        v.name,
        v.phones.get(0)
    );
    Ok(())
}

// 序列化 Theme嵌套结构类型
pub fn parser_theme(data: &str) -> Result<Theme> {
    let v: Theme = serde_json::from_str(data)?;
    Ok(v)
}

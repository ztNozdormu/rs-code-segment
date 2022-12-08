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
    animation: HashMap<String, String>,
    breakpoints: HashMap<String, String>,
    colors: HashMap<String, String>,
    fonts: HashMap<String, String>,
    font_sizes: Vec<i32>,
    font_weights: HashMap<String, i32>,
    line_heights: HashMap<String, f32>,
    space: Vec<i32>,
    sizes: HashMap<String, i32>,
    radii: HashMap<String, i32>,
    shadows: HashMap<String, HashMap<String, String>>,
    gradients: HashMap<String, String>,
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

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
    pub breakpoints: HashMap<String, String>,
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

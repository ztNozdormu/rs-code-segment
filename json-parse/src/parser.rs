use serde_json::{Result, Value};

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

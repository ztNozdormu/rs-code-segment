pub mod str_split;
use str_split::SplitStr;
fn main() {
    let remainder_str = "a,b,c,d,e,f,g,h,i,j,k,l";
    let letters: Vec<&str> = SplitStr::new(remainder_str, ",").collect();
    println!("迭代结果:{:?}", letters)
}

// 参考文章:https://mp.weixin.qq.com/s/Air9iZvHV3vhGFezMfLZSA
fn until_char(s: &str, c: char) -> &str {
    SplitStr::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

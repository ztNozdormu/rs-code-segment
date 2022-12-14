pub mod str_split;
use str_split::SplitStr;
fn main() {
    let remainder_str = "a,b,c,d,e,f,g,h,i,j,k,l";
    let letters: Vec<&str> = SplitStr::new(remainder_str, ",").collect();
    println!("迭代结果:{:?}", letters)
}

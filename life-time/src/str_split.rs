// 泛型化Delimiter
pub struct SplitStr<'a, D> {
    #[warn(dead_code)]
    pub remainder: Option<&'a str>,
    #[warn(dead_code)]
    pub delimiter: D,
}

// 定义分隔符号特征
pub trait Delimiter {
    // 返回分隔符在字符串中开始和结束的位置
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// 分隔符一般为&str和char类型  为这两个类型实现自定义的Delimiter特征
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<'a, D> SplitStr<'a, D> {
    pub fn new(remainder: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(remainder),
            delimiter,
        }
    }
}

/**
 * 实现自定义迭代器
 */
impl<'a, D> Iterator for SplitStr<'a, D>
where
    D: Delimiter,
{
    // 迭代的结果也要与StrSplit拥有相同的生命周期，是因为要在StrSplit的成员remainder上做迭代。
    type Item = &'a str;
    // remainder Option前
    // fn next(&mut self) -> Option<Self::Item> {
    //     if let Some(next_index) = self.remainder.find(self.delimiter) {
    //         // 截取符合条件的部分
    //         let until_remainder = &self.remainder[..next_index];
    //         // 剩余部分处理
    //         self.remainder = &self.remainder[next_index + self.delimiter.len()..];
    //         // 返回符合条件的部分字符
    //         Some(until_remainder)
    //     } else if self.remainder.is_empty() {
    //         //  remainder split完成
    //         None
    //     } else {
    //         // 没有符合分隔条件的字符,清空值并返回
    //         let remainder = self.remainder;
    //         // 为什么空字符串可以赋值给self.remainder？这是因为self.remainder的生命周期是&'a str，空字符串的生命周期是&'static str，static的生命周期一直到程序结束。static生命周期大于'a
    //         self.remainder = "";
    //         Some(remainder)
    //     }
    // }
    // 参考文章:https://mp.weixin.qq.com/s/T9p9sTobjKJb8WA7O4A1Qw
    // remainder Option后
    // fn next(&mut self) -> Option<Self::Item> {
    //     // 这里为什么用Some(ref mut remainder)，而不用Some(&mut refmainder) ???
    //     // 这是因为在进行Some(&mut remainder)=self.remainder模式匹配时，remainder会被自动解引用成str类型，而不是可变的&str类型。str类型使用一次后其所有权会被转移 &str可多次使用
    //     if let Some(ref mut remainder) = self.remainder {
    //         if let Some(delimiter_index) = remainder.find(self.delimiter) {
    //             // 截取符合条件的字符
    //             let until_remainder = &remainder[..delimiter_index];
    //             // 剩余部分重新赋值给remainder
    //             *remainder = &remainder[delimiter_index + self.delimiter.len()..];
    //             Some(until_remainder)
    //         } else {
    //             self.remainder.take()
    //         }
    //     } else {
    //         None
    //     }
    // }
    // 参考文章:https://mp.weixin.qq.com/s/Air9iZvHV3vhGFezMfLZSA
    // fn next(&mut self) -> Option<Self::Item> {
    //     let remainder = self.remainder.as_mut()?;
    //         if let Some(delimiter_index) = remainder.find(self.delimiter) {
    //             // 截取符合条件的字符
    //             let until_remainder = &remainder[..delimiter_index];
    //             // 剩余部分重新赋值给remainder
    //             *remainder = &remainder[delimiter_index + self.delimiter.len()..];
    //             Some(until_remainder)
    //         } else {
    //             self.remainder.take()
    //         }
    // }

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            // 截取符合条件的字符
            let until_remainder = &remainder[..delim_start];
            // 剩余部分重新赋值给remainder
            *remainder = &remainder[delim_end..];
            Some(until_remainder)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = SplitStr::new(haystack, " ").collect();
    println!("迭代结果:{:?}", letters);
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
#[test]
fn tail1() {
    let haystack = "a b c d e";
    let letters: Vec<_> = SplitStr::new(haystack, " ").collect();
    println!("迭代结果:{:?}", letters);
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

pub struct SplitStr<'a> {
    #[warn(dead_code)]
    pub remainder: &'a str,
    #[warn(dead_code)]
    pub delimiter: &'a str,
}

impl<'a> SplitStr<'a> {
    pub fn new(remainder: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder,
            delimiter,
        }
    }
}

/**
 * 实现自定义迭代器
 */
impl<'a> Iterator for SplitStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_index) = self.remainder.find(self.delimiter) {
            // 截取符合条件的部分
            let until_remainder = &self.remainder[..next_index];
            // 剩余部分处理
            self.remainder = &self.remainder[next_index + self.delimiter.len()..];
            // 返回符合条件的部分字符
            Some(until_remainder)
        } else if self.remainder.is_empty() {
            //  remainder split完成
            None
        } else {
            // 没有符合分隔条件的字符,清空值并返回
            let remainder = self.remainder;
            self.remainder = "";
            Some(remainder)
        }
    }
}

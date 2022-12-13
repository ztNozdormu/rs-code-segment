pub struct SplitStr<'a> {
    #[warn(dead_code)]
    remainder: &'a str,
    #[warn(dead_code)]
    delimiter: &'a str,
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
        todo!()
    }
}

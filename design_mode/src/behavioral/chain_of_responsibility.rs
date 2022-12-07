
// !责任链是一种行为设计模式，它允许您沿着处理程序链传递请求。

// !在接收到请求后，每个处理程序决定是处理请求还是将其传递给链中的下一个处理程序。

// Handler特征声明了一个用于构建链的方法

//处理程序。它还声明了一个用于执行请求的方法。

pub trait Handler<'a> {
    fn set_next(&mut self,next: &'a dyn Handler<'a>) ->&mut dyn Handler<'a>;
    fn handle(&self,request: &str);
}

pub struct AHandler<'a> {
   name: String,
   next: Option<&'a dyn Handler<'a>>,
}

impl<'a> AHandler<'a> {
    pub fn new(name: String) -> AHandler<'a> {
        AHandler{
            name,
            next: None,
        }
    }
}

impl<'a> Handler<'a> for AHandler<'a> {

   fn set_next(&mut self,next: &'a dyn Handler<'a>) ->&mut dyn Handler<'a> {

       self.next = Some(next);
       self
   }

   fn handle(&self,request: &str) {
      println!("{} handle the request: {}", self.name, request);
      if let Some(v) = &self.next {
        v.handle(request)
      }
    }
}

pub struct BHandler<'a> {
    next: Option<&'a dyn Handler<'a>>
}

impl <'a> BHandler<'a> {
    pub fn new() -> BHandler<'a> {
        BHandler { next: None }
    }
}
    
impl<'a> Handler<'a> for BHandler<'a> {

    fn set_next(&mut self,next: &'a dyn Handler<'a>) ->&mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }

    fn handle(&self,request: &str) {
        if let Some(v) = &self.next {
            v.handle(request)
        }
    }
}

pub struct Client;

impl<'a> Client{
    pub fn handle<T: Handler<'a>> (handle: &T) {
        handle.handle("do something...")
    }
}
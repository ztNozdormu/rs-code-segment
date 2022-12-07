// !状态是一种行为设计模式，允许对象在其内部状态发生变化时改变其行为。

// !看起来好像对象改变了它的类。


// !我们将实现一个博客发布工作流

// !1. 一篇博客文章开始时是一篇空洞的草稿。

// !2. 草稿完成后，要求对该职位进行审查。

// !3.当帖子被批准后，它就会被发布。

// !4. 只有发布过的博客文章才会返回内容打印，所以未经批准的文章不会被意外发布。


pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 默认实现 返回空指针
    fn content<'a> (&self,_post:&'a Post) -> &'a str{
        " "
    }
}
/**
 * 文章
 */
pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl  Post {
    pub fn new() ->Post {

        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
        
    } 

    pub fn add_text(&mut self,text: &str) {
       self.content.push_str(text)
    }

    pub fn content (& self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
       if let Some(s) = self.state.take() {
          self.state = Some(s)
       }
    }

    pub fn approve(&mut self){

        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
  
  
}

/**
 * 草稿状态
 */
pub struct Draft;

impl State for Draft {

  fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
      self
  }
}


pub struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

pub struct Published;

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
     
    fn content<'a> (&self,post:&'a Post) -> &'a str {
        &post.content
    }
}

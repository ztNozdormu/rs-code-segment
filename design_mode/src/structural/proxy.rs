// !代理是一种结构设计模式，允许您为另一个对象提供替代或占位符。

// !代理控制对原始对象的访问，允许您在请求到达原始对象之前或之后执行某些操作。

// Subject trait声明了RealSubject和

// 代理。只要客户端使用这个与RealSubject一起工作

//接口，你将能够传递给它一个代理而不是一个真正的主题。
pub trait Subject {
    fn request(&self);
}

// RealSubject包含一些核心业务逻辑。通常,RealSubjects
//能够做一些有用的工作，也可能是非常缓慢或
// sensitive -例如纠正输入数据。代理可以解决这些问题
//没有任何更改RealSubject的代码。
pub struct RealSubject {

}

impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: handling request.");
    }
}


pub struct Proxy<'a> {
    real_subject: &'a  RealSubject,
}

impl<'a> Proxy<'a> {
    
   pub fn new<>(real_subject: &'a  RealSubject) -> Proxy {
        Proxy{
            real_subject
        }
    }
     
    fn check_access(&self) -> bool {
         // Some real checks should go here.
         println!("Proxy: checking access prior to firing a real request.");
        true
    }

    fn log_access(&self) {
        println!("Proxy: logging the request.");
    }
}

impl<'a> Subject for Proxy<'a> {

//代理模式最常见的应用程序是惰性加载，

//缓存，控制访问，日志记录等。代理可以执行其中之一

// 然后，根据结果，将执行传递给

//相同的方法在一个链接的RealSubject对象
   fn request(&self) {
       if self.check_access() {
          self.real_subject.request();
          self.log_access();
       }
   }
}


pub struct Client;

impl Client {
    //客户端代码应该与所有对象(两个主题)一起工作

    //和代理)通过Subject接口，以支持真实的

    //主体和代理。然而，在现实生活中，客户大多与之合作

    //他们的真实主题。在本例中，实现模式

    //更容易，你可以扩展你的代理从真正的主题的类。
    pub fn client_code<T: Subject>(subject: &T) {
        subject.request();
    }
}
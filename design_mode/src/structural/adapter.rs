use std::{fmt::format, rc::Rc};


pub trait Target {
    fn get_request(&self) -> String {
        String::from("获取请求数据.......")
    }
}

pub struct DefaultTarget;

impl Target  for DefaultTarget{
    
}

pub struct Adaptee {
    req_str: String,
}

impl Adaptee {
    pub fn new(req_str: String) -> Adaptee {
         Adaptee { req_str }
    }

    pub fn specific_request(&self) -> String {
        format!("adaptee req_str is {}",self.req_str)
    }
}

pub struct Adapter {
    adaptee: Rc<Adaptee>,
}

impl Adapter {
    pub fn new(adaptee: Rc<Adaptee>) -> Adapter {
        Adapter{
            adaptee
        } 
    } 
}

impl Target for Adapter {

    fn get_request(&self) -> String {
        self.adaptee.specific_request()
    }
}

pub struct Client;

impl Client {
  pub fn client_code<T: Target> (target: &T) -> String {
          target.get_request()
    }
    
}

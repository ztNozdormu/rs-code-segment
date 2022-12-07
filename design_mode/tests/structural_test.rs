include!("../src/main.rs");

use std::rc::Rc;
use structural::adapter;
use structural::decorator;
use structural::proxy;

#[cfg(test)]
mod tests {
    use crate::structural::decorator::Decorator;

    
     // 结构型模式
     //  ==============================适配器模式==================================
    #[test]
    fn adapter_test(){
        println!("Client: I can work just fine with the Target objects:");
        super::adapter::Client::client_code(&super::adapter::DefaultTarget {});
        let adaptee = super::Rc::new(super::adapter::Adaptee::new("hello world".to_string()));
        println!("Client: The Adaptee class has a weird interface. See, I don't understand it:");
        println!("Adaptee: {}", adaptee.specific_request());
    
        println!("Client: But I can work with it via the Adapter:");
        let adapter = super::adapter::Adapter::new(adaptee);
        super::adapter::Client::client_code(&adapter);
    
    }
    //  ==============================装饰器模式==================================
    #[test]
    fn decorator_test(){
        let component = super::Rc::new(super::decorator::ConcreteComponent {});
        println!("client: i get a simple component: ");
        super::decorator::Client::client_code(component.as_ref());
        println!("client: now I've got a decorated component:");
        let decorator_a1 = super::decorator::ConcreteDecoratorA::new(component.clone());
        super::decorator::Client::client_code(&decorator_a1);
    
        let decorator_a2 = super::decorator::ConcreteDecoratorB::new(super::Rc::new(decorator_a1));
        super::decorator::Client::client_code(&decorator_a2);
    }  

    //  ==============================代理模式==================================
    #[test]
    fn proxy_test(){
        let real_subject = super::proxy::RealSubject {};
        println!("client: executing the client code with a real subject:");
        super::proxy::Client::client_code(&real_subject);
    
        println!("");
        println!("client: executing the same client code with a proxy:");
        let proxy = super::proxy::Proxy::new(&real_subject);
        super::proxy::Client::client_code(&proxy);
    }

}

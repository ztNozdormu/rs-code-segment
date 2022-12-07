include!("../src/main.rs");

use behavioral::chain_of_responsibility as cResponsibility;
use behavioral::command;
use behavioral::iterator;
use behavioral::observer;
use behavioral::state;
use behavioral::strategy;

#[cfg(test)]
mod tests {
    use crate::behavioral::{chain_of_responsibility::Handler, iterator::Iterator, observer::ISubject,strategy::Duck};

     // 行为模式
     //  ==============================适配器模式==================================
    #[test]
    fn adapter_test(){
        let a1 = super::cResponsibility::AHandler::new("dog".to_string());
        super::cResponsibility::Client::handle(&a1);
    
        println!();
        let mut b = super::cResponsibility::BHandler::new();
        let mut a2 = super::cResponsibility::AHandler::new("cat".to_string());
        b.set_next(&a1);
        // or
        // let h = b.set_next(&a1);
        //a2.set_next(h);
        a2.set_next(&b);
        super::cResponsibility::Client::handle(&a2);
    
    }

    //  ==============================命令模式==================================
    #[test]
    fn command_test(){
        let tv = super::command::TV::new(1);
        let mut remote_control = super::command::TVRemoteControl::new();
        remote_control.press_button(0);
        remote_control.set_command(1, Box::new(super::command::TVOnCommand::new(tv)));
        
        remote_control.set_command(2, Box::new(super::command::TVOffCommand::new(tv)));
    
        remote_control.press_button(1);
        remote_control.press_button(2);
    }
    
    //  ==============================迭代器模式==================================
    #[test]
    fn iterator_test(){
        let mut c = super::iterator::Container::new();
        c.add_item(1);
        c.add_item(2);
        c.add_item(3);
    
        let mut iter = c.iter();
        let has_next = iter.has_next();
        assert_eq!(has_next, true);
        let item = iter.next();
        println!("item: {:?}", item);
        iter.reset();
        while iter.has_next() {
            let v = iter.next().unwrap();
            println!("item: {}", v);
        }
        let item = iter.next();
        assert_eq!(item, None);
    }

    //  ==============================观察者模式==================================
    #[test]
    fn observer_test(){
        let mut subject = super::observer::Subject::new();
        let observer_a = super::observer::ConcreteObserver::new(1);
        let observer_b = super::observer::ConcreteObserver::new(2);
    
        subject.attach(&observer_a);
        subject.attach(&observer_b);
        let message = "subject添加了2个观察者".to_string();
        subject.notify_observers(&message);
        subject.detach(&observer_b);
        let meessage1 = "subject移除了一个观察者对象!".to_string();
        subject.notify_observers(&meessage1);
    }

    //  ==============================观察者模式==================================
    #[test]
    fn state_test() {
        let mut post = super::state::Post::new();

        let text = "State is a behavioral design pattern.";
        post.add_text(text);
        assert_eq!("", post.content());
    
        post.request_review();
        assert_eq!("", post.content());
    
        post.approve();
        assert_eq!(text, post.content());
        println!("post content: {}", post.content());
    }
    //  ==============================策略模式==================================
    #[test]
    fn strategy_test(){
        let mut mallard_duck = super::strategy::MallardDuck::new(Box::new(super::strategy::FlyWithWing));
        mallard_duck.fly();
        mallard_duck.set_fly_behaviour(Box::new(super::strategy::FlyNoWay));
        mallard_duck.fly();
    
        let model_duck = super::strategy::ModelDuck::new(Box::new(super::strategy::FlyNoWay));
        model_duck.fly();
    }
}

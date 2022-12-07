use std::rc::Rc;

pub trait Component{
     fn operation(&self) -> String;
}
pub struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) ->String {
        "ConcrateComponent".to_string()
    }
}


pub trait Decorator: Component {
   fn new(component: Rc<dyn Component>) -> Self;
}

pub struct ConcreteDecoratorA {
   component: Rc<dyn Component>
}

impl Decorator for ConcreteDecoratorA{
    fn new(component: Rc<dyn Component>) -> Self {
        ConcreteDecoratorA{component}
    }
}

impl Component for ConcreteDecoratorA {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

pub struct ConcreteDecoratorB {
    component: Rc<dyn Component>
}

impl Decorator for ConcreteDecoratorB {
    fn new(component: Rc<dyn Component>) -> Self {
        ConcreteDecoratorB{component}
    }
}

impl Component for ConcreteDecoratorB {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

pub struct  Client;

impl Client {
    pub fn client_code<T: Component>(component: &T) {
        print!("result is {}",component.operation()) 
    }
}
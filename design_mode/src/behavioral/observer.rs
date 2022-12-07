// !观察者是一种行为设计模式，允许一个对象通知其他对象其状态的变化。
// 观察者抽象行为 收到通知后执行update业务逻辑......
pub trait IObserver {
    fn update(&self,message: &String);
}
// 目标对象抽象行为
pub trait ISubject<'a,T: IObserver> {

   fn attach(&mut self,observer: &'a T);

   fn detach(&mut self,observer:&'a T);

   fn notify_observers(&self,message: &String);
}

pub struct Subject<'a,T>{
    observers: Vec<&'a T>
}

impl<'a,T: IObserver + PartialEq> Subject<'a,T> {
    pub fn new() ->Subject<'a,T> {
        Subject { observers: Vec::new() }
    }
}

impl<'a,T: IObserver + PartialEq> ISubject<'a,T> for Subject<'a,T> {

    fn attach(&mut self,observer: &'a T) {
        self.observers.push(observer)
    }
    fn detach(&mut self,observer:&'a T) {
      
        if let  Some(idx) = self.observers.iter().position(|ob| *ob == observer){
            self.observers.remove(idx);
        }
        
    }

    fn notify_observers(&self,message: &String) {
        
        for observer in self.observers.iter() {
            observer.update(message);
        }
    }
}


#[derive(PartialEq)]
pub struct ConcreteObserver {
    id: usize
}

impl ConcreteObserver {
    pub fn new(id: usize) ->ConcreteObserver{
        ConcreteObserver{id}
    }
}

impl IObserver for ConcreteObserver {
    fn update(&self,message: &String) {
        println!("Observer id is {},accept subject event message is {}",self.id,message)
    }
}
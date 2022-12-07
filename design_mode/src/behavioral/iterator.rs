// !Iterator是一种行为设计模式，允许您遍历集合的元素，而不暴露其底层表示(列表、堆栈、树等)
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}

pub struct Container<T> {
    data: Vec<T>
} 

impl<T: Clone> Container<T> {


    pub fn new() ->Container<T> {
        Container{data:Vec::new()}
    }
    
    pub fn add_item(&mut self,item: T){
        self.data.push(item)
    }

    pub fn iter(&self) -> impl Iterator<T> +'_{
        ConcreteIterator::new(self)
    }

}

pub struct ConcreteIterator<'a, T>{
    idx: usize,
   container: &'a Container<T>
}

impl<'a,T: Clone> ConcreteIterator<'a,T> {

    pub fn new(container: &'a Container<T>) -> ConcreteIterator<T>{
        ConcreteIterator{idx: 0,container}
    }
}

impl<'a,T: Clone>  Iterator<T>  for ConcreteIterator<'a,T>  {

      fn next(&mut self) -> Option<T> {
       let item = self.container.data.get(self.idx).cloned();
       self.idx += 1;
       item
      }

      fn current(&self) -> Option<T> {
          self.container.data.get(self.idx).cloned()
      }

      fn has_next(&self) -> bool {
         self.container.data.len() > self.idx
      }
      
      fn reset(&mut self) {
          self.idx = 0;
      }
   
}
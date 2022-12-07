pub trait FlyBehaviour {
    fn fly(&self);
}

pub struct FlyWithWing;

pub struct FlyNoWay;

impl FlyBehaviour for FlyWithWing {
    fn fly(&self) {
        println!("I can fly with wing!!!")
    }
}

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("I have now wing;I can not fly!!!")
    }
}

pub trait Duck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour;

    fn fly(&self){
      let fly_behaviour = self.get_fly_behaviour();
      fly_behaviour.fly();
    }
}
pub struct MallardDuck {
   fly_behaviour: Box<dyn FlyBehaviour>
}


impl MallardDuck {
    pub fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> MallardDuck {
        MallardDuck{fly_behaviour}
    }

    pub fn set_fly_behaviour(&mut self,fly_behaviour: Box<dyn FlyBehaviour>){
        self.fly_behaviour = fly_behaviour;
    }
}

impl Duck for MallardDuck {

    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
           &(*self.fly_behaviour)
    }
   
}

pub struct ModelDuck {
    fly_behaviour: Box<FlyNoWay>
}

impl Duck for ModelDuck {
   fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
       &(*self.fly_behaviour)
   }

}

impl ModelDuck {
    pub fn new(fly_behaviour: Box<FlyNoWay>) -> Self {
        ModelDuck { fly_behaviour }
    }
}
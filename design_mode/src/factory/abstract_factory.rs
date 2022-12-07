pub trait GuiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_check_box(&self) -> Box<dyn CheckBox>;
}

pub trait Button {
    fn paint(&self);
}

pub trait CheckBox {
    fn paint(&self);
}


pub struct WinFactory;

pub struct MacFactory;

pub struct WinButton;

pub struct MacButton;

pub struct WinCheckBox;

pub struct MacCheckBox;

impl GuiFactory for WinFactory {

    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton{})
    }
    fn create_check_box(&self) -> Box<dyn CheckBox>{
       Box::new(WinCheckBox{}) 
    }
}

impl GuiFactory for MacFactory {

    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton{})
    }

    fn create_check_box(&self) -> Box<dyn CheckBox> {
        Box::new(MacCheckBox{})
    }

}

impl Button for WinButton {
    fn paint(&self) {
        println!("i am WinButton")
    }
}


impl Button for MacButton {
    fn paint(&self) {
        println!("i am MacButton")
    }
}

impl CheckBox for WinCheckBox {
   fn paint(&self) {
     println!("i am WinCheckBox")
   }
}

impl CheckBox for MacCheckBox {
    fn paint(&self) {
        println!("i am MacCheckBox")
    }
}


pub struct Application;

impl Application {

    pub fn new_gui_factory(os: &str) -> Box<dyn GuiFactory> {
          match os {
              "WIN" => Box::new(WinFactory{}),
              "MAC" => Box::new(MacFactory{}),
              _ => panic!("{}","找不到匹配的系统环境!"),
          }
    }
}
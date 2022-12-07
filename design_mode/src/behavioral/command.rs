use std::collections::HashMap;

// !每个操作都用trait命令封装到一个结构中
pub trait Command {
  fn excute(&self);
} 
#[derive(Clone, Copy)]
pub struct TV {
  name: u32,
}

impl TV {
    pub fn new(name: u32) -> TV {
        TV{
            name
        }
    }

    pub fn on(&self){
        println!("TV name is {},command is {}",self.name,"TV is on, watch movies.".to_string());
    }

    pub fn off(&self){
        println!("TV name is {},command is {}",self.name,"TV is off".to_string());
    }

}

pub struct TVOnCommand{
  tv: TV
}

impl TVOnCommand {
    pub fn new(tv: TV) -> TVOnCommand {
        TVOnCommand{tv}
    }
}

impl Command for TVOnCommand {
    fn excute(&self) {
        self.tv.on()
    }
}

pub struct TVOffCommand {
    tv: TV
}

impl TVOffCommand {
    pub fn new(tv: TV)->TVOffCommand {
        TVOffCommand { tv }
    }
}

impl Command for TVOffCommand {

    fn excute(&self) {
        self.tv.off()
    }

}

pub struct TVRemoteControl {
    commands: HashMap<u32,Box<dyn Command>>
}

impl TVRemoteControl {
    pub fn new() -> TVRemoteControl {
        TVRemoteControl{commands:HashMap::new()}
    }

    pub fn set_command(&mut self,idx: u32,command: Box<dyn Command>) {
            self.commands.insert(idx,command);
    }

    pub fn press_button(&self,idx: u32) {

        if let Some(cmd) = self.commands.get(&idx) {
            cmd.excute(); 
        } else {
            println!("未知命令!!!")
        }
    }
    
}
#[derive(Debug)]
pub struct Object {
   pub name: String,
    id: Option<u32>,
    email: Option<String>
}

impl Object {
   pub fn new(name: String) ->ObjectBuilder {
        ObjectBuilder::new(name)
    }
}

pub struct ObjectBuilder {
    name: String,
    id: Option<u32>,
    email: Option<String>
}

impl ObjectBuilder {
    pub fn new(name: String) -> Self {
        Self{
            name,
            id: None,
            email: None,
        }
    }
    
    pub fn id(mut self,id: u32) -> Self {
        self.id = Some(id);
        self
    }
    pub fn email(mut self,email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn build(self) -> Object {
        // 构造Obejct，注意self的所有权进来，成员都被move给了Object，self所有权结束使用
        Object {
            name: self.name,
            id: self.id,
            email: self.email
        }
    }
}

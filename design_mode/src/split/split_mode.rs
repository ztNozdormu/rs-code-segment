
#[derive(Debug)]
pub struct SplitObject {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SplitObjectMore{
    pub name: String,
    pub id: Option<u32>,
    pub email: Option<String>, 
}

impl SplitObject {
    
    pub fn split(self) -> (String,Vec<u8>) {
        (self.name,self.data)
    }

    pub fn decode(data: Vec<u8>) ->(u32,String) {
         (data.len().try_into().unwrap(),"sss".to_string())
    }
}


use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf, env,fs};
use crate::env::enviroment::{Environment,Environment::Development,Environment::Production,Environment::Staging};
use super::properties::BaseConf;
use crate::common::common::{Result,CONFIG_FILENAME};
use crate::errors::error::ConfigError;

#[derive(Debug)]
pub struct PeomConfg {
    environment: Environment,
    config: HashMap<Environment,BaseConf>
}

impl PeomConfg {
    // 从当前目录开始查找名字为 CONFIG_FILENAME 的配置文件 返回文件信息对象PathBuf
    fn find() -> Result<PathBuf> {

        let cwd = env::current_dir().map_err(|_|ConfigError::NotFound)?;
        let mut current = cwd.as_path();
        loop {
           let mainfest = current.join(CONFIG_FILENAME);
           if fs::metadata(&mainfest).is_ok() {
              return Ok(mainfest);
           }

           match current.parent() {
             Some(p) => current = p,
             None => break,
           }
        }

        Err(ConfigError::NotFound)
    }
    
   // 读取配置文件
   pub fn read_config() -> Result<PeomConfg>{

      // 获取配置文件对象
      let file = PeomConfg::find()?;
      // 获取文件操作句柄
      let mut handle = File::open(&file).map_err(|_|ConfigError::IoError)?;
      // 定义文件内容接受空字符串对象
      let mut content = String::new();
      // 将内容读取到content
      handle.read_to_string(&mut content).map_err(|_|ConfigError::IoError)?;

      // 解析内容为需要的结构体格式
      Self::parse(content,&file)
   }

   fn active_default_from(file_name: Option<&Path>) -> Result<PeomConfg> {
       let mut defaults = HashMap::new();
       
       if let Some(path) = file_name {
            defaults.insert(Development, BaseConf::default_from(Development, path)?);
            defaults.insert(Production, BaseConf::default_from(Production, path)?);
            defaults.insert(Staging, BaseConf::default_from(Staging, path)?);
       }else{
            defaults.insert(Development, BaseConf::default(Development));
            defaults.insert(Production, BaseConf::default(Production));
            defaults.insert(Staging, BaseConf::default(Staging));
       };

       let mut config: PeomConfg = PeomConfg {
           environment: Environment::active()?,
           config: defaults,
       };
    
       Ok(config)
   }

   // 解析配置文件内容
//    fn parse<P> (content: String,file_name: P) -> Result<PeomConfg>  where P: AsRef<Path> {
    fn parse<P: AsRef<Path>>(content: String, file_name: P) -> Result<PeomConfg> {

          let path = file_name.as_ref().to_path_buf();

           // Parse the values from the TOML file.
          let table = match content.parse::<toml::Value>(){
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(content, path, err, Some((1,1))))
            }
            Err(e) => return Err(ConfigError::ParseError(content,path,e.to_string(),e.line_col())),
        };
    
        let mut config = Self::active_default_from(Some(file_name.as_ref()))?;

         // Each environment must be a table.
        for (entry,value) in table {

            let key_pairs = match value.as_table() {
                Some(table) => table,
                None => return Err(ConfigError::BadType(entry,"a table", value.type_str(),Some(path.clone()))) ,
            };
        }
        //dataBase数据匹配  TODO 
        
        Ok(config)

   }

   pub fn get_mut(&mut self,env: Environment) -> & mut BaseConf {

    match self.config.get_mut(&env) {
        Some(config) => config,
        None => panic!("set(): {} config is missing.", env),
    }

   }

   pub fn active() -> Result<BaseConf> {
      Ok(BaseConf::new(Environment::active()?))
   }
}
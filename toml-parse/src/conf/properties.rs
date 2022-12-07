use std::{path::{PathBuf, Path}};
use crate::{env::enviroment::Environment, errors::error::ConfigError};
use crate::common::common::Result;

#[derive(Debug)]
pub struct BaseConf {
    environment: Environment,
    address: String,
    port: u32,
    database: Option<DataBase>,
    workers: Option<u16>,
    config_file_path: Option<PathBuf>,
    root_path: Option<PathBuf>,
}


#[derive(Debug)]
pub struct DataBase {
   pub(crate) adapter: String,
   pub(crate) db_name: String,
   pub(crate) pool: u32,
}

impl BaseConf {
    pub fn new(env: Environment) -> BaseConf {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> BaseConf {
        // // 基础配置公共的基本参数
        // let common_conf = BaseConf {
        //     environment: env,
        //     address: "localhost".to_string(),
        //     port: 3306,
        //     database: None,
        //     workers: Self::get_workers_by_cup_nums(),
        //     config_file_path: None,
        //     root_path: None,
        // };
       
        // BaseConf {
        //     environment: Self::get_environment(env),
        //     ..common_conf
        // }

        BaseConf {
            environment: env, //Self::get_environment(env)
            address: "localhost".to_string(),
            port: 3306,
            database: None,
            workers: Self::get_workers_by_cup_nums(),
            config_file_path: None,
            root_path: None,
        }
        
    }
    // 根据环境参数匹配对应的环境参数
    
    pub(crate) fn get_environment(env: Environment) -> Environment {
        match env {
            Environment::Production => Environment::Production,
            Environment::Development => Environment::Development,
            Environment::Staging => Environment::Staging,
        }
    }

     // 根据硬件cpu核心数计算合理的工作线程
    pub(crate) fn get_workers_by_cup_nums() -> Option<u16> {
        Some((num_cpus::get()*2) as u16)
    }
   
    // 路径设置 TODO

    pub(crate) fn set_root<P: AsRef<Path>> (&mut self,path: P) {
        self.root_path = Some(path.as_ref().into());
    }


    pub(crate) fn default_from<P>(env: Environment,path: P) -> Result<Self> where P: AsRef<Path> {
       
        let mut base_conf = BaseConf::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        
        // 根据配置文件路径获取其父级
        if let Some(parent) = config_file_path.parent() {
            base_conf.set_root(parent);
        }else{
            let msg = "Configuration files must be rooted in a directory.";
            return  Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        base_conf.config_file_path = Some(config_file_path);

        Ok(base_conf)
    }

}

impl PartialEq for BaseConf {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.port == other.port && self.workers == other.workers
    }
}
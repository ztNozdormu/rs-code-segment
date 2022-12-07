use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};


#[derive(Debug)]
pub struct Config {
  pub db_connect_url: String
}

pub fn get_config() -> &'static Mutex<Config> {

    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONE: Once = Once::new();

    ONE.call_once(|| 
        unsafe {
            CONF.as_mut_ptr().write(Mutex::new(Config{
                db_connect_url: "dburl连接地址".to_string()
        }))
    });

    unsafe {&*CONF.as_ptr()}

}
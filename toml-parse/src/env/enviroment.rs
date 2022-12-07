
use std::{env, str::FromStr, fmt};
use Environment::*;
use crate::errors::error::ConfigError;

// 定义常量
pub const CONFIG_ENV: &str = "CONF_ENV";

#[derive(PartialEq,Eq,Hash,Clone, Copy,Debug)]
pub enum Environment {
      /// The development environment. for Debug mode.
      Development,
      /// The staging environment. for Debug mode.
      Staging,
      /// The production environment. for Release mode.
      Production,
}

impl Environment {

    // List of all of the possible environments.
    pub(crate) const ALL: [Environment;3] = [Development,Staging,Production];
    // String of all valid environments.
    pub(crate) const VALID: &'static str = "development,staging,production";

    pub fn active() -> Result<Environment,ConfigError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_|ConfigError::BadEnv(s)),
            // for Debug mode
            #[cfg(debug_assertions)]
            _ => Ok(Development),
            // for Release mode
            #[cfg(not(debug_assertions))]
            _ => Ok(Production),
        }
    }
   // Rust程序的编译优化(opt-level、lto、codegen-units、inline的差异) 参考地址:https://magiclen.org/rust-compile-optimize/
   /**
    * inlineRust内置#[inline]属性，可以手动建议(非强制)编译器去对某个函数进行内联(inline)处理，
    * 将函数的实作展开(拷贝)至调用这个函数的地方。内联的目的在于减少函数调用的次数，
    * 以避免创建堆栈框(Stack Frame)而有额外的开支(overhead)。对于一个体积极小(例如只有一个表达式)、
    * 需要进行快速计算或是为使代码易读而从某个函数中切分出来的函数，我们可以将其加上#[inline]属性，
    * 使它们在编译的时候可以在被调用的位置上展开
    */
    #[inline]
    fn is_dev(self) -> bool {
        self == Development
    }
    #[inline]
    fn is_stage(self) -> bool {
        self == Staging
    }
    #[inline]
    fn is_pro(self) -> bool {
        self == Production
    }
}

impl FromStr for Environment {
    type Err = ();

    /// Parsing a production environment:
    ///
    /// ```rust
    /// let env = "p".parse::<Environment>();
    /// assert_eq!(env.unwrap(), Environment::Production);
    ///
    /// let env = "prod".parse::<Environment>();
    /// assert_eq!(env.unwrap(), Environment::Production);
    /// ```

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let env = match s {
            "d"|"devel"|"development" => Development,
            "s"|"stage"|"Staging" => Staging,
            "p"|"pro"|"Production" => Production,
            _ => return Err(()),
        };
        
        Ok(env)
    }
}

impl fmt::Display for Environment {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Development => write!(f,"development"),
            Staging => write!(f,"staging"),
            Production =>  write!(f,"production"),
        }
    }

}
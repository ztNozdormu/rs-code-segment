pub mod errors;
pub mod env;
pub mod conf;
pub mod common;

use conf::peom_config::PeomConfg;

fn main() {

    let toml_conf = PeomConfg::read_config();

    println!("{:#?}", toml_conf);

}

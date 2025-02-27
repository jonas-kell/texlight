#[macro_use]
extern crate log;

use env_logger;

fn main() -> () {
    env_logger::init();

    info!("Successful started application")
}

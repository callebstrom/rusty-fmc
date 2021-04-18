#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

use crate::domain::crj::Crj700;
use crate::port::primary::cli::Cli;

mod domain;
mod port;

fn main() {
    let opts = Cli::new();

    let airplane = Crj700::new();
    println!("{:?}", opts);
    println!("{:?}", airplane);
}

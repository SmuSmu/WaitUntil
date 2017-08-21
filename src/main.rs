// Disable Warning because of CamelCase Name :-)
#![allow(non_snake_case)]


// Extern Libary for command line arguments
// Docs https://docs.rs/clap/2.26.0/clap/
extern crate clap;
use clap::{Arg, App, SubCommand};
 
fn main() {
    let matches = App::new("WaitUntil")
                          .version("1.0")
                          .author("SmuSmu")
                          .about("a commandline tool to wait for special events")
                          .arg(Arg::with_name("fileexists")
                               .long("fileexists")
                               .value_name("FILE")
                               .help("Wait until a given File exists")
                               .required(true)
                               .takes_value(true))
                          .get_matches();
 
    // Same as previous example...
}

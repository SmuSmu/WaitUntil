// Disable Warning because of CamelCase Name :-)
#![allow(non_snake_case)]


// Extern Libary for command line arguments
// Docs https://docs.rs/clap/2.26.0/clap/
extern crate clap;
use clap::{Arg, App, SubCommand};
 
fn main() {
    let matches = App::new("myapp")
                          .version("1.0")
                          .author("SmuSmu")
                          .about("Does awesome things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();
 
    // Same as previous example...
}

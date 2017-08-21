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
                               .short("f")
                               .help("Wait until a given File exists")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("timeout")
                               .long("timeout")
                               .value_name("SECONDS")
                               .short("t")
                               .help("Set the Timeout Value Default is 30 Seconds")
                               .required(false)
                               .takes_value(true))
                          .get_matches();
 
    // Start
    if matches.is_present("timeout") {
        if let Some(varTimeOut) = matches.value_of("timeout") {
            println!("TimeOut setted to : {} seconds", varTimeOut);
            }
        }
    else {
        let varTimeOut = "30"; 
        println!("Using Default Timeout of : {} seconds", varTimeOut);
        }


    if matches.is_present("fileexists") {
        if let Some(varFileName) = matches.value_of("fileexists") {
            println!("Now Waiting for file : {}", varFileName);
            }
        }
    else {
        println!("Please run me with --help to see Options");
        }
}

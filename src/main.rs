// Disable Warning because of CamelCase Name :-)
#![allow(non_snake_case)]


// Extern Libary for command line arguments
// Docs https://docs.rs/clap/2.26.0/clap/
extern crate clap;
extern crate time;
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
    //let ten_millis = time::Duration::from_millis(10);

    println!("Current Time : {} seconds", time::precise_time_s());
    println!("Current Time : {} seconds", time::precise_time_s() + 30.0 as f64);

    // default timeout is 30 sec
    // Using f64 because precise_time_s() returns f64. So better for calculation later.
    let mut varTimeOut = 30.0 as f64 ;

    if matches.is_present("timeout") {
        // get timeout value. First as String(?)
        if let Some(varArgTimeOut) = matches.value_of("timeout") {
            // convert TimeOut to f64
            varTimeOut = varArgTimeOut.parse::<f64>().unwrap();
            }
        }
    
    let varTimeOutTraget = time::precise_time_s() + varTimeOut;

    println!("Timeout is setted to : {} seconds", varTimeOut);

    if matches.is_present("fileexists") {
        if let Some(varFileName) = matches.value_of("fileexists") {
            println!("Now Waiting for file : {}", varFileName);
            while time::precise_time_s() < varTimeOutTraget {
                println!("Current Time : {} seconds", time::precise_time_s());
                //thread::sleep(ten_millis);
                }
            }
        }
    else {
        println!("Please run me with --help to see Options");
        }
}

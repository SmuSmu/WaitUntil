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

    // default timeout is 30 sec
    let mut varTimeOut = 30 as i64 ;

    if matches.is_present("timeout") {
        // get timeout value. First as String(?)
        if let Some(varArgTimeOut) = matches.value_of("timeout") {
            // convert TimeOut to i64
            varTimeOut = varArgTimeOut.parse::<i64>().unwrap();
            }
        }

    println!("Timeout is setted to : {} seconds", varTimeOut);

    if matches.is_present("fileexists") {
        if let Some(varFileName) = matches.value_of("fileexists") {
            let ten_millis = std::time::Duration::from_millis(10);
            println!("Now Waiting for file : {}", varFileName);

            // _ is a disposable variable to avoid debug message "warning: unused variable:"
            // varTimeOut is multiplied by 100 because of sleep(ten_millis)
            for _ in 0..varTimeOut*100 {
                if std::path::Path::new(varFileName).exists() {
                    println!("Found file");
                    std::process::exit(0);
                    }
                
                // Be nice to the cpu :-)
                std::thread::sleep(ten_millis);
                }

            println!("Timeout exeeded");
            std::process::exit(1);
            }
        }
    else {
        println!("Please run me with --help to see Options");
        }
}

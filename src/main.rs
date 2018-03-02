extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate rand;

use std::env;
use std::error::Error;
use std::process;
//use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::Rng;
//use log::Level;
use getopts::Options;

mod encode;
use encode::to_phi;
mod encode_064;
mod encode_128;
mod encode_192;
mod encode_256;

/* Macros */
/*
macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}
*/

/* Functions */
fn encode(inp: &str, out: &str) {
    println!("{:?}", out);
    let spath = Path::new(inp);
    let s = match File::open(spath) {
        Err(why) => panic!("couldn't open {}: {}", inp, why.description()),
        Ok(file) => file,
    };
    let mut rng = rand::thread_rng();
    if rng.gen() {
        // Read data and encode it
        for byte in s.bytes() {
            let b = byte.unwrap();
            trace!("Byte : {}", b);
            let e = to_phi(b, rng.gen::<i32>());
            println!("{:?}", e);
        }
    }
}

fn decode(inp: &str, out: &str) {
    println!("{:?} {:?}", inp, out);
    //let spath = Path::new(inp.to_slice());
    //let mut s = File::open(spath)?;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -i SOURCE -o TARGET [options]", program);
    print!("{}", opts.usage(&brief));
}

/*********************/
/***      MAIN     ***/
/*********************/
// macros: error!, warn!, info!, debug! and trace! where error! represents the highest-priority log level, and trace! the lowest.
fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "encode", "encode source to target");
    opts.optflag("d", "decode", "decode source to target");
    opts.optopt("i", "", "set input file name, or - for stdin", "SOURCE");
    opts.optopt("o", "", "set output file name, or - for stdout", "TARGET");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }
    // Must encode or decode
    if !matches.opt_present("e") && !matches.opt_present("d") {
        error!("Missing mandatory option -e or -d");
        process::exit(1);
    }
    // Input is a file or stdin
    if !matches.opt_present("i") {
        error!("Missing mandatory option -i");
        process::exit(1);
    }
    // Ouptut is a file or stdout
    if !matches.opt_present("o") {
        error!("Missing mandatory option -o");
        process::exit(1);
    }

    let input = matches.opt_strs("i");
    let output = matches.opt_strs("o");

    if matches.opt_present("e") {
        encode(&input[0], &output[0]);
    } else if matches.opt_present("d") {
        decode(&input[0], &output[0]);
    }
}

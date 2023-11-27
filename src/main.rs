// import the modules
mod cache;
mod verify;
mod cache_commands;
mod config;

// clap is a command line argument parser
// https://docs.rs/clap/2.33.0/clap/
use clap::{App, Arg, SubCommand};

// get cli args:
/*
    Total cache size
    Block size
    Unified vs. split I- and D-caches
    Associativity
    Write back vs. write through
    Write allocate vs. write no allocate
*/
fn get_args() -> (u32, u32, bool, u32, bool, bool) {
    let matches = App::new("Cache Simulator")
        .version("1.0")
        .author("Jason Ivey")
        .about("Simulates a cache")
        .arg(
            Arg::with_name("total_cache_size")
                .short("t")
                .long("total_cache_size")
                .value_name("TOTAL_CACHE_SIZE")
                .help("Sets the total cache size")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("block_size")
                .short("b")
                .long("block_size")
                .value_name("BLOCK_SIZE")
                .help("Sets the block size")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("associativity")
                .short("a")
                .long("associativity")
                .value_name("ASSOCIATIVITY")
                .help("Sets the associativity")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("write_back")
                .short("w")
                .long("write_back")
                .value_name("WRITE_BACK")
                .help("Sets the write back")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("write_allocate")
                .short("l")
                .long("write_allocate")
                .value_name("WRITE_ALLOCATE")
                .help("Sets the write allocate")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("split_cache")
                .short("s")
                .long("split_cache")
                .value_name("SPLIT_CACHE")
                .help("Sets the split cache")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    

fn main() {
    println!("Hello, world!");
}

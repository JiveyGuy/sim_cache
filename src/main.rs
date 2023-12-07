#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// ^ remove before submission TODO

// import the external crates
use log::{debug, error, info, trace, warn};
use env_logger;

// import the pub modules (local files)
pub mod cache;
pub mod verify;
pub mod cache_commands;
pub mod config;
pub mod get_cli_args;


// main function   
fn main() 
{
    // configure the logger from "use log::{}"
    env_logger::init();


    // get the command line args
    let args = get_cli_args::get_values();

    // get vars
    let block_size              = args.0;
    let unified_cache_size      = args.1;
    let instruction_cache_size  = args.2;
    let data_cache_size         = args.3;
    let associativity           = args.4;
    let write_back              = args.5;
    let write_allocate          = args.6;
    let split_cache             = args.7;

    // debug print
    debug!("block_size: {}
            unified_cache_size: {}
            instruction_cache_size: {}
            data_cache_size: {}
            associativity: {}
            write_back: {}
            write_allocate: {}
            split_cache: {}",
            block_size,
            unified_cache_size,
            instruction_cache_size,
            data_cache_size,
            associativity,
            write_back,
            write_allocate,
            split_cache);


}




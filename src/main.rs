#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// ^ remove before submission TODO

// import the pub modules
pub mod cache;
pub mod verify;
pub mod cache_commands;
pub mod config;
pub mod get_cli_args;


// main function   
fn main() 
{
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

    


}




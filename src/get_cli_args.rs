use crate::config;

// clap is a command line argument parser
// https://docs.rs/clap/2.33.0/clap/
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Cli 
{
    /*
    cli args:

        Total cache size
        Block size
        Unified vs. split I- and D-caches
        Associativity
        Write back vs. write through
        Write allocate vs. write no allocate

        -h:         this message
        -bs <bs>:   set cache block size to <bs>
        -us <us>:   set unified cache size to <us>
        -is <is>:   set instruction cache size to <is>
        -ds <ds>:   set data cache size to <ds>
        -a <a>:     set cache associativity to <a>
        -wb:        set write policy to write back
        -wt:        set write policy to write through
        -wa:        set allocation policy to write allocate
        -nw:        set allocation policy to no write allocate

        USE DEFAULTS IF NO ARGS ARE GIVEN
    */

    /// Sets the cache block size to <bs>
    #[arg(long)]
    bs: Option<u32>,

    /// Sets the unified cache size to <us>
    #[arg(long)]
    us: Option<u32>,

    /// Sets the instruction cache size to <is>
    #[arg(long)]
    is: Option<u32>,

    /// Sets the data cache size to <ds>
    #[arg(long)]
    ds: Option<u32>,

    /// Sets the cache associativity to <a>
    #[arg(long)]
    a: Option<u32>,

    /// Sets the write policy to write back
    #[arg(long)]
    wb: Option<String>,

    /// Sets the write policy to write through
    #[arg(long)]
    wt: Option<String>,

    /// Sets the allocation policy to write allocate
    #[arg(long)]
    wa: Option<String>,

    /// Sets the allocation policy to no write allocate
    #[arg(long)]
    nw: Option<String>,
}

pub fn get_values() -> (u32, u32, u32, u32, u32, bool, bool, bool)
{
    // get the command line args
    let args = Cli::parse();

    // get vars
    let block_size          = match args.bs 
    {
        Some(ref bs) => bs,
        None     => &config::DEFAULT_BLOCK_SIZE,
    };
    
    let unified_cache_size  = match args.us 
    {
        Some(ref us) => us,
        None     => &config::DEFAULT_TOTAL_CACHE_SIZE,
    };

    let instruction_cache_size  = match args.is 
    {
        Some(ref is) => is,
        None     => &config::DEFAULT_TOTAL_CACHE_SIZE,
    };

    let data_cache_size  = match args.ds 
    {
        Some(ref ds) => ds,
        None     => &config::DEFAULT_TOTAL_CACHE_SIZE,
    };

    let associativity       = match args.a 
    {
        Some(ref a) => a,
        None    => &config::DEFAULT_ASSOCIATIVITY,
    };

    let write_back          = match args.wb 
    {
        Some(ref wb) => match wb.as_str()
        {
            "true"  => true,
            "false" => false,
            _       => config::DEFAULT_WRITE_BACK,
        },

        None     => config::DEFAULT_WRITE_BACK,
    };

    let write_allocate      = match args.wa 
    {
        Some(ref wa) => match wa.as_str()
        {
            "true"  => true,
            "false" => false,
            _       => config::DEFAULT_WRITE_ALLOCATE,
        },

        None     => config::DEFAULT_WRITE_ALLOCATE,
    };

    let split_cache         = match args.nw 
    {
        Some(ref nw) => match nw.as_str()
        {
            "true"  => true,
            "false" => false,
            _       => config::DEFAULT_SPLIT_CACHE,
        },

        None     => config::DEFAULT_SPLIT_CACHE,
    };

    // we need to verify that only one of the write policies is set
    // and only one of the allocation policies is set
    // if not, use the default values
    if args.wb.is_some() && args.wt.is_some()
    {
        println!("Error: cannot set both write back and write through policies. Using default write back policy.");
        return (*block_size, *unified_cache_size, *instruction_cache_size, *data_cache_size, *associativity, write_back, write_allocate, split_cache);
    }

    if args.wa.is_some() && args.nw.is_some()
    {
        println!("Error: cannot set both write allocate and no write allocate policies. Using default write allocate policy.");
        return (*block_size, *unified_cache_size, *instruction_cache_size, *data_cache_size, *associativity, write_back, write_allocate, split_cache);
    }

    // make sure to dereference the values
    return (*block_size, *unified_cache_size, *instruction_cache_size, *data_cache_size, *associativity, write_back, write_allocate, split_cache);
} 
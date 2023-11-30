// this is the main cache where
// all the magic happens
// (ie: configuration handling and logic)

// stuff for LRU list
use std::collections::LinkedList;
use std::collections::HashMap;

use clap::builder::NonEmptyStringValueParser;

// local includes
use crate::config;
use crate::cache_commands;

struct CacheLine
{
    tag         : u32,
    dirty       : bool,
    lru         : LinkedList<u32>,
}

pub struct Cache
{
    // cache configuration
    size                : u32, // cache size
    associativity       : u32, // cache associativity
    n_sets              : u32, // number of sets
    sets                : Vec<u32>, // number of valid entries in each set
    contents            : u32,      // number of valid entries in the cache
    lines               : Vec<CacheLine>, // cache lines
}
       
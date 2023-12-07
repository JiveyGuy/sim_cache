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

#[derive(Clone, Copy, Debug)]
struct LRU
{
    /*

    LRU list:
        - list of cache lines in the set
        - each entry is a cache line index
        - the first entry is the LRU line
        - the last entry is the MRU line
    */

    // LRU listpub fn new(associativity: u32) -> LRU
    {
        // create the LRU
        let mut lru = LRU
        {
            list    : Vec::new(),
            map     : HashMap::new(),
        };

        // initialize the LRU list to be empty
        lru.list = vec![0; associativity as usize];

        // initialize the LRU map to be empty
        for i in 0..associativity
        {
            lru.map.insert(i, i as usize);
        }

        // return the LRU
        return lru;
    }
    // LRU map
    map     : HashMap<u32, usize>,
}

impl LRU 
{
    // create a new LRU
    pub fn new(associativity: u32) -> LRU
    {
        // create the LRU
        let mut lru = LRU
        {
            list    : Vec::new(),
            map     : HashMap::new(),
        };

        // set the LRU list
        lru.list = vec![0; associativity as usize];

        // set the LRU map
        for i in 0..associativity
        {
            lru.map.insert(i, i as usize);
        }

        // return the LRU
        return lru;
    }

    // update the LRU
    pub fn update(&mut self, index: u32)
    {
        // get the index of the line in the LRU list
        let line_index = self.map[&index];

        // remove the line from the LRU list
        self.list.remove(line_index);

        // add the line to the end of the LRU list
        self.list.push(index);

        // update the LRU map
        for i in 0..self.list.len()
        {
            self.map.insert(self.list[i], i);
        }
    }

    // get the LRU line
    pub fn get_lru(&self) -> u32
    {
        // return the LRU line
        return self.list[0];
    }

    // get the MRU line
    pub fn get_mru(&self) -> u32
    {
        // return the MRU line
        return self.list[self.list.len() - 1];
    }

    // print the LRU
    pub fn print(&self)
    {
        // print the LRU list
        println!("LRU list: {:?}", self.list);
    }

    // print the LRU map
    pub fn print_map(&self)
    {
        // print the LRU map
        println!("LRU map: {:?}", self.map);
    }   
}

#[derive(Clone, Copy, Debug)]
struct CacheLine
{
    tag         : u32,
    dirty       : bool,
    lru         : LRU,
}

#[derive(Clone, Copy, Debug)]
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

impl Cache
{
    // create a new cache
    pub fn new(
        block_size          : u32,
        unified_cache_size  : u32,
        instruction_cache_size  : u32,
        data_cache_size     : u32,
        associativity       : u32,
        write_back          : bool,
        write_allocate      : bool,
        split_cache         : bool,
    ) -> Cache
    {
        // create the cache
        let mut cache = Cache
        {
            size                : 0,
            associativity       : 0,
            n_sets              : 0,
            sets                : Vec::new(),
            contents            : 0,
            lines               : Vec::new(),
        };

        // set the cache size
        if split_cache
        {
            cache.size = instruction_cache_size + data_cache_size;
        }
        else
        {
            cache.size = unified_cache_size;
        }

        // set the cache associativity
        cache.associativity = associativity;

        // set the number of sets
        cache.n_sets = cache.size / (block_size * associativity);

        // set the number of valid entries in each set
        cache.sets = vec![0; cache.n_sets as usize];

        // set the number of valid entries in the cache
        cache.contents = 0;

        // set the cache lines
        cache.lines = vec![CacheLine{tag: 0, dirty: false, lru: LinkedList::new()}; cache.n_sets as usize];

        // return the cache
        return cache;
    }

    // print cach settings with this format:
    pub fn print_settings(&self)
    {
        println!("*** CACHE SETTINGS ***");
        println!("Unified I- D-cache");
        println!("Size: \t{}", self.size);
        println!("Associativity: \t{}", self.associativity);
        println!("Block size: \t{}", config::DEFAULT_BLOCK_SIZE);
        println!("Write policy: \t{}", if config::DEFAULT_WRITE_BACK {"WRITE BACK"} else {"WRITE THROUGH"});
        println!("Allocation policy: \t{}", if config::DEFAULT_WRITE_ALLOCATE {"WRITE ALLOCATE"} else {"WRITE NO ALLOCATE"});
    }
}
       
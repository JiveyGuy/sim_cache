#[derive(Clone, Copy, Debug)]
pub struct cache_stat 
{
    pub accesses        :u32,
    pub misses          :u32,
    pub replacements    :u32,
    pub demand_fetches  :u32,
    pub copies_back     :u32,
}

impl cache_stat 
{
    
    // get miss rate
    pub fn get_miss_rate(&self) -> f32 
    {
        return self.misses as f32 / self.accesses as f32;
    }
    
    
    // std output
    pub fn print(&self) 
    {
        println!("*** CACHE STATISTICS ***");
        println!("INSTRUCTIONS");
        println!(" accesses:  {}", self.accesses);
        println!(" misses:    {}", self.misses);
        println!(" miss rate: {:.4} (hit rate {:.4})", self.get_miss_rate(), 1.0 - self.get_miss_rate());
        println!(" replace:   {}", self.replacements);
        println!("DATA");
        println!(" accesses:  {}", self.accesses);
        println!(" misses:    {}", self.misses);
        println!(" miss rate: {:.4} (hit rate {:.4})", self.get_miss_rate(), 1.0 - self.get_miss_rate());
        println!(" replace:   {}", self.replacements);
        println!("TRAFFIC (in words)");
        println!(" demand fetch:  {}", self.demand_fetches);
        println!(" copies back:   {}", self.copies_back);
    }
    
}
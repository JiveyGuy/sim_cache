pub struct cache_stat {
    pub accesses        :u32,
    pub misses          :u32,
    pub replacements    :u32,
    pub demand_fetches  :u32,
    pub copies_back     :u32,
}
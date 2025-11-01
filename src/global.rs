use ahash::{AHasher, AHashMap};
use lazy_static::lazy_static;
use std::sync::{Mutex};

/*
lazy_static! {
    pub static ref Perf: Mutex<AHashMap<&'static str, u64>> = Mutex::new(AHashMap::new());
}
*/
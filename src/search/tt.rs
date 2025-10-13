use crate::movegen::Move;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Table {
    entries: Vec<Entry>,
    pub usage: usize,
    pub capacity: usize,
}

#[derive(Clone)]
pub struct Entry {
    pub key: u64,
    pub depth: u8,
    pub eval: i32,
    pub bound: Bound,
    pub mv: Option<Move>,
}

#[derive(Clone)]
pub enum Bound {
    Exact,
    Lower,
    Upper,
}

impl Table {
    pub fn with_mb(size_mb: usize) -> Self {
        let size_bytes = size_mb.saturating_mul(1024 * 1024);
        let capacity = size_bytes / std::mem::size_of::<Entry>();

        // We use the nearest lower power of two for capacity so that indexing can
        // use fast bitwise AND (key & (pow2 - 1)) instead of modulo, as in index()
        let pow2 = capacity.next_power_of_two() / 2;

        let entries = vec![
            Entry {
                key: 0,
                depth: 0,
                eval: 0,
                bound: Bound::Exact,
                mv: None,
            };
            pow2
        ];

        Self {
            entries,
            usage: 0,
            capacity: pow2,
        }
    }

    pub fn probe(&self, key: u64) -> Option<&Entry> {
        let entry = &self.entries[self.index(key)];
        if entry.key == key { Some(entry) } else { None }
    }

    pub fn store(&mut self, key: u64, depth: u8, eval: i32, bound: Bound, mv: Option<Move>) {
        let index = self.index(key);
        let entry = &mut self.entries[index];

        if depth >= entry.depth {
            if entry.key == 0 {
                self.usage += 1;
            }
            entry.key = key;
            entry.depth = depth;
            entry.eval = eval;
            entry.bound = bound;
            entry.mv = mv;
        }
    }

    #[inline]
    fn index(&self, key: u64) -> usize {
        key as usize & (self.capacity - 1)
    }
}

pub const MIN_SIZE_MB: usize = 1;
pub const MAX_SIZE_MB: usize = 4096;
pub const DEFAULT_SIZE_MB: usize = 64;

pub fn set_size_mb(size_mb: usize) {
    if !(MIN_SIZE_MB..=MAX_SIZE_MB).contains(&size_mb) {
        panic!("invalid transposition table size: {size_mb}mb");
    }
    SIZE_MB.store(size_mb, Ordering::Relaxed);
}

pub fn size_mb() -> usize {
    SIZE_MB.load(Ordering::Relaxed)
}

static SIZE_MB: AtomicUsize = AtomicUsize::new(DEFAULT_SIZE_MB);

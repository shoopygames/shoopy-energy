use std::cell::RefCell;
// Shared mutable state using RefCell
thread_local! {
    pub static DIFFICULTY: RefCell<usize> = RefCell::new(0);
    pub static CPU_THREADS: RefCell<usize> = RefCell::new(0);
    pub static TOTAL_HPS: RefCell<f64> = RefCell::new(0.0);
    pub static ACCEPTED_SHARES: RefCell<usize> = RefCell::new(0);
    pub static TOTAL_SHARES: RefCell<u64> = RefCell::new(0);
}
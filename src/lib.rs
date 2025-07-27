mod convert;
mod hashmap;
mod median;
use rand::Rng;
use std::i32::{MAX, MIN};
pub fn start() {
    let nums = vec![13, 14, 14, 16, 1, 3];
    for i in 65..=85 {
        let mut vec: Vec<i32> = Vec::new();
        let range: u16 = i;
        println!("Range {}:",range);
        for _ in 0..range {
            vec.push(rand::rng().random_range(MIN..=MAX));
        }
        median::md(&vec);
        println!("");
    }
    let md: median::Md = median::md(&nums);
    if let (Some(median), Some(mode)) = (md.median, md.mode) {
        println!("Median: {}\nMode: {}", median, mode)
    }
}

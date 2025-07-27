use std::{cmp::Ordering};
//use std::time;
pub struct Md {
    pub median: Option<u32>,
    pub mode: Option<i32>,
}
impl Md {
    fn init() -> Md {
        Md {
            median: Some(0),
            mode: Some(0),
        }
    }
    fn none() -> Md {
        Md {
            median: None,
            mode: None,
        }
    }
}
pub fn md(numbers: &Vec<i32>) -> Md {
    match numbers.len() {
        // handles cases, in that the following code would fail.
        0 => return Md::none(),
        1 => return Md::init(),
        _ => (),
    };
    // for the median part
    let mut res: Md = Md::init();
    let mut sorted: Vec<i32> = numbers.clone();
    sorted.sort();
    let length: usize = sorted.len();
    res.median = match length % 2 {
        1 => Some(((length - 1) / 2) as u32),
        0 => Some((length / 2) as u32),
        _ => None,
    };
    // for the mode part (the most common number)
    if length > 75 {
        res.mode = Some(hashmap(numbers));
    } else {
        res.mode = Some(mode_vec(numbers));
    }
    res
}
//    for _ in 0..5 {
//        let mut hashmap_time = Millis::new();
//        hashmap(numbers);
//        hashmap_time.duration = Some(hashmap_time.start.elapsed());
//        let mut mode_vec_time = Millis::new();
//        mode_vec(numbers);
//        mode_vec_time.duration = Some(mode_vec_time.start.elapsed());
//        println!(
//            "HashMap time: '{:?}' Vector time: '{:?}'",
//            hashmap_time.duration, mode_vec_time.duration
//        );
//    }
//}
//struct Millis {
//    start: time::Instant,
//    duration: Option<time::Duration>,
//}
//impl Millis {
//    fn new() -> Millis {
//        Millis {
//            start: time::Instant::now(),
//            duration: None,
//        }
//    }
//}
fn hashmap(numbers: &Vec<i32>) -> i32 {
    // HashMap implementation
    use std::collections::HashMap;
    let mut most_common_nums: HashMap<i32, u32> = HashMap::new();
    for i in numbers {
        let insert = most_common_nums.entry(*i).or_insert(0);
        *insert += 1;
    }
    let mut highest_num: i32 = 0;
    let mut highest_key: u32 = 0;
    for (num, val) in most_common_nums {
        if val > highest_key {
            highest_key = val;
            highest_num = num;
        }
    }
    highest_num
}
// For mode_vec specifically
struct VecCommon {
    num: i32,
    count: u32,
}
impl VecCommon {
    fn new(int: &i32) -> VecCommon {
        VecCommon {
            num: *int,
            count: 1,
        }
    }
}
fn mode_vec(numbers: &Vec<i32>) -> i32 {
    // With Vectors instead of HashMap
    let mut v: Vec<VecCommon> = Vec::new();
    let mut found: bool;
    for i in numbers {
        found = false;
        for n in &mut v {
            if n.num == *i {
                n.count += 1;
                found = true
            }
        }
        if !found {
            v.push(VecCommon::new(i))
        }
    }
    let mut max: u32 = 0;
    let mut index: usize = 0;
    for i in 1..v.len() {
        match v[i].count.cmp(&max) {
            Ordering::Greater => {
                index = i;
                max = v[i].count;
            }
            _ => (),
        }
    }
    v[index].num
}

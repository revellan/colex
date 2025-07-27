use colex::median;
fn main() {
    _test_convert();
}
fn _test_convert() {
    let mut string = String::from("apple");
    colex::convert::convert(&mut string);
    println!("{}", string);
}
fn _test_median() {
    let nums = vec![13, 14, 14, 16, 1, 3];
    let md: median::Md = median::md(&nums);
    if let (Some(median), Some(mode)) = (md.median, md.mode) {
        println!("Median: {}\nMode: {}", median, mode)
    }
}

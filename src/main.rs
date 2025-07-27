use colex::median;
fn main() {
    _test_convert();
}
fn _test_convert() {
    let mut string = String::new();
    print!("Enter String: ");
    use std::{io, io::Write};
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin().read_line(&mut string).expect("Failed to read line");
    string.pop();
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

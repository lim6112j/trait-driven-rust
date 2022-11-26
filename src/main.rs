use std::{time:: Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
// struct Progress<Iter> {
//     iter: Iter,
//     i : usize,
// }
// impl<Iter> Progress<Iter> {
//     pub fn new (iter: Iter) -> Self {
//         Progress {iter, i: 0}
//     }
// }
fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
    where Iter : Iterator {
    let mut i: usize = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
fn main() {
    let v = vec![1,2,3];
    progress(v.iter(), expensive_calculation);
    println!("Hello, world!");
}

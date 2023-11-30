mod algorithms;
use algorithms::insertion;
use std::time::Instant;
use rand::prelude::*;
fn main() {
    //Create random list
    let n: u64 = 1000000;

    let mut rng = rand::thread_rng();

    let list: Vec<f32> = (0..n).map(|_| rng.gen_range(0.0..100.0)).collect();

    let start_time = Instant::now();
    let result = insertion(list);
    let elapsed_time = start_time.elapsed();

    println!();
    println!("result =>{:?}", result);
    println!("required time => {}", elapsed_time.as_millis());
}

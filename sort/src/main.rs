mod algorithms;
use algorithms::insertion;
use algorithms::merge_sort;
use rand::prelude::*;
use std::time::Instant;

fn main() {
    //Create random list
    let n: u64 = 100000;

    let mut rng = rand::thread_rng();

    let list: Vec<f32> = (0..n).map(|_| rng.gen_range(0.0..100.0)).collect();

    let start_time = Instant::now();
    let result = merge_sort(&list);
    let elapsed_time = start_time.elapsed();

    println!();
    println!("Merge sort =>{:?}", &result[..10]);
    println!("required time Merge sort => {}", elapsed_time.as_millis());

    let start_time: Instant = Instant::now();
    let result: Vec<f32> = insertion(list);
    let elapsed_time = start_time.elapsed();
    println!();
    println!("Insertion sort =>{:?}", &result[..10]);
    println!("required time Insertion sort => {}", elapsed_time.as_millis());
}

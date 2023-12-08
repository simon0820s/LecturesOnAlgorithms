mod algorithms;
use algorithms::insertion;
use algorithms::merge_sort;
use rand::prelude::*;
use std::time::Duration;
use std::time::Instant;

struct Result {
    result: Vec<f32>,
    elapsed_time: Duration,
}

fn main() {
    //Create random list
    let n: u64 = 10000;

    let mut rng = rand::thread_rng();

    let list: Vec<f32> = (0..n).map(|_| rng.gen_range(0.0..100.0)).collect();

    let start_time = Instant::now();
    let result = merge_sort(&list);
    let elapsed_time = start_time.elapsed();

    let insertion_result = Result {
        result: result,
        elapsed_time: elapsed_time,
    };

    println!();
    println!("Merge sort =>{:?}", &insertion_result.result[..10]);
    println!(
        "required time Merge sort => {}",
        insertion_result.elapsed_time.as_millis()
    );

    let start_time: Instant = Instant::now();
    let result: Vec<f32> = insertion(list);
    let elapsed_time = start_time.elapsed();

    let merge_result: Result = Result {
        result: result,
        elapsed_time: elapsed_time,
    };

    println!();
    println!("Insertion sort =>{:?}", &merge_result.result[..10]);
    println!(
        "required time Insertion sort => {}",
        merge_result.elapsed_time.as_millis()
    );
}

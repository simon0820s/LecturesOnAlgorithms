mod algorithms;
use algorithms::binary_search;
use algorithms::N;

use rand::prelude::*;
use std::time::Instant;
fn main() {

    let mut rng = rand::thread_rng();

    let mut arr: [u32; N] = [0; N];

    let range: u32 = match (N*100).try_into() {
        Ok(value) => value,
        Err(err) => {
            println!("Error at try parse: {}", err);
            return;
        }
    };

    for i in 0..arr.len() {
        loop {
            let num = rng.gen_range(0.. range);

            if !arr.contains(&num) {
                arr[i] = num;
                break;
            }
        }
    }

    arr.sort();

    let item = arr.get(rng.gen_range(0..arr.len())).unwrap();

    let start_time = Instant::now();

    let result = binary_search(&arr, &item, &0, &arr.len());

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);
    println!("result => {} item => {}", arr[result], item);
    println!("time => {:?}", elapsed_time.as_millis());
}

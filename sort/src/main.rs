mod algorithms;
use algorithms::insertion;
fn main() {
    let list: Vec<f32> = vec![0., 20., 3., 4.];

    let result = insertion(list);

    println!("{:?}", result)
}

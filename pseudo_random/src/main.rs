fn main() {
    const N: u64 = 100;

    let pseudo_random_list = generate_n_pseudo_random(N, 14.5, 4.3, 3.8);
    println!("{:?}", pseudo_random_list);
}

fn generate_n_pseudo_random (n: u64, mile: f32, multiplicative: f32, module: f32) -> Vec<f32> {
    let mut aleatory_list: Vec<f32> = vec![];

    for i in 1..n {
        aleatory_list.push(generate_pseudo_random(mile, multiplicative, i, module))
    }

    aleatory_list
}

fn generate_pseudo_random (mile: f32, multiplicative: f32, incremental: u64, module: f32) -> f32 {
    (multiplicative * mile + incremental as f32) % module
}

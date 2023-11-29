mod get_input;
use get_input::interact::input_to_f64;

fn main() {
    println!("🦀===Welcome to the float-binary conversor===🦀");
    let float = input_to_f64();
    let binary = number_to_binary(float);
    println!("🦀Your result is: {}", binary);
}

fn number_to_binary(float: f64) -> u64 {
    const UMBRAL: f64 = 1e-4;
    let fraction = float.abs() - float.abs().trunc();

    let binary_result_s: String;

    if fraction < UMBRAL {
        let integer = float.trunc() as i64;
        binary_result_s = int_to_binary(integer);
    }
    else {
        let powered_float = float*256.0;
        let integer: i64 = powered_float.trunc() as i64;
        binary_result_s = int_to_binary(integer)
    }
    
    let binary_result: u64 = match binary_result_s.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error at try convert");
            return 0;
        }
    };

    binary_result

}

fn int_to_binary(mut integer: i64) -> String {

    let mut binary_s = String::new();

    while integer >= 1 {
        println!("n: {} r: {}", integer, integer % 2);
        binary_s = format!("{}{}", integer % 2, binary_s);
        integer /= 2;
    }
    binary_s
}

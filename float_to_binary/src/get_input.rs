pub mod interact {

    use std::io;

    pub fn input_to_f64() -> f64 {
        println!("Please enter your float number. ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error at read line");

        let float: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter a valid f64");
                input_to_f64()
            }
        };

        float
    }
}

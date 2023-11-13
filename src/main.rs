use std::time::Instant;
use rand::Rng;

use std::env;
use std::io;

fn catalan_number(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut result = 0;
    for i in 0..n {
        result += catalan_number(i) * catalan_number(n - 1 - i);
    }

    result
}

fn generate_birth_year() -> u32 {
    // Assuming we're interested in birthdays from the year 2000 onwards
    let mut rng = rand::thread_rng();
    rng.gen_range(2000..=2023)
}

fn main() {
    

    let year: i32 = get_year_value();

    println!("$$$User inputted Year: {}", year);

    let start_time = Instant::now();

    for _ in 0..10 {
        let birth_year = generate_birth_year();
        let catalan = catalan_number(birth_year as u64 % 10);
        println!("Birth Year: {}, Catalan Number: {}", birth_year, catalan);
    }

    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:?}", elapsed_time);
}

fn get_year_value() -> i32 {
    // Check if a command-line argument is provided
    let args: Vec<String> = env::args().collect();
    let mut year: u16 = 1990;

    let mut my_string: String;
    if let Some(arg) = env::args().nth(1) {
        my_string = arg;
        println!("String assigned from program argument: {}", my_string);
    } else {
        // If no program argument, prompt user for input
        println!("Please enter a string:");
        my_string = String::new();
        io::stdin().read_line(&mut my_string).expect("Failed to read line");
        println!("String assigned from user input: {}", my_string.trim());
    }


    let year = match string_to_integer(&my_string) {
        Ok(parsed_value) => {
            parsed_value
        }
        Err(err) => {
            eprintln!("Error asdf: {}", err);
            // Set a default value or handle the error as needed
            0
        }
    };

    year
}


fn string_to_integer(input_string: &str) -> Result<i32, std::num::ParseIntError> {
    let parsed_number = input_string.trim().parse::<i32>()?;
    Ok(parsed_number)
}





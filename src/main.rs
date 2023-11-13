use std::time::Instant;
use std::env;
use std::io;
use lazy_static::lazy_static;


lazy_static! {
    static ref LOG_ENABLED: bool = false;
}

fn main() {
    let year: i32 = get_year_value();
    let start_time = Instant::now();
    
    if *LOG_ENABLED {

        log(format!("User inputted Year: {}", year));
        let ranges = vec![(1700, 1710), (1800, 1810), (1900, 1920), (2000, 2010), (2300, 2310)];

        for (start, end) in ranges {
            log(format!("Iterating over the range {}-{}", start, end));

            for year in start..=end {
                log(format!("Year: {}, Programmer's Day: {}", year, get_programmers_day_date(year)));
            }

            log(format!("Finished iterating over the range {}-{}", start, end));
            log(format!(""));
        }
    }
    
    println!("{}", get_programmers_day_date(year));

    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:?}", elapsed_time);
}

fn log(txt: String) {
    if *LOG_ENABLED {
        println!("{}", txt);
    }
}

fn get_programmers_day_date(year: i32) -> String {
    let pivot_year: i32 = 1918;

    let comparison: i32 = compare(year, pivot_year);

    if comparison == -1 {
        get_julian_programmers_day(year)
    } else if comparison == 0 {
        get_transition_year_programmers_day(year)
    } else {
        get_gregorian_programmers_day(year)
    }
}

fn get_transition_year_programmers_day(year: i32) -> String {
    format!("26.09.{}", year)
}

fn get_gregorian_programmers_day(year: i32) -> String {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        format!("12.09.{}", year)
    }
    else {
        format!("13.09.{}", year)
    }
}

fn get_julian_programmers_day(year: i32) -> String {
    if year % 4 == 0 {
        format!("12.09.{}", year)
    }
    else {
        format!("13.09.{}", year)
    }
}

fn get_year_value() -> i32 {
    let mut my_string: String;
    if let Some(arg) = env::args().nth(1) {
        my_string = arg;
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

fn compare(x: i32, y: i32) -> i32 {
    if x < y {
        -1
    } else if x == y {
        0
    } else {
        1
    }
}






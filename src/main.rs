use std::env;
use std::io;
use std::time::Instant;

fn main() {
    let year: i32 = get_year_value();

    let start_time = Instant::now();

    println!("{}", get_programmers_day_date(year));

    println!("Execution time: {:?}", start_time.elapsed());
}

fn get_programmers_day_date(year: i32) -> String {
    if year == 1918 {
        format!("26.09.{}", year)
    } else if year < 1918 && year % 4 == 0
        || (year > 1918 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
    {
        format!("12.09.{}", year)
    } else {
        format!("13.09.{}", year)
    }
}

fn get_year_value() -> i32 {
    let mut year_string: String;
    if let Some(arg) = env::args().nth(1) {
        year_string = arg;
    } else {
        println!("User input year:");
        year_string = String::new();
        io::stdin()
            .read_line(&mut year_string)
            .expect("read_line failure");
    }

    year_string.trim().parse().expect("integer parse failure")
}

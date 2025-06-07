use chrono::{Local, NaiveDate, Utc};

fn export_types() {
    // Integers
    println!("Please enter your birthday (YYYY-MM-DD):");
    let mut birthday = String::new();
    std::io::stdin()
        .read_line(&mut birthday)
        .expect("Failed to read line");
    let birthday = birthday.trim();
    println!("Your birthday is: {}", birthday.trim());
    let age: u8 = 28;
    println!("Your age is: {}", age);

    let biggie: i32 = -1_000_000;
    println!("Big No. {}", biggie);

    let pi: f64 = 3.14159;
    println!("Pi is approximately: {}", pi);

    let is_learning = true;
    println!("Learning? {}", is_learning);

    let japanese: &str = "こんにちは";
    println!("Hello in Japanese is {}", japanese);
}

fn main() {
    let name = "Wilmer";
    let date = Local::now();
    let tomorrow = date + chrono::Duration::days(1);

    println!("Hello, {}", name);
    println!("Today is {}", date.format("%Y-%m-%d"));
    println!("Tomorrow is {}", tomorrow.format("%Y-%m-%d"));

    export_types();
}

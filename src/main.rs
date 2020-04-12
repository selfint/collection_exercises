use std::io::{self, Write};
mod ex_1;

fn main() {
    let list_of_integers = get_integers();
    let mean = ex_1::get_mean(&list_of_integers);
    let median = ex_1::get_median(&list_of_integers);
    let mode = ex_1::get_mode(&list_of_integers);
    println!("The mean is: {}", mean);
    println!("The median is: {}", median);
    println!("The mode is: {}", mode);
}

fn get_integers() -> Vec<i32> {
    // get amount of integers from user
    println!("How many numbers do you wish to enter?");
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    // convert string to int
    let amount: u32 = amount.trim().parse().expect("Failed to convert to u32");

    // get integers from user
    let mut integers: Vec<i32> = vec![];
    for i in 0..amount {
        let mut number = String::new();

        // get integer from user
        print!("Enter number {}: ", i);
        io::stdout().flush().unwrap(); // flush to stdout since we used print! and not println!
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Failed to convert to u32");
        integers.push(number);
    }

    integers
}

use std::time::{SystemTime};

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn print_separator(style: &str) {
    if style == "1" {
        println!("===============");
    } else if style == "2" {
        println!("---------------");
    } else if style == "3" {
        println!("###############");
    } else if style == "4" {
        println!("***************");
    } else if style == "5" {
        return;
    } else {
        // Choose the default style if the choice the user made doesn't match any of the ones listed above
        println!("===============");
    }
}

fn start_calculation(separator_style: &str) {
    // Ask the user for a number to do a calculation on
    println!("Which fibonacci number do you want to print? (0 is the lowest)");
    let mut choice = String::new(); 
    std::io::stdin().read_line(&mut choice).expect("Failed to read line"); // Store the user's choice
    let count: i32 = choice.trim().parse().unwrap(); // Convert the user's choice into a number

    print_separator(separator_style);
    println!("You selected the number: {}", count);
    print_separator(separator_style);
    
    let time_now = SystemTime::now(); // Get the current time (Start timer)

    let final_number = fibonacci(count);

    // Check and print how long it took to calculate the number
    match time_now.elapsed() {
        Ok(elapsed) => {
            println!("Time elapsed: {} ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("Ran into an error: {}", e)
        }
    }
    
    println!("Number {} of the sequence: {}", count, final_number);

    print_separator(separator_style);
}

fn main() {
    // Ask the user what style of separator lines to use
    println!("Choose separator line style: 1. = (default) \n2. - \n3. # \n4. * \n5. No separators");
    let mut choice = String::new(); 
    std::io::stdin().read_line(&mut choice).expect("Failed to read line"); // Store the user's choice
    let separator_style: &str = choice.trim(); // Remove whitespace from the string

    start_calculation(separator_style);

    // Loop until the user chooses to exit the program
    loop {
        println!("1. Print another number 2. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let trimmed_choice = choice.trim();
        if trimmed_choice == "1" {
            print_separator(separator_style);
            start_calculation(separator_style)
        } else {
            return;
        }
    }
}
fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn start_calculation() {
    println!("Which fibonacci number do you want to print? (0 is the lowest)");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let count: i32 = choice.trim().parse().unwrap();
    println!("===============");
    println!("You selected the number: {}", count);
    println!("===============");
    let final_number = fibonacci(count);
    println!("Number {} of the sequence: {}", count, final_number);
    println!("===============");
}

fn main() {
    start_calculation();
    loop {
        println!("1. Print another number 2. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let trimmed_choice = choice.trim();
        if trimmed_choice == "1" {
            println!("===============");
            start_calculation()
        } else {
            return;
        }
    }
}

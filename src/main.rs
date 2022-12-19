use std::io;
fn main() {
    println!("Input a number to get the corresponding fibonacci value");
    // add loop and user input, create exit situation
    let mut int = String::new();
    loop {
    io::stdin()
        .read_line(&mut int)
        .expect("Failed to read input");
    if int.trim( ) == "exit" {
        break;
    };

    let _int: i32 = match int.trim().parse() {
        Ok(int) => int,
        Err(_) => continue,
    };
    println!("Fibonacci ({}) => {}", int, 
    fibonacci(_int));
  }
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1{
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}
use std::io;

fn main() {
    loop {
        println!("\nEnter a number and I'll tell you if it's odd\n");

        let mut num: String = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Unexpected Error...");

        let num: i64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n=================================\n\tError: Enter a valid number\n=================================\n");
                continue;
            }
        };

        match num % 2 {
            0 => println!("\n{} is even", num),
            _ => println!("\n{} is odd", num),
        }
    }
}

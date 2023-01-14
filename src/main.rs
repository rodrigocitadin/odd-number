use std::io;

fn main() {
    loop {
        println!("\nWhat you want to know?\n");
        println!("\nIs it prime number? (1)       N prime number (2)\n");

        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler entrada");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n=================================\nError: Insert a valid option\n=================================\n");
                continue;
            }
        };

        print!("\nInsert a number\n");

        let mut number: String = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Falha ao ler entrada");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n=================================\nError: Insert a valid number\n=================================\n");
                continue;
            }
        };

        match choice {
            1 => is_it_prime(number),
            2 => n_prime_number(number),
            _ => continue,
        }
    }
}

fn is_it_prime(number: u64) {
    todo!()
}

fn n_prime_number(number: u64) {
    todo!()
}

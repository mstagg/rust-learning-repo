use std::io;
use std::cmp::Ordering;
use rand::Rng;

const MAX_NUM: i32 = 1000;
const MIN_NUM: i32 = 1;

fn fetch_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn valid_input(guess_num: i32) -> bool {
    if guess_num < MIN_NUM || guess_num > MAX_NUM {
        return false;
    } else {
        return true;
    }
}

fn main() {
    // Generate the secret number
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(MIN_NUM, MAX_NUM + 1);

    loop {
        // Fetch raw user input
        println!("Please input your guess between {}-{}: ", MIN_NUM, MAX_NUM);
        let guess_raw: String = fetch_input();

        // Parse from string to uint
        let guess_num: i32 = match guess_raw.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        // Check if guess is in range
        if !valid_input(guess_num) {
            println!("Input out of range.");
            continue;
        }

        // Process guess
        println!("You guessed: {}", guess_num);
        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Input too small!"),
            Ordering::Greater => println!("Input too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
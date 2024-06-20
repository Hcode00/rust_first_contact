use colored::Colorize;
use rand::Rng;

fn main() {
    loop {
        let mut guess = String::new();
        println!("Guess a number from 1 to 10");
        match std::io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.trim_end().parse() {
                Ok(value) => {
                    let secret_num = rand::thread_rng().gen_range(1..=10);
                    if secret_num == value {
                        println!("You Guessed the secret number.");
                        break;
                    } else {
                        println!("Wrong, the secret number is {}", secret_num)
                    }
                }
                Err(_error) => {
                    println!("{}", "Can't convert input to an integer!".red().bold());
                }
            },
            Err(_err) => {
                println!("{}", stringify!(err).red().bold());
            }
        }
    }
}

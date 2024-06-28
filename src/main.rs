use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        let mut guess = String::new();
        println!("Guess a number from 1 to 10 or q to quit");
        match std::io::stdin().read_line(&mut guess) {
            Ok(_) => {
                if guess.trim() == "q" {
                    println!("{}", "Quiting ...".red().bold());
                    return;
                }
                let guess: u32 = guess.trim_end().parse().expect(stringify!(
                    "Can't convert input to an integer!".red().bold()
                ));
                let secret_num: u32 = rand::thread_rng().gen_range(1..=10);
                match guess.cmp(&secret_num) {
                    Ordering::Equal => {
                        println!(
                            "{}",
                            "You Guessed The Secret Number Correctly".green().bold()
                        );
                        return;
                    }
                    Ordering::Less => {
                        println!("{}", "Aim a little higher".red())
                    }
                    Ordering::Greater => {
                        println!("{}", "Aim a little lower".red())
                    }
                }
            }

            Err(_err) => {
                println!("{}", stringify!(err).red().bold());
            }
        }
    }
}

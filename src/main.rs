use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Hello, guesser!");
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("blah");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        let guess:u32 = guess.trim().parse().expect("Please type a number");

        println!("the secret number is {}", secret_number);
        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
    }
}

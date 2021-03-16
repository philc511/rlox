fn main() {
    println!("Hello, world!");
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess);

        println!("you guessed: {}", guess);
}

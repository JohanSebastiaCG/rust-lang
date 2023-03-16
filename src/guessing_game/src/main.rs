use std::io;

// preludio ::io

fn main() {
    println!("guess the number");
    println!("please input your please: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed {guess}");
}   
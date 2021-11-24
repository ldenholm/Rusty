use std::io;
use rand::Rng;

fn main() {
    println!("Greetings traveller, welcome to the game a la guess");
    println!("We are thinking of a natural number, please enter your best guess");
    let rand = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();
    // vars are immutable by default.
    // let apples = 5; binds 5 to apples, apples is immutable.
    // let mut apples = 5; '            ' apples is mutable.

    io::stdin()
        .read_line(&mut guess) // similarly, references are immutable by default.
        // so &mut is used to declare mutable reference to address of guess variable.
        .expect("failed to read line");
    
    println!("You guessed {}", guess);
    let mut apples = 5;
    let pears = 3;
    println!("apples = {}", apples);
    apples = 7;
    println!("apples = {}, pears = {}", apples, pears);

    /* we can use:
        
    let x = 5;
    let y = 10;
    println!("value of x = {}, value of y = {}", x, y);

    */
}
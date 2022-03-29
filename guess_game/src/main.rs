use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Greetings traveller, welcome to the game a la guess");
    let secret_number = rand::thread_rng().gen_range(1..101); //range inclusive of lower bound but not upper, hence we set upper to 101.

    println!("the secret_number is: {}", secret_number);
    // vars are immutable by default.
    // let apples = 5; binds 5 to apples, apples is immutable.
    // let mut apples = 5; '            ' apples is mutable.

    loop {
        println!("We are thinking of a natural number, please enter your best guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // similarly, references are immutable by default.
            // so &mut is used to declare mutable reference to address of guess variable.
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
        Above we use shadowing to reuse the guess variable name rather than forcing us
        to create two unique variables, such as guess_str and guess for example.
        */

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Congrats, you guessed the correct number!");
                break;
            }
        };
    }
}
    // examples of compile errors from mutable/immutable vars:
    // let mut apples = 5;
    // let pears = 3;
    // println!("apples = {}", apples);
    // apples = 7;
    // println!("apples = {}, pears = {}", apples, pears);

    // trivial change
    /* we can use:
    let x = 5;
    let y = 10;
    println!("value of x = {}, value of y = {}", x, y);

    */

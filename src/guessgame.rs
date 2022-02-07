use rand::Rng;
use std::io;

pub fn run() 
{
    let mut rng = rand::thread_rng();
    let answer : i32 = rng.gen_range(1..10);

    //println!("Generated number: {}", answer);
    let mut no_guesses : i16 = 0;
    while guess != answer {
        let mut input = String::new(); // Not sure if this is ideal, but when I put it outside the scope of the loop then it would retain the previous values, which I didn't know how to deal with without raising the complexity of the code, which would probably have a similar impact on performance if not bigger anyway. I don't know anything.
        println!("Currently string is as follows: {}", input);
        io::stdin() // No need to wrap this in a method I believe since it's just one time use
            .read_line(&mut input)
            .expect("Error!");
        guess = input.trim().parse().expect("Your guess needs to be a number.");
        if guess == answer {
            println!("Congratulations! You won.");
        }
        else if guess > answer {
            println!("Too high.");
        }
        else {
            println!("Too low!");
        }
        no_guesses = no_guesses + 1;
    }
    println!("It took you {} times to guess. Thank you for playing.", no_guesses);
}
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();
    let Some(word) = args.next() else {
        println!("expected word as second argument");
        process::exit(2);
    };

    println!("World: {word}");
}

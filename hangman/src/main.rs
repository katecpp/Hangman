extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;
// use std::path::Path;

fn main() {
    println!("Hello, world!");

    let mut discovered_letters = String::new();
    
    let secret_line = read_input()
                      .expect("Failed to read input data!");

    let a = String::from("Kasia");
    let b = String::from("a");
    print_masked_string(secret_line, b);

    let mut user_guess = read_guess();
    // TODO: check if input is appearing in line...
    // TODO: print as for each undiscovered char

}

fn read_guess() -> Result<String, io::Error>
{
    println!("Please input your guess.");

    // TODO: read one char instead of line
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);

    Ok(guess)
}

fn read_input() -> Result<String, io::Error>
{
    let f = try!(File::open("input.txt"));
    let file = BufReader::new(&f);

    let mut v: Vec<String> = Vec::new();

    for line in file.lines()
    {
        let l = line.unwrap();
        println!("{}", l);
        v.push(l);
    }

    println!("Read {} lines", v.len());

    let random_line = rand::thread_rng().gen_range(1, v.len());
    let secret_line = v[random_line].clone();
    println!("Randomly chosen line: {}. {}", random_line, secret_line);

    Ok(secret_line)
}

fn print_masked_string(input: String, mask: String)
{
    println!("Original string: {}. Mask: {}", input, mask);

    for (u, c) in input.chars().enumerate()
    {
        print!("{} ",
                if c == ' ' {c}
                else if mask.contains(c) {c}
                else {'_'});
    }

    print!("\n");
}

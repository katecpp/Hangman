extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;
// use std::path::Path;

fn main() {
    println!("Hello, world!");

    read_input().expect("failed to read input data!");
    read_guess();
}

fn read_guess() {
    println!("Please input your guess.");

    // TODO: read one char instead of line
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
}

fn read_input() -> Result<(), io::Error>
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
    println!("Randomly chosen line: {}. {}", random_line, v[random_line]);

    Ok(())
}


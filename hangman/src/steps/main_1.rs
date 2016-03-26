use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main()
{
    read_input().expect("Failed to read input data!");
}

fn read_input() -> Result<(), io::Error>
{
    let f = try!(File::open("input.txt"));

    let file = BufReader::new(&f);
    let mut v: Vec<String> = Vec::new();

   for line in file.lines()
    {
        let l = line.unwrap().to_lowercase();
        println!("{}", l);
        v.push(l);
    }

    println!("Correctly loaded {} lines.", v.len());

    Ok(())
}

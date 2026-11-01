use missing_number::missing_number;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let n: u64 = values.next().unwrap().parse().unwrap();

    let numbers: Vec<u64> = values.map(|x| x.parse().unwrap()).collect();

    let result = missing_number(n, &numbers);

    println!("{}", result);
}

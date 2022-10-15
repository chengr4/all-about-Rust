// std = standard
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    // mutable
    let mut name = String::new();
    //g unmutable
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Did not receive input");
    println!("Hello {} {}", name.trim_end(), greeting);

    // variable
    // > 0
    const ONE: u16 = 1;
    const ONE_MIL: u32 = 1_000_000;
    // shadowing
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("msg");

}

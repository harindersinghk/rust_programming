#![allow(unused)]

use std::io::{self, Read};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // println!("What is your name?");
    // let mut name: String = String::new();
    // let greeting: &str= "Nice to meet you";
    // io::stdin().read_line(&mut name)
    // .expect("Didn't Recieve Input!");

    // println!("Hello, {}! {}", name.trim_end(),greeting);
    
    
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age: &str = "24";
    // let mut age: u32 = age.trim().parse()
    // .expect("Age wasn't assigned a number");

    // age = age + 1;
    // println!("I'm {} and I want ${}",age,ONE_MIL);

    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
    
}

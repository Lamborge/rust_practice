use std::io;
use std::io::Write;

fn main() {
    let mut num = String::new();

    println!("Insert lenght of numbers: ");
    io::stdout().flush().expect("flush failed!");

    io::stdin().read_line(&mut num)
        .expect("Error read line");

    let num: u32 = num.trim().parse().expect("Enter a number!");

    let mut n1: usize;
    let mut n2: usize = 0;
    let mut current: usize = 1;

    let mut i: u32 = 1;
    while i < num+1 {
        print!("{current} ");
        n1 = n2;
        n2 = current;
        current = n1 + n2;
        
        i += 1;
    }
    println!();
}

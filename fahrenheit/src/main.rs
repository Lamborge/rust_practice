use std::io;
use io::Write;

fn main() {
    print!("\nEnter Celsius(°C):");
    io::stdout().flush().expect("flush failed!");

    let mut cel = String::new();
    io::stdin().read_line(&mut cel).
        expect("Err");

    let cel: f64 = cel.trim().parse().expect("Please insert a float number!");

    let far: f64 = (cel * (9.0/5.0)) + 32.0;

    println!("The temperature in Fahrenheit is: {far}°F");
}

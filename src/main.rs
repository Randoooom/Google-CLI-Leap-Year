use std::io;
use std::process;

fn main() {
    println!("Please enter a year");

    let mut year = String::new();

    io::stdin().read_line(&mut year)
        .expect("Failed to read line");

    let uyear: i16 = year.trim().parse()
        .expect("Please enter a valid year!");

    if uyear <= 0{
        println!("Please enter a valid year!");
        process::exit(1);
    }

    if (uyear % 4) == 0 || (uyear % 100) == 0 || (uyear % 400) == 0{
        println!("The year {} is a leap year!", year);
    }
    else {
        println!("The year {} isn`t a leap year!", year);
    }
}

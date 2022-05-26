use std::{thread, time};
use std::io;

fn main() {

    let mut result:f64 = 0.0;
    let mut i = 0;

    println!("Hello World!");
    println!("Welcome to my first Rust Program!");

    delay_ms(2000);

    while i <= 10{
        println!(" ");
        i += 1;
    }
    
    loop {

        println!("Enter the first number");
        let mut strNum1 = readUser();

        println!("Enter operator: [+, -, /, *]");
        let mut operator = readUser();

        println!("Enter the second number");
        let mut strNum2 = readUser();

        let mut num1: f64 = strNum1.parse().unwrap();
        let mut num2: f64 = strNum2.parse().unwrap();

        if operator == "+"
        {
            result = num1 + num2;
        }

        if operator == "-"
        {
            result = num1 - num2;
        }

        if operator == "/"
        {
            result = num1 / num2;
        }

        if operator == "*"
        {
            result = num1 * num2;
        }

        println!("The Result is: {}", result);

        delay_ms(5000);

        i = 0;

        while i <= 10{
            println!(" ");
            i += 1;
        }

        
    }
}

fn delay_ms(timeVal:u64){

    thread::sleep(time::Duration::from_millis(timeVal));

}

fn delay_us(timeVal:u64){

    thread::sleep(time::Duration::from_micros(timeVal));

}

fn readUser() -> String{

    let mut line = String::new();
    //thread::sleep(time::Duration::from_millis(5000));

    match io::stdin().read_line(&mut line)
    {
        Ok(__) => {
            //println!("{}", line);
        },
        Err(e) => println!("Something went wrong! {}", e)
    }

    line.trim().to_string()
}

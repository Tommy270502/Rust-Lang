use std::{thread, time};
use std::io;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

fn main() {
    println!("Tommy's PW Generator");
    println!("Enter password length: ");
    let userInput = readUser();
    let mut pwLength: u32 = userInput.parse().unwrap();
    
    let mut rng = rand::thread_rng();

    let password: String = (0..pwLength)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{}", password);

    delay_ms(5000);

    clearScreen();
}

fn clearScreen() {

    let mut i = 0;

    while i <= 10{
        println!(" ");
        i += 1;
    }

}

fn delay_ms(timeVal:u64){

    thread::sleep(time::Duration::from_millis(timeVal));

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
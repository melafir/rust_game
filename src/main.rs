use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;
fn main() {
    println!("Welcome to guesting game!");
    let sercet_number:u32 = rand::thread_rng().gen_range(1..101);
    loop {
        print!("Pick your number: ");    
        let mut num = String::new();        
        match std::io::stdin()
        .read_line(&mut num){
            Ok(num) => num,
            Err(_) => {
                println!("Enter number please!");
                continue;
            },
        };
        let num:u32 = match num.trim().parse(){
            Ok(num) =>num,
            Err(_) => continue,
        };

        println!("");
        println!("Your number is {}", num);
        //println!("Secret number is {}", sercet_number);

        match num.cmp(&sercet_number) {
            Ordering::Less => println!("{}","Lesser!".red()),
            Ordering::Greater => println!("{}","Greater!".blue()),
            Ordering::Equal =>{
                println!("{}","Well done!".yellow().bold());
                break;
            } ,
        }
    }    
    
}

use rand::Rng;
use std::cmp::Ordering;
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
            Ordering::Less => println!("Lesser!"),
            Ordering::Greater => println!("Greater!"),
            Ordering::Equal =>{
                println!("Well done!");
                break;
            } ,
        }
    }    
    
}

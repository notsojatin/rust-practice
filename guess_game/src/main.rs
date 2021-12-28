use rand::Rng; //import rand crate for generating random number
use std::cmp::Ordering;
use std::io; //import stdout package for input output

fn main() {
    println!("---------Guess the number---------");

    loop {
        println!("Input a number");

        //request a random number between 1 to 100 both inclusive
        //equivalently we can use (1..=10)
        let secret_number = rand::thread_rng().gen_range(1..101);
        //create a mutable variable
        let mut guess_num = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read line");

        // if expect is not written than build will thow a warning

        let guess_num: u32 = match guess_num.trim().parse() {
          Ok(num)=>num,
          Err(_)=>{
            println!("Wrong input format");
            continue;
          }
        };
        println!("You guessed: {}", guess_num);

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Too small, num: {}", secret_number),
            Ordering::Greater => println!("Too large, num: {}", secret_number),
            Ordering::Equal => {
              println!("You win!");
              break;
            }
        }
    }
}

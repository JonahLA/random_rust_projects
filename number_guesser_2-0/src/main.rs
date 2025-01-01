use std::env;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn usage() {
    println!("Usage: ./number_guesser_2-0 <guess>");
    std::process::exit(EXIT_FAILURE);
}

fn main() {
    // validate and parse input
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            // prase guess
            let guess: i32 = match args[1].parse() {
                Ok(n) => { n },
                Err(_) => {
                    eprintln!("Value given for guess ({}) could not be parsed", args[1]);
                    std::process::exit(EXIT_FAILURE);
                }
            };

            // generate random number
            let rand_num: i32 = 47;  // TODO: switch to use random library
        
            // check if correct
            if guess == rand_num {
                println!("That is correct!");
            } else {
                println!("Sorry, that is incorrect. The correct number was {}", rand_num);
            }
            std::process::exit(EXIT_SUCCESS);
        },
        _ => {
            usage()
        }
    }
}

extern crate rand; //declare that you'll need an external dependency, imported via  your cargo.toml file

use std::io;  //let us use some types not brought by the 'default types' called 'the prelude'
use std::cmp::Ordering;
use rand::Rng; //Rng is a Trait... 'cargo doc --open' will generate a doc of used crates, so we'll know why we need it

//Every program starts with its main function
fn main() {
    println!("Guess the number!");

    //rand::thread_rng() will return a number generator that exposes the gen_range method
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{ //loop creates an infinite loop
        println!("Please input your guess.");
        //let instantiate an immutable variable (mut here, let it be mutable)
        let mut guess = String::new(); //new is an associated function of type String. This is the name in Rust for a static method

        //We could use std::io::stdin if we hadn't import it at the beginning of the file
        //io::stdin() returns a instance of std::io::stdin
        //readline here, is a method of the type std::io::stdin, that takes a mutable String as an argument
        //& indicates that the argument is a reference, and mut let this reference be mutable
        io::stdin().read_line(&mut guess) //read_lin will return an instance of io::Result, which is a specific type of Result. Its is an enum that contains Ok and Err
            .expect("Failed to read line"); //Result instances have a 'expect' method. If if gets an Err, it will crash and display the param message. If it receives Ok, it will just return the value passed to it by read_line
            //If you compile Rust code, without this expect (or any Error Handling), you'll get a warning for not using the returned Result object

        //guess already exists, so let guess will 'shadow' it
        //:u32, let the type inference of Rust tell to parse how to behave and moreover tells the compiler that secret_number will be a u32 too because it is compared to guess later.
        let guess: u32 = match guess.trim().parse() { //Error handling example using pattern matching over the values of the Result enum
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value used to match all kind of Errors, and continue goes to the next loop iteration
        };


        println!("You guessed: {}", guess);

        //match is used for pattern matching
        match guess.cmp(&secret_number) { //cmp return an Ordering, which, like Return, is an enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //Let the infinte loop terminate
            },
        }
    }
}

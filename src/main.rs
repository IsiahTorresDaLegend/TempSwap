
/*
--------------------------------------------------------------------------
Program: Name That Movie Quiz
Author: Isiah Torres
--------------------------------------------------------------------------
*/


// add a lib so we can input from the command line
use std::io;

// main function is the entry point into our program
fn main() {

    // let is used to create a variable
    // title is the name of the variable
    // r"" is a raw string literal, allows us to use any character without escape codes. 
    // The desired output is placed between the "". Used raw string literal to display ASCII art.
    let title = r"
     _   _                        _____ _           _     __  __            _         ___        _     
    | \ | | __ _ _ __ ___   ___  |_   _| |__   __ _| |_  |  \/  | _____   _(_) ___   / _ \ _   _(_)____
    |  \| |/ _` | '_ ` _ \ / _ \   | | | '_ \ / _` | __| | |\/| |/ _ \ \ / / |/ _ \ | | | | | | | |_  /
    | |\  | (_| | | | | | |  __/   | | | | | | (_| | |_  | |  | | (_) \ V /| |  __/ | |_| | |_| | |/ / 
    |_| \_|\__,_|_| |_| |_|\___|   |_| |_| |_|\__,_|\__| |_|  |_|\___/ \_/ |_|\___|  \__\_\\__,_|_/___|
    ";
    
    // created another variable with the 'format!' macro which is used to create a formatted string.
    // \n is a newline character.
    // \xtb[33;1;5m is an ANSI escape code to change the text to yellow (33), bold (1), and 
    // blinking (5), ending with 'm'. The {} is where the value of arguments will be inserted.
    // \x1b[0m resets the style back to default. 'title' is the argument passed.
    let styled_title = format!("\n\n\x1b[33;1;5m{}\n\n\x1b[0m", title);

    // print a title to the console. println! Adds a newline to the end automatically. 
    // the value of the styled_title variable will be output as a string "{}"
    println!("{}", styled_title);

    // create a string to hold the user's input
    // let is used to create a variable
    // mut is used to make a variable mutable (changeable), not mutable would be (readonly)
    // user_answer is the name of the variable.
    // ': String' lets you know it is a string type, but not required. 
    // = String::new(); creates a new string object.
    let mut user_answer = String::new();

    // will be used to stop while loop. String::from("y") assigns the value to "y"
    let mut try_again = String::from("y");

    // while loop will continue until try_again is not equal to "n"
    while try_again != "n" {

    // print a movie quote to the console. Does not add a newline to the end. 
    print!("\n\"May the Force be with you.\" - is from what movie?\n");

    // get the user input and store it in the user_answer variable
    // io is the input output module
    // stdin() is a function within the io module, it returns a handle to the std input stream
    // .read_line() is a method call on the stdin handle, used to read a line of text from 
    // the standard input and store it in a mutable string buffer '&mut user_answer'
    // '&mut user_answer' is a mutable (mut) reference (&) to the user_answer variable. 
    // References allow parts of code to access and modify a value without taking ownership. 
    // .expect() is an error handling method chained onto the read_line() operation
    // .expect() will print a message to should an error occur.
    user_answer.clear();  // clears buffer for new input.
    io::stdin().read_line(&mut user_answer).expect("Failed to read line");
    
    // TRIM the user input to remove any extra white space with .trim()
    // convert the value to a string type with .to_string()
    user_answer = user_answer.trim().to_string();

    // Make the user input all lowercase with .to_lowercase()
    user_answer = user_answer.to_lowercase();

    // check the user input against the correct answer
    // if user_answer is equal to "star wars"...
        if user_answer == "star wars"{

            // display this message to the user.
            println!("\nYou are correct! A jedi you are.");
            break; // immediately break out of loop.
        }
        // otherwise...
        else{
            // display this message to user. 
            println!("\nAnswer is not correct! You are not a jedi.");
            println!("\nTry again? (Enter 'n' to quit or any charcter to try again.)");
            try_again.clear(); // clear buffer for new input.
            // store user input into try_again variable.
            io::stdin().read_line(&mut try_again).expect("Failed to read line");
            try_again = try_again.trim().to_string(); // trim user input and convert to string.
            try_again = try_again.to_lowercase(); // convert string to lowercase. 
        }
    } // end while loop.

    // End program by telling the user goodbye
    println!("\n\n\t\x1b[34;1m*** Thank you for using our movie quiz. Goodbye! ***\n\n\x1b[0m");

}



// on main rust file remember to put
// mod defaultLibary;
// use defaultLibary::*;
pub use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    // pause function allows to let the user read and wait until they are ready to continue
    println!("Press enter to continue...");
    input();
}

pub fn clear() {
    //clears the screen
    print!("\x1B[2J\x1B[1;1H");
}

pub fn input() -> String {
    // gets stdin and sets to buffer
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    // get tmps and removes the last 2 characters of it    
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if cfg!(unix) {
            if x == (input.len() - 1) {
                break;
            }
        }
        else if cfg!(windows) {
            if x == (input.len() - 2) {
                break;
            }
        }
        
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    // returns the modified tmp
    return tmp;
}

pub fn input_pr(content: String) -> String {
    // prints a single line without \n and flushes the stdout
    print!("{}", content);
    std::io::stdout().flush().unwrap();

    // gets stdin and sets to buffer
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    // get tmps and removes the last 2 characters of it    
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if cfg!(unix) {
            if x == (input.len() - 1) {
                break;
            }
        }
        else if cfg!(windows) {
            if x == (input.len() - 2) {
                break;
            }
        }
        
        tmp.push(tmp_chars.nth(0).unwrap());
    }

    // returns the modified tmp
    return tmp;
}


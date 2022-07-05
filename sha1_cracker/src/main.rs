use std::env;

fn main() {
    // args comes from env::args() and returns an iterator that can be "collected" into a Vec<String> (a vector of string objects)
    // a vector is an array that can be resized
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        // the ! in prinln! specifies that it is a macro
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return;
    }
}
mod eve;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
fn main() {
    // Purpose:    Driver for DH problems
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      arg_tests/ and stdio_tests/
    // Status:     Student does this
    let args: Vec<String> = env::args().collect();

    let mut new_args: Vec<String> = [].to_vec();
    let mut u_input_test = String::new();
    let mut u_mode = String::new();
    let mut u_key = String::new();
    if args.len() == 3 {
        let mut read_file = File::open(args[1].to_string()).expect("file not found");
        let mut contents = String::new();
        read_file
            .read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let words = contents.split(" ");
        let word_list = words.collect::<Vec<&str>>();
        new_args.push(word_list[0].to_string());
        new_args.push(word_list[1].to_string());
        new_args.push(word_list[2].to_string());
        let output = eve::baby_eve(
            new_args[0].parse::<u64>().unwrap(),
            new_args[1].parse::<u64>().unwrap(),
            new_args[2].parse::<u64>().unwrap(),
        );
        let mut ofile = File::create(args[2].to_string()).expect("unable to create file");
        ofile
            .write_all(output[0].to_string().as_bytes())
            .expect("unable to write)");
        ofile
            .write_all(" ".to_string().as_bytes())
            .expect("unable to write)");
        ofile
            .write_all(output[1].to_string().as_bytes())
            .expect("unable to write)");
        ofile
            .write_all(" ".to_string().as_bytes())
            .expect("unable to write)");
        ofile
            .write_all(output[2].to_string().as_bytes())
            .expect("unable to write)");
    } else if args.len() == 4 {
        new_args.push(args[1].to_string());
        new_args.push(args[2].to_string());
        new_args.push(args[3].to_string());
        let output = eve::baby_eve(
            new_args[0].parse::<u64>().unwrap(),
            new_args[1].parse::<u64>().unwrap(),
            new_args[2].parse::<u64>().unwrap(),
        );
        println!("{} {} {}", output[0], output[1], output[2]);
    } else {
        io::stdin()
            .read_line(&mut u_input_test)
            .expect("Failed to read line");
        new_args.push(u_input_test.replace("\n", "").to_string());
        io::stdin()
            .read_line(&mut u_mode)
            .expect("Failed to read line");
        new_args.push(u_mode.replace("\n", "").to_string());
        io::stdin()
            .read_line(&mut u_key)
            .expect("Failed to read line");
        new_args.push(u_key.replace("\n", "").to_string());
        let output = eve::baby_eve(
            new_args[0].parse::<u64>().unwrap(),
            new_args[1].parse::<u64>().unwrap(),
            new_args[2].parse::<u64>().unwrap(),
        );
        println!("{} {} {}", output[0], output[1], output[2]);
    }
}

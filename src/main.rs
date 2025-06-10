use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn read_file(file: &File) -> io::Result<()> {
    let reader = BufReader::new(file); // Buffered reader for efficient reading

    // Read the file line by line
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line), // Print each line to stdout
            Err(error) => return Err(error),  // Return error if reading fails
        }
    }
    Ok(()) // Return Ok if reading is successful
}

fn write_to_file(file_path: &str) {
    let mut input = String::new(); // String to hold user input
    println!("Enter text to write to file: "); // Prompt user for input

    // Flush stdout to ensure the prompt is displayed before reading input
    if let Err(error) = io::stdout().flush() {
        // Handle errors that occur while flushing stdout
        eprintln!("Failed to flush stdout: {}", error);
        std::process::exit(1);
    }

    // Read user input
    if let Err(error) = io::stdin().read_line(&mut input) {
        // Handle errors that occur while reading input
        eprintln!("Error reading input: {}", error);
        std::process::exit(1);
    }

    // Open the file in append mode, creating it if it doesn't exist
    let mut file = match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(file) => file,
        Err(error) => {
            // Handle errors that occur while opening the file
            eprintln!("Failed to open file for writing: {}", error);
            std::process::exit(match error.kind() {
                std::io::ErrorKind::NotFound => 2,
                std::io::ErrorKind::PermissionDenied => 3,
                _ => 1,
            });
        }
    };

    // Write the input to the file
    if let Err(error) = file.write_all(input.as_bytes()) {
        // Handle errors that occur during writing
        eprintln!("Error writing to file: {}", error);
        std::process::exit(match error.kind() {
            std::io::ErrorKind::PermissionDenied => 3,
            std::io::ErrorKind::WriteZero => 1,
            _ => 1,
        });
    } else {
        println!("Data written successfully!") // Notify user of successful write
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(4);
    }

    let file_path = &args[1]; // Get the file path from command line arguments
    let mut mode_input = String::new(); // String to hold user input for mode selection

    // Prompt user for mode selection until valid input is received
    loop {
        println!("Enter 'r' to read file, or 'w' to write to file: ");

        // Flush stdout to ensure the prompt is displayed before reading input
        if let Err(error) = io::stdout().flush() {
            // Handle errors that occur while flushing stdout
            eprintln!("Failed to flush stdout: {}", error);
            std::process::exit(1);
        }

        mode_input.clear(); // Clear the previous input

        // Read user input for mode selection
        if let Err(error) = io::stdin().read_line(&mut mode_input) {
            // Handle errors that occur while reading input
            eprintln!("Error reading input: {}", error);
            continue; // Restart loop to prompt user again
        }

        // Trim whitespace and check if the input is valid
        let mode = mode_input.trim();
        if mode == "r" || mode == "w" {
            break;
        }
        println!("Invalid input."); // Notify user of invalid input
    }

    // Match the mode input to determine the action to take
    match mode_input.trim() {
        "r" => {
            // Attempt to open the file for reading
            match File::open(file_path) {
                // If the file is opened successfully, read it
                Ok(file) => {
                    if let Err(error) = read_file(&file) {
                        // Handle errors that occur while reading the file
                        eprintln!("Error reading file: {}", error);
                        std::process::exit(1);
                    }
                }
                // If there is an error opening the file, handle it
                Err(error) => {
                    // Handle errors that occur while opening the file
                    eprintln!("Error opening file: {}", error);
                    std::process::exit(match error.kind() {
                        std::io::ErrorKind::NotFound => 2,
                        std::io::ErrorKind::PermissionDenied => 3,
                        _ => 1,
                    });
                }
            }
        }
        "w" => write_to_file(file_path), // Call the function to write to the file
        _ => unreachable!(),
    }
}

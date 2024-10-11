use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).expect("File could not be created.");
    for book in books {
        write!(file, "{}, {}, {}\n", book.title, book.author, book.year).expect("Data could not be written.")
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).expect("File could not be opened.");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Line could not be read");
        let parts: Vec<&str> = line.split(',').collect();
        let year = parts[2].trim().parse::<u16>().expect("Year is not valid");

        books.push( Book {
            title: parts[0].to_string(),
            author: parts[1].to_string(),
            year
        });
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "How to Make a Time Machine".to_string(), author: "Andrew Aboytes".to_string(), year: 2024 }
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by{}, published in {}", book.title, book.author, book.year);
    }
}
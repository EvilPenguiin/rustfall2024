use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};

struct Car {
    maker: String,
    year: u32,
}

fn create_file() {
    File::create("user_info.txt").unwrap();
}

fn write_file(car: &Car) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("user_info.txt")
        .unwrap();
    writeln!(file, "Maker: {}, Year: {}", car.maker, car.year).unwrap();
}

fn read_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn reading_from_console() -> Car {
    let mut buffer = String::new();

    print!("What's your car maker? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let maker = buffer.trim().to_string();
    buffer.clear();

    print!("What's your car year? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    Car { maker, year }
}

fn main() {
    create_file();

    let car = reading_from_console();

    write_file(&car);

    read_file();
}

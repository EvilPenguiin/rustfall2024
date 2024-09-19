fn main() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    fn is_even(n: i32) -> bool {
        if n % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }

    for i in array {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}: FizzBuzz", i);
        } else if i % 5 == 0 {
            println!("{}: Buzz", i);
        } else if i % 3 == 0 {
            println!("{}: Fizz", i);
        } else if is_even(i) {
            println!("{}: Even", i);
        } else {
            println!("{}: Odd", i);
        }
    }
    println!();

    let mut i = 0;
    let mut total = 0;
    while i < array.len() {
        total += array[i];
        i += 1;
    }
    println!("The sum of all numbers in the array is: {}", total);
    println!();

    let mut large_num = array[0];
    for n in array {
        if n > large_num {
            large_num = n;
        }
    }
    println!("The largest number in the array is: {}", large_num);
}

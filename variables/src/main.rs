use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("spaces is : {}", spaces);

    let literals = b'B' - b'A';
    println!("{}", literals);

    let x = 2.0;
    let y: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", tup.0);

    let mut array = [3; 5];
    array = [1, 2, 3, 4, 5];

    println!("The first value of array is: {}", array[0]);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )

}
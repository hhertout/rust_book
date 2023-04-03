use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x*2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("One slot in the tupple: {_y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];

    println!("The value of the elment at index {index} is: {element}");

    second_func(5, 'h');

    let x = plus_one(5);
    println!("The value of x is {x}");

    loop_naming();
    while_func();
    for_func();
}
//This is a comment

fn second_func(x: i32, unit_label: char) {
    println!("Another function, the value sent is {x}{unit_label}");
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn loop_naming() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {count}")
}

fn while_func() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF !!!");
}

fn for_func() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
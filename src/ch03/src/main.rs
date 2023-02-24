use std::io;

const THERE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is :{}", x);
    let y = x;
    println!("The value of y is :{}", y);
    // x = 6;
    let x = 6;
    println!("The value of x is :{}", x);
    println!("{}", THERE_HOURS_IN_SECONDS);
    {
        let x = x * 2;
        println!("inner x = {}", x);
    }
    println!("outer x = {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // let guess :u32 = "42".parse().unwrap();
    let guess = "42".parse::<u32>().unwrap();
    println!("{}", guess);

    let x = 2.0;
    let y = 4.5;
    let z = x + y;
    println!("the sum of x and y is {}", z);

    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
    println!("{}", diff);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", floored);
    println!("{}", remainder);

    let mut t = true;
    t = false;
    println!("t: {}", t);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup = (10, 20.0, Some(10));

    let (x, y, z) = tup;
    println!("{}", z.unwrap());

    let mut a = [1, 2, 3, 4, 5, 6];
    // a[10] = 20;
    println!("{:?}", a);

    let b = [3u8, 5];

    assign_to_array()
}

fn assign_to_array() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();

    let index = index.trim().parse::<usize>().unwrap();
    let element = a.get(index).unwrap_or(&10);
    println!("the value of element is {}", element);
}

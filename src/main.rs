fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!(
        "Hi {}, on your next birthday, you'll be {}!",
        name, next_age
    );
}

fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let mut x = 5;
    let y = 6;
    let z = x + y;

    println!("z is {}", z);

    x += 1;

    println!("x is {}", x);

    let a = true;
    let b = false;

    if a {
        println!("a is true");
    }

    if b {
        println!("b is true");
    }

    let tup = (1, 'c', true);
    let first = tup.0;
    let second = tup.1;

    println!("the first is {} and the second is {}", first, second);
    next_birthday("Bob", 34);
    println!("The answer is {}", square(32));
}

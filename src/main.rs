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
}

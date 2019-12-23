fn main() {
    for i in 1..11 {
        println!("Now serving number {}", i);
    }

    let x = 3;
    match x {
        1 => println!("One is the loneliest number."),
        2 => println!("Two's company"),
        3 => println!("Three is a crowd"),
        _ => println!("Some other number"),
    }
}

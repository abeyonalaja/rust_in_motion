fn main() {
    let s = String::from("book");
    let p = pluralize(s.clone());
    println!("I have one {}, you have two {}", s, p,);
}

fn pluralize(word: String) -> String {
    word + "s"
}

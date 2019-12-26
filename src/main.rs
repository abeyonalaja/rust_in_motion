struct Person {
    name: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {}!!!", person.name);
}

fn main() {
    let s = String::from("book");
    let p = pluralize(&s);
    println!("I have one {}, you have two {}", s, p,);

    let ps = Person {
        name: String::from("Abey"),
    };
    congratulate(&ps);
    println!("Can still use ps here: {}", ps.name)
}

fn pluralize(word: &str) -> String {
    word.to_owned() + "s"
}

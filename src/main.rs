use std::collections::HashMap;

struct Player {
    score: i32,
}

impl Player {
    fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    fn score(&self) -> i32 {
        self.score()
    }

    fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    let mut player1 = Player::new();
    let mut score = player1.score;

    player1.set_score(score + 1);

    let text = "hello world hello";
    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequences: {:?}", freqs);
}

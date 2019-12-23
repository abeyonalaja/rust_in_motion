enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It is about {} o'clock", hours),
        Clock::Digital(hours, minutes) => println!("It is {} minutes past {} ", minutes, hours),
        Clock::Analog(hours, minutes, seconds) => {
            println!(
                "It is {} minutes and {} seconds past {} 0'clock",
                minutes, seconds, hours
            );
        }
    }
}

fn next_player(position: HockeyPosition) {}

fn main() {
    let position = HockeyPosition::Defense;
    next_player(position);
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

    tell_time(Clock::Analog(1, 51, 30));
}

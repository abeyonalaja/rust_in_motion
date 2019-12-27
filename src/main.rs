#[derive(Debug)]
struct Bucket {
    liters: u32,
}

#[derive(Debug)]
struct CarPool {
    passengers: Vec<String>,
}

impl CarPool {
    /// Add the named passenger to the carpool
    fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 10 };

    let mut monday_car_pool = CarPool { passengers: vec![] };

    monday_car_pool.pick_up(String::from("Abey"));
    println!("Carpool state: {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Christy"));
    println!("Carpool state: {:?}", monday_car_pool);

    pour(&mut bucket1, &mut bucket2, 3);

    println!("Bucket 1: {:?}", bucket1);
    println!("Bucket 2: {:?}", bucket2);
}

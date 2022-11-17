mod marv;
mod pid;

use std::time::{Duration, SystemTime};

use marv::{Coord, Marv};

fn main() {
    let mut marv = Marv::new(8.0, 7.0, 3.5, Coord { x: 0.0, y: 0.0 });
    let start = SystemTime::now();
    marv.run(Duration::from_millis(2500), 200000.0);
    println!("{}ms", start.elapsed().unwrap().as_millis());
}

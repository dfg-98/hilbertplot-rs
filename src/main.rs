mod hilbertcurve;
mod point;
use point::HPoint;

fn main() {
    let mut p = HPoint::new(1, 2);
    p.set_index(3);
    let mut p2 = HPoint::new(1, 3);
    p2.set_index(4);
    println!("Hello, world! {:?}", p > p2);
}

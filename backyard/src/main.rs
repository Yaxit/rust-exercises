use crate::garden::vegetables::Carrot;

pub mod garden;

fn main() {
    let plant = Carrot {};
    println!("The plant is a {:?}", plant);
}

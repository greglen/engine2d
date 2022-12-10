use crate::engine2d::{maths::Vect2D, rope};

pub mod engine2d;

fn main() {
    println!("Hello, world!");
    let r = rope::create_rope(&Vect2D::new(1.0, 1.0), &Vect2D::new(2.0, 2.0), 2);
    print!("{:?}", r);
}

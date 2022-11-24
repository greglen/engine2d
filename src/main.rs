use crate::engine2d::rope;

pub mod engine2d;

fn main() {
    println!("Hello, world!");
    let r = rope::create_rope(1);
    print!("{:?}\n", r);
}

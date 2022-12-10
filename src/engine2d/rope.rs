use std::ops::Add;

use crate::engine2d::maths::Vect2D;

#[derive(Debug)]
pub struct Rope {
    pub nodes: Vec<Node>,
}

// Create a Rope with [segments] number of points that form the Rope
pub fn create_rope(start: &Vect2D, end: &Vect2D, segments: u32) -> Rope {
    let mut nodes: Vec<Node> = Vec::new();

    for i in 0..(segments + 1) {
        nodes.push(Node {
            pos: start.add(end.minus(*start).mult((i as f64) * 1.0 / segments as f64)),
            previous_pos: None,
            mass: 0.0,
        });
    }
    Rope { nodes }
}

impl Rope {
    pub fn update(&self) -> &Self {
        for mut node in self.nodes.clone() {
            let tmp = node.pos;
            node.previous_pos = Some(tmp);
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub pos: Vect2D,
    pub previous_pos: Option<Vect2D>,
    pub mass: f64,
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::engine2d::maths::Vect2D;

    use super::create_rope;

    #[test]
    fn create_rope_test() {
        let start = Vect2D { x: 1.0, y: 1.0 };
        let end = start.add(start);
        let r = create_rope(&start, &end, 40);
        assert_eq!(r.nodes.len(), 41);
        assert_eq!(r.nodes[0].pos, start);
        assert_eq!(r.nodes.last().expect("Should have a last node").pos, end);
    }
}

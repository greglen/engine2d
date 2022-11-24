use crate::engine2d::maths::Point;

#[derive(Debug)]
pub struct Rope {
    pub nodes: Vec<Node>,
}

pub fn create_rope(elements: i32) -> Rope {
    let mut nodes: Vec<Node> = Vec::new();

    for _i in 0..elements {
        nodes.push(Node {
            pos: Point { x: 0.0, y: 0.0 },
            previous_pos: Point { x: 0.0, y: 0.0 },
            mass: 0.0,
        });
    }
    Rope { nodes: nodes }
}

impl Rope {
    pub fn update(&self) -> &Self {
        for mut node in self.nodes.clone() {
            let tmp = node.pos;
            node.previous_pos = tmp;
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub pos: Point,
    pub previous_pos: Point,
    pub mass: f64,
}

#[cfg(test)]
mod tests {
    use super::create_rope;

    #[test]
    fn create_rope_test() {
        let r = create_rope(4);
        assert_eq!(r.nodes.len(), 4);
    }
}

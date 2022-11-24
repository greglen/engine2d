#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn add(&mut self, p: Point) -> Self {
        self.x += p.x;
        self.y += p.y;
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn add() {
        let mut p1 = Point { x: 4.0, y: 7.0 };

        let p2 = Point { x: 2.0, y: 2.0 };

        p1.add(p2);

        assert_eq!(p1.x, 6.0);
        assert_eq!(p1.y, 9.0);
    }
}

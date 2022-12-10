use std::ops::Add;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Vect2D {
    pub x: f64,
    pub y: f64,
}

impl Add for Vect2D {
    type Output = Self;

    fn add(self, p: Self) -> Vect2D {
        Vect2D {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl Vect2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vect2D { x, y }
    }

    pub fn minus(self, p: Vect2D) -> Vect2D {
        Vect2D {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }

    pub fn mult(self, t: f64) -> Vect2D {
        Vect2D {
            x: self.x * t,
            y: self.y * t,
        }
    }

    pub fn length_square(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::Vect2D;

    #[test]
    fn add() {
        let p1 = Vect2D { x: 4.0, y: 7.0 };

        let p2 = Vect2D { x: 2.0, y: 2.0 };

        let p = p1.add(p2);

        assert_eq!(p.x, 6.0);
        assert_eq!(p.y, 9.0);
    }
    #[test]
    fn minus() {
        let p1 = Vect2D { x: 4.0, y: 7.0 };

        let p2 = Vect2D { x: 2.0, y: 2.0 };

        let p = p1.minus(p2);

        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 5.0);
    }

    #[test]
    fn length() {
        let p1 = Vect2D { x: 4.0, y: 3.0 };

        let length = p1.length();

        assert_eq!(length, 5.0);
    }

    #[test]
    fn length_square() {
        let p1 = Vect2D { x: 4.0, y: 3.0 };

        let length = p1.length_square();

        assert_eq!(length, 25.0);
    }
}

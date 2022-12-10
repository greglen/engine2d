use std::ops::{Add, Mul, Sub};

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
impl Sub for Vect2D {
    type Output = Self;

    fn sub(self, p: Self) -> Vect2D {
        Vect2D {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}
impl Mul<f64> for Vect2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vect2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Vect2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vect2D { x, y }
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
    use std::ops::{Add, Mul, Sub};

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
    fn sub() {
        let p1 = Vect2D { x: 4.0, y: 7.0 };

        let p2 = Vect2D { x: 2.0, y: 2.0 };

        let p = p1.sub(p2);

        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 5.0);
    }

    #[test]
    fn mul() {
        let p = Vect2D { x: 4.0, y: 7.0 };

        let p = p.mul(2.0);

        assert_eq!(p.x, 8.0);
        assert_eq!(p.y, 14.0);
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

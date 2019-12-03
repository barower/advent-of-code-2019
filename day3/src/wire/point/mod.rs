#[derive(PartialEq, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
    pub fn x(&self) -> i64 {
        self.x
    }
    pub fn y(&self) -> i64 {
        self.y
    }
}

pub fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p2.x - p1.x).abs() + (p2.y - p1.x).abs()
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(0, 0);

        let p2 = Point::new(3, 3);
        assert_eq!(6, manhattan_distance(&p1, &p2));

        let p2 = Point::new(-3, 3);
        assert_eq!(6, manhattan_distance(&p1, &p2));

        let p2 = Point::new(-3, -3);
        assert_eq!(6, manhattan_distance(&p1, &p2));
    }

}

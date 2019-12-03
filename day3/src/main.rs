#[allow(dead_code)]
mod crossedwires {
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
    }

    pub fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
        (p2.x - p1.x).abs() + (p2.y - p1.x).abs()
    }

    pub struct Wire {
        points: Vec<Point>,
    }

    impl Wire {
        pub fn new() -> Wire {
            let mut wire = Wire { points: Vec::new() };
            wire.points.push(Point::new(0, 0));
            wire
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let p1 = crossedwires::Point::new(0, 0);

        let p2 = crossedwires::Point::new(3, 3);
        assert_eq!(6, crossedwires::manhattan_distance(&p1, &p2));

        let p2 = crossedwires::Point::new(-3, 3);
        assert_eq!(6, crossedwires::manhattan_distance(&p1, &p2));

        let p2 = crossedwires::Point::new(-3, -3);
        assert_eq!(6, crossedwires::manhattan_distance(&p1, &p2));
    }   

}
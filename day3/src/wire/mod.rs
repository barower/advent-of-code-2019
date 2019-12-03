mod point;

pub struct Wire {
    points: Vec<point::Point>,
}

impl Wire {
    pub fn new(path: String) -> Wire {

        let mut wire = Wire { points: Vec::new() };
        wire.points.push(point::Point::new(0, 0));

        for movement in path.split(',') {
            wire.points.push(Wire::translate_movement(movement, wire.points.last().unwrap()));
        }

        wire
    }

    fn translate_movement(movement: &str, prev: &point::Point) -> point::Point {
        let mut str_it = movement.chars();

        match str_it.next() {
            Some('U') => {
                point::Point::new(prev.x(), prev.y() + str_it.as_str().parse::<i64>().unwrap())
            },
            Some('D') => {
                point::Point::new(prev.x(), prev.y() - str_it.as_str().parse::<i64>().unwrap())
            },
            Some('L') => {
                point::Point::new(prev.x() - str_it.as_str().parse::<i64>().unwrap(), prev.y() )
            },
            Some('R') => {
                point::Point::new(prev.x() + str_it.as_str().parse::<i64>().unwrap(), prev.y() )
            },
            x => {
                let retstr = format!("Wrong direction: {:?}", x);
                panic!(retstr);
            },
        }
    }

    fn get_last_point(self) -> point::Point {
        self.points.last().cloned().unwrap()
    }
}

#[cfg(test)]
mod wire_tests {
    use super::*;

    #[test]
    fn test_translate_movement() {
        let point = point::Point::new(0, 0);
        assert_eq!(Wire::translate_movement("U217", &point), point::Point::new(0, 217));

        let point = point::Point::new(0, 0);
        let new_point = Wire::translate_movement("D21", &point);
        let new_point = Wire::translate_movement("U21", &new_point);
        assert_eq!(new_point, point::Point::new(0, 0));

        let point = point::Point::new(0, 0);
        let new_point = Wire::translate_movement("D37", &point);
        let new_point = Wire::translate_movement("R21", &new_point);
        assert_eq!(new_point, point::Point::new(21, -37));
    }

    #[test]
    #[should_panic]
    fn test_translate_movement_panic_0() {
        let point = point::Point::new(0, 0);
        assert_eq!(Wire::translate_movement("", &point), point::Point::new(0, 217));
    }

    #[test]
    #[should_panic]
    fn test_translate_movement_panic_1() {
        let point = point::Point::new(0, 0);
        assert_eq!(Wire::translate_movement("Z32", &point), point::Point::new(0, 217));
    }

    #[test]
    #[should_panic]
    fn test_translate_movement_panic_2() {
        let point = point::Point::new(0, 0);
        assert_eq!(Wire::translate_movement("U", &point), point::Point::new(0, 217));
    }

    #[test]
    #[should_panic]
    fn test_translate_movement_panic_3() {
        let point = point::Point::new(0, 0);
        assert_eq!(Wire::translate_movement("U-1", &point), point::Point::new(0, 217));
    }

    #[test]
    fn test_new() {
        let w = Wire::new(String::from("R8,U5,L5,D3"));
        let desired_point = point::Point::new(3, 2);
        assert_eq!(w.get_last_point(), desired_point);

        let w = Wire::new(String::from("U7,R6,D4,L4"));
        let desired_point = point::Point::new(2, 3);
        assert_eq!(w.get_last_point(), desired_point);
    }

}
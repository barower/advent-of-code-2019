mod point;

pub struct Wire {
    points: Vec<point::Point>,
}

impl Wire {
    pub fn new() -> Wire {
        let mut wire = Wire { points: Vec::new() };
        wire.points.push(point::Point::new(0, 0));
        wire
    }
}

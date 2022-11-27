use std::cmp::Ordering;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HPoint {
    x: u32,
    y: u32,
    i: u64,
}

impl HPoint {
    pub fn new(x: u32, y: u32) -> HPoint {
        HPoint { x: x, y: y, i: 0 }
    }

    pub fn set_index(&mut self, i: u64) {
        self.i = i;
    }
}

impl PartialOrd for HPoint {
    fn partial_cmp(&self, other: &HPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HPoint {
    fn cmp(&self, other: &HPoint) -> Ordering {
        self.i.cmp(&other.i)
    }
}

impl ops::Add<HPoint> for HPoint {
    type Output = HPoint;

    fn add(self, other: HPoint) -> HPoint {
        HPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            i: self.i,
        }
    }
}

impl ops::Add<u32> for HPoint {
    type Output = HPoint;

    fn add(self, other: u32) -> HPoint {
        HPoint {
            x: self.x + other,
            y: self.y + other,
            i: self.i,
        }
    }
}

impl ops::Sub<HPoint> for HPoint {
    type Output = HPoint;

    fn sub(self, other: HPoint) -> HPoint {
        HPoint {
            x: self.x - other.x,
            y: self.y - other.y,
            i: self.i,
        }
    }
}

impl ops::Sub<u32> for HPoint {
    type Output = HPoint;

    fn sub(self, other: u32) -> HPoint {
        HPoint {
            x: self.x - other,
            y: self.y - other,
            i: self.i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpoint_ord() {
        let mut p = HPoint::new(1, 2);
        p.set_index(3);
        let mut p2 = HPoint::new(1, 3);
        p2.set_index(4);
        assert!(p < p2);
    }

    #[test]
    fn test_hpoint_derives() {
        let mut p = HPoint::new(1, 2);
        p.set_index(3);
        let mut p2 = HPoint::new(1, 2);
        p2.set_index(3);
        assert_eq!(p, p);
        assert_eq!(p, p2);
        assert_eq!(p2, p);

        let mut p3 = HPoint::new(1, 2);
        p3.set_index(4);
        assert_ne!(p, p3);
        let p4 = HPoint::from(p3);
        assert_eq!(dbg!(p3), dbg!(p4));

        let p5 = p3.clone();
        assert_eq!(p3, p5);
    }

    #[test]
    fn test_hpoint_add() {
        let mut p = HPoint::new(1, 2);
        p.set_index(3);
        let mut p2 = HPoint::new(1, 3);
        p2.set_index(4);
        let p3 = p + p2;
        assert_eq!(p3.x, 2);
        assert_eq!(p3.y, 5);
        assert_eq!(p3.i, 3);

        let p4 = p + 1;
        assert_eq!(p4.x, 2);
        assert_eq!(p4.y, 3);
        assert_eq!(p4.i, 3);
    }

    #[test]
    fn test_hpoint_sub() {
        let mut p = HPoint::new(1, 2);
        p.set_index(3);
        let mut p2 = HPoint::new(1, 3);
        p2.set_index(4);
        let p3 = p2 - p;
        assert_eq!(p3.x, 0);
        assert_eq!(p3.y, 1);
        assert_eq!(p3.i, 4);

        let p4 = p2 - 1;
        assert_eq!(p4.x, 0);
        assert_eq!(p4.y, 2);
        assert_eq!(p4.i, 4);
    }
}

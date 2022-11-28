use crate::point::HPoint;

use std::mem::swap;

/// Orientation
pub enum Orientation {
    A, // ↑
    B, // ←
    C, // ↓
    D, // →
}

struct QuasiSquare {
    orientation: Orientation,
    origin: HPoint,
    n: usize,
    m: usize,
}

pub enum HCType {
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
    H10,
    H11,
    H12,
    H13,
    H14,
    H15,
    H16,
    H17,
    H18,
    H19,
    H20,
    H21,
    H22,
    H23,
    H24,
    H25,
    H26,
    H27,
    H28,
    H29,
    H30,
    H31,
    H32,
    H33,
    H34,
    H35,
    H36,
    H37,
    H38,
    H39,
}

pub struct HilbertCurve {
    hc_type: HCType,
    curve: Vec<HPoint>,
    mean_diff: f64,
    qs: QuasiSquare,
}

impl QuasiSquare {
    fn build_curve(&mut self, coordinates: &mut Vec<HPoint>, mut index: usize) {
        let mut qsv: Vec<QuasiSquare> = Vec::new();

        match (self.n, self.m) {
            (1, 1) => {
                coordinates[index] = self.origin;
                coordinates[index].set_index(index);
            }
            (1, 2) => match self.orientation {
                Orientation::A | Orientation::B => {
                    coordinates[index] = self.origin;
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index + 1].set_index(index + 1);
                }
                Orientation::C | Orientation::D => {
                    coordinates[index + 1] = self.origin;
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index].set_index(index);
                }
            },
            (2, 1) => match self.orientation {
                Orientation::A | Orientation::B => {
                    coordinates[index] = self.origin;
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index + 1].set_index(index + 1);
                }
                Orientation::C | Orientation::D => {
                    coordinates[index + 1] = self.origin;
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index].set_index(index);
                }
            },
            (2, 2) => match self.orientation {
                Orientation::A => {
                    coordinates[index] = self.origin;
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index + 2] = HPoint::new(self.origin.x + 1, self.origin.y + 1);
                    coordinates[index + 2].set_index(index + 2);
                    coordinates[index + 3] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index + 3].set_index(index + 3);
                }
                Orientation::B => {
                    coordinates[index] = self.origin;
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index + 2] = HPoint::new(self.origin.x + 1, self.origin.y + 1);
                    coordinates[index + 2].set_index(index + 2);
                    coordinates[index + 3] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index + 3].set_index(index + 3);
                }
                Orientation::C => {
                    coordinates[index] = HPoint::new(self.origin.x + 1, self.origin.y + 1);
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index + 2] = self.origin;
                    coordinates[index + 2].set_index(index + 2);
                    coordinates[index + 3] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index + 3].set_index(index + 3);
                }
                Orientation::D => {
                    coordinates[index] = HPoint::new(self.origin.x + 1, self.origin.y + 1);
                    coordinates[index].set_index(index);
                    coordinates[index + 1] = HPoint::new(self.origin.x, self.origin.y + 1);
                    coordinates[index + 1].set_index(index + 1);
                    coordinates[index + 2] = self.origin;
                    coordinates[index + 2].set_index(index + 2);
                    coordinates[index + 3] = HPoint::new(self.origin.x + 1, self.origin.y);
                    coordinates[index + 3].set_index(index + 3);
                }
            },
            //QuasiSquare isn't a primitive so need to keep Partitioning
            (_, _) => {
                self.partition(&mut qsv);

                for _i in 0..4 {
                    let mut qs = qsv.pop().unwrap();
                    qs.build_curve(coordinates, index);
                    index += qs.n * qs.m;
                }
            }
        }
    }

    /// Perform a QuasiSquare partition.
    ///
    /// A QuasiSquare is partitioned into 4 inner quasisquares.
    /// According to the orientation, inners quasisquares are
    /// orientated differently.
    ///
    /// # Orientation Partitioning:
    ///
    /// A -> [D, A, A, B]
    /// B -> [C, B, B, A]
    /// C -> [B, C, C, D]
    /// D -> [A, D, D, C]
    ///
    fn partition(&mut self, partitions: &mut Vec<QuasiSquare>) {
        let mut n1: usize;
        let mut n2: usize;
        let mut m1: usize;
        let mut m2: usize;

        // Even partition of the quasisquare
        n1 = self.n / 2;
        n2 = self.n - n1;
        m1 = self.m / 2;
        m2 = self.m - m1;

        match self.orientation {
            Orientation::A | Orientation::B => {
                if (n1 % 2 == 1) {
                    swap(&mut n1, &mut n2);
                }
                if (m1 % 2 == 1) {
                    swap(&mut m1, &mut m2);
                }
            }
            Orientation::C | Orientation::D => {
                if (n2 % 2 == 1) {
                    swap(&mut n1, &mut n2);
                }
                if (m2 % 2 == 1) {
                    swap(&mut m1, &mut m2);
                }
            }
        }

        match self.orientation {
            Orientation::A => {
                partitions.push(QuasiSquare {
                    orientation: Orientation::D,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y),
                    n: n1,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::A,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y + n1),
                    n: n2,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::A,
                    origin: HPoint::new(self.origin.x, self.origin.y + n1),
                    n: n2,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::B,
                    origin: self.origin,
                    n: n1,
                    m: m1,
                });
            }
            Orientation::B => {
                partitions.push(QuasiSquare {
                    orientation: Orientation::C,
                    origin: HPoint::new(self.origin.x, self.origin.y + n1),
                    n: n2,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::B,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y + n1),
                    n: n2,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::B,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y),
                    n: n1,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::A,
                    origin: self.origin,
                    n: n1,
                    m: m1,
                });
            }
            Orientation::C => {
                partitions.push(QuasiSquare {
                    orientation: Orientation::B,
                    origin: HPoint::new(self.origin.x, self.origin.y + n1),
                    n: n2,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::C,
                    origin: self.origin,
                    n: n1,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::C,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y),
                    n: n1,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::D,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y + n1),
                    n: n2,
                    m: m2,
                });
            }
            Orientation::D => {
                partitions.push(QuasiSquare {
                    orientation: Orientation::A,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y),
                    n: n1,
                    m: m2,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::D,
                    origin: self.origin,
                    n: n1,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::D,
                    origin: HPoint::new(self.origin.x, self.origin.y + n1),
                    n: n2,
                    m: m1,
                });

                partitions.push(QuasiSquare {
                    orientation: Orientation::C,
                    origin: HPoint::new(self.origin.x + m1, self.origin.y + n1),
                    n: n2,
                    m: m2,
                });
            }
        };
    }
}

impl HilbertCurve {
    fn new(
        width: usize,
        height: usize,
        hc_type: HCType,
        origin: HPoint,
        orientation: Orientation,
    ) -> HilbertCurve {
        let mut hc = HilbertCurve {
            hc_type,
            curve: vec![],
            mean_diff: 0.0,
            qs: QuasiSquare {
                orientation,
                origin,
                n: width,
                m: height,
            },
        };

        hc.build();
        return hc;
    }

    fn build(&mut self) {
        match self.hc_type {
            HCType::H0 => self.build_h0(),
            _ => return,
        }
    }

    fn build_h0(&mut self) {
        self.curve = Vec::with_capacity(self.qs.n * self.qs.m);
        self.curve.resize(self.qs.n * self.qs.m, HPoint::new(0, 0));
        self.qs.build_curve(&mut self.curve, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let hc = HilbertCurve::new(4, 4, HCType::H0, HPoint::new(0, 0), Orientation::A);

        assert_eq!(
            hc.curve,
            vec![
                HPoint::new_with_index(0, 0, 0),
                HPoint::new_with_index(1, 0, 1),
                HPoint::new_with_index(1, 1, 2),
                HPoint::new_with_index(0, 1, 3),
                HPoint::new_with_index(0, 2, 4),
                HPoint::new_with_index(0, 3, 5),
                HPoint::new_with_index(1, 3, 6),
                HPoint::new_with_index(1, 2, 7),
                HPoint::new_with_index(2, 2, 8),
                HPoint::new_with_index(2, 3, 9),
                HPoint::new_with_index(3, 3, 10),
                HPoint::new_with_index(3, 2, 11),
                HPoint::new_with_index(3, 1, 12),
                HPoint::new_with_index(2, 1, 13),
                HPoint::new_with_index(2, 0, 14),
                HPoint::new_with_index(3, 0, 15)
            ]
        );
    }
}

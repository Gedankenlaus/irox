use std::{
    ops::{Add, BitXor, Mul}
};

use container::List;

#[derive(Default)]
pub struct Polynomial {
    pub coefficient: f64,
    pub order: i32,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            coefficient: 1.0,
            order: 1,
        }
    }

    pub fn to(&self, order: i32) -> Self {
        Polynomial {
            coefficient: self.coefficient,
            order: self.order * order,
        }
    }
}

impl Add for Polynomial {
    type Output = container::List<Polynomial>;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.order == self.order {
            return container::List::from_array([Polynomial {
                coefficient: self.coefficient + rhs.coefficient,
                order: self.order,
            }]);
        }

        if self.order > rhs.order {
            return container::List::from_array([
                Polynomial {
                    coefficient: rhs.coefficient,
                    order: rhs.order,
                },
                Polynomial {
                    coefficient: self.coefficient,
                    order: self.order,
                },
            ]);
        }

        container::List::from_array([
            Polynomial {
                coefficient: self.coefficient,
                order: self.order,
            },
            Polynomial {
                coefficient: rhs.coefficient,
                order: rhs.order,
            },
        ])
    }
}

impl Add<Polynomial> for List<Polynomial> {
    type Output = List<Polynomial>;

    fn add(self, rhs: Polynomial) -> Self::Output {
        let mut contains_same_order = false;
        let mut resulting_series = List::new();
        for poly_element in self.iter() {
            if poly_element.order == rhs.order {
                resulting_series.append(Polynomial {
                    coefficient: poly_element.coefficient * rhs.coefficient,
                    order: poly_element.order,
                });

                contains_same_order = true;
            } else {
                resulting_series.append(Polynomial {
                    coefficient: poly_element.coefficient,
                    order: poly_element.order,
                });
            }
        }

        if !contains_same_order {
            resulting_series.append(rhs);
        }

        resulting_series
    }
}

impl Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            coefficient: self * rhs.coefficient,
            order: rhs.order,
        }
    }
}

impl BitXor<i32> for Polynomial {
    type Output = Polynomial;

    fn bitxor(self, order: i32) -> Self::Output {
        self.to(order)
    }
}
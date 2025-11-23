// this will be the testing ground, use this as a polynomial adder

use std::{ops::{Add, BitXor, Mul}};


#[derive(Default)]
struct Polynomial
{
    coefficient: f64,
    order: i32
}

impl Polynomial {
    fn new() -> Self
    {
        Polynomial { coefficient: 1.0, order: 1 }
    }

    fn to(&self, order: i32) -> Self
    {
        Polynomial { coefficient: self.coefficient, order: self.order * order }
    }
}

impl Add for Polynomial {
    type Output = container::List<Polynomial>;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.order == self.order {
            return container::List::from_array([
                Polynomial{
                    coefficient: self.coefficient + rhs.coefficient,
                    order: self.order
                }
            ]);
        }

        if self.order > rhs.order{
            return  container::List::from_array([
                Polynomial{
                    coefficient: rhs.coefficient,
                    order: rhs.order
                },
                Polynomial{
                    coefficient: self.coefficient,
                    order: self.order
                }
            ]);
        }

        container::List::from_array([
                Polynomial{
                    coefficient: self.coefficient,
                    order: self.order
                },
                Polynomial{
                    coefficient: rhs.coefficient,
                    order: rhs.order
                }
        ])
    }
}

impl Mul<Polynomial> for f64
{
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        Polynomial{
            coefficient: self * rhs.coefficient,
            order: rhs.order,
        }
    }
}

impl BitXor<i32> for Polynomial{
    type Output = Polynomial;

    fn bitxor(self, order: i32) -> Self::Output {
        self.to(order)
    }
}

fn poly_variable() -> Polynomial
{
    Polynomial::new()
}


fn main() {
    let x = poly_variable();
    let mut poly_series = 3.0 * x.to(2) + 1.0 * x.to(3);
    let mut poly_str_vec: Vec<String> = Vec::new();

    for poly in poly_series.iter()
    {
        let c = (*poly).coefficient;
        let order = (*poly).order;

        if c == 1.0 {
            poly_str_vec.push(format!("x^{order}"));
        }
        else {
            poly_str_vec.push(format!("{c}x^{order}"));
        }
    }

    let joined_poly_str = poly_str_vec.join(" + ");

    println!("Current Series: {joined_poly_str}");
}

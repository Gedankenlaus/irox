// this will be the testing ground, use List for a polynomial adder

pub mod poly;

use poly::Polynomial;

fn main() {
    let x = Polynomial::new();
    let poly_series = 3.0 * x.to(2) + 1.0 * x.to(3) + 3.0 * x.to(4);

    // use map for formatting the poly string
    let poly_as_str: String = poly_series
        .iter()
        .map(|x| {
            let coeff = x.coefficient;
            let order = x.order;
            if coeff == 1.0 {
                return format!("x^{order}");
            }

            format!("{coeff}x^{order}")
        })
        .collect::<Vec<_>>().join(" + ");

    println!("Current Series: {poly_as_str}");
}

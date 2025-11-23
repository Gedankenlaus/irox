// this will be the testing ground, use List for a polynomial adder

pub mod poly;

use poly::Polynomial;

fn main() {
    let x = Polynomial::new();
    let poly_series = 3.0 * x.to(2) + 1.0 * x.to(3) + 3.0 * x.to(4);

    // use mapping for the formatting the poly string
    let poly_str_vec: Vec<String> = poly_series
        .iter()
        .map(|x| {
            let coeff = x.coefficient;
            let order = x.order;
            if coeff == 1.0 {
                return format!("x^{order}");
            }

            format!("{coeff}x^{order}")
        })
        .collect();

    let joined_poly_str = poly_str_vec.join(" + ");

    println!("Current Series: {joined_poly_str}");
}

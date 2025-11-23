// this will be the testing ground, use List for a polynomial adder
pub mod poly;

use poly::Polynomial;

fn main() {
    let x = Polynomial::new();
    let poly_series = 3.0 * x.to(2) + 1.0 * x.to(3) + 3.0 * x.to(4);
    let mut poly_str_vec: Vec<String> = Vec::new();

    for poly in poly_series.iter() {
        let c = (*poly).coefficient;
        let order = (*poly).order;

        if c == 1.0 {
            poly_str_vec.push(format!("x^{order}"));
        } else {
            poly_str_vec.push(format!("{c}x^{order}"));
        }
    }

    let joined_poly_str = poly_str_vec.join(" + ");

    println!("Current Series: {joined_poly_str}");
}

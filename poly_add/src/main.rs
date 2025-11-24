// this will be the testing ground, use List for a polynomial adder

pub mod poly;

use poly::Polynomial;


fn main() -> Result<(), ()> {
    let x = Polynomial::new();
    let poly_series = 3.0 * x.to(2) + 1.0 * x.to(3) + 3.0 * x.to(4);

    //let ref_val = poly_series.get(1).ok_or(())?;
    println!("Order at index 1: {}", poly_series[1].order);

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
        .fold("".to_string(), |acc, e| format!("{acc} + {e}"));

    println!("Current Series: {poly_as_str}");

    Ok(())
}

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = if c == 0 {
        f64::NEG_INFINITY
    } else {
        (c.abs() as f64).ln()
    };
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| (n as f64).exp().to_string())
        .collect();
    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&n| if n == 0 {
            f64::NEG_INFINITY
        } else {
            (n.abs() as f64).ln()
        })
        .collect();
    (b, ln_values)
}
const FREEZE_F: f64 = 32.0;

fn f_to_c(f: f64) -> f64 {
    let c: f64 = (f - FREEZE_F) * (5.0 / 9.0);
    return c;
}

fn c_to_f(c: f64) -> f64 {
    let f: f64 = (c * (9.0 / 5.0)) + FREEZE_F;
    return f;
}

fn main() {
    let mut temp_f: f64 = 32.0;

    let temp_c = f_to_c(temp_f);
    println!("{:.2} F = {:.2} C", temp_f, temp_c);

    for _ in 1..=5 {
        temp_f += 1.0;
        let c = f_to_c(temp_f);
        println!("{:.2} F = {:.2} C", temp_f, c);
    }
}

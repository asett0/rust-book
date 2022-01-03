fn main() {
    println!("{}°F is {}°C", 50, far_to_cel(50.));
    println!("{}°C is {}°F", 50, cel_to_far(50.));
}

fn far_to_cel(t_far: f64) -> f64 {
    (t_far - 32.) / 1.8
}

fn cel_to_far(t_cel: f64) -> f64 {
    1.8 * t_cel + 32.
}

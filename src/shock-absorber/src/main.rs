// EVERYTHING USES SI UNITS!!! the STANDARD! so kg, m, s, A, cd, mol, K and their derived ones

const EARTH_GRAVITY: f64 = 9.80665;

fn calculate_score(
    input_accel: f64,
    piston_mass: f64,
    piston_area: f64,
    outer_tube_area: f64,
    area_valve_input: f64,
) -> f64 {
    input_accel / (input_accel - ((piston_mass * piston_area * outer_tube_area) / area_valve_input))
}

fn main() {
    let mut best_score = 250.0;
    let mut values: Vec<f64> = Vec::new();
    let mut score: f64 = 0.0;
    while score < best_score {
        values.push(100.0 * EARTH_GRAVITY);
        for _ in 0..4 {
            values.push(fastrand::f64() * (100.0 - 1.0)); // between 100 and 1
        }
        score = calculate_score(values[0], values[1], values[2], values[3], values[4]);
        println!("score: {}", score);
        println!("values: {:?}", values);
        values.clear();
    }
}

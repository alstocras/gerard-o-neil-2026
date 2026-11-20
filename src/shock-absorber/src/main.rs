// EVERYTHING USES SI UNITS!!! the STANDARD! so kg, m, s, A, cd, mol, K and their derived ones
fn calculate_score(
    input_accel: f64,
    piston_mass: f64,
    piston_accel: f64,
    outer_tube_area: f64,
    area_valve_input: f64,
) -> f64 {
    input_accel
        / (input_accel - ((piston_mass * piston_accel * outer_tube_area) / area_valve_input))
}

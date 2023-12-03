use rayon::prelude::*;

pub fn compute_residual(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    // v1.iter().zip(v2.iter()).map(|(x, y)| (x-y).abs()).max().unwrap_or(0.0)
    v1.par_iter().zip(v2.par_iter())
        .map(|(x, y)| (x-y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap_or(0.0)
}

#[test]
fn test_compute_residual() {
    let vx = vec![1.0, 2.0, 3.0];
    let vy = vec![3.0, 2.0, 1.0];
    let r = compute_residual(&vx, &vy);
    assert_eq!(r, 2.0);
}
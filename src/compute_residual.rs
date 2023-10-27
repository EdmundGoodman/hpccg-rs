pub fn compute_residual_direct(n: usize, v1: &[f64], v2: &[f64], residual: &mut f64) -> u32 {
    let mut local_residual: f64 = 0.0;

    for i in 0..n {
        let diff = (v1[i] - v2[i]).abs();
        if diff > local_residual {
            local_residual = diff;
        }
    }

    *residual = local_residual;
    return 0;
}

pub fn compute_residual_idiomatic(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    // v1.iter().zip(v2.iter()).map(|(x, y)| (x-y).abs()).max().unwrap_or(0.0)
    v1.iter().zip(v2.iter())
        .map(|(x, y)| (x-y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap_or(0.0)
}

#[test]
fn test_compute_residual_direct() {
    let vx: [f64; 3] = [1.0,2.0,3.0];
    let vy: [f64; 3] = [3.0,2.0,1.0];
    let mut r: f64 = 0.0;
    compute_residual_direct(vx.len(), &vx, &vy, &mut r);
    assert_eq!(r, 2.0);
}

#[test]
fn test_compute_residual_idiomatic() {
    let vx = vec![1.0, 2.0, 3.0];
    let vy = vec![3.0, 2.0, 1.0];
    let r = compute_residual_idiomatic(&vx, &vy);
    assert_eq!(r, 2.0);
}
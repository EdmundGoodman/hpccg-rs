pub fn ddot_direct(n: usize, x: &[f64], y: &[f64], result: &mut f64) -> u32 {
    let mut local_result: f64 = 0.0;
    if std::ptr::eq(x, y) {
        for i in 0..n {
            local_result += x[i]*x[i];
        }
    } else {
        for i in 0..n {
            local_result += x[i]*y[i];
        }
    }
    *result = local_result;
    return 0;
}

pub fn ddot_idiomatic(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    if std::ptr::eq(x, y) {
        x.iter().map(|x| x * x).sum()
    } else {
        x.iter().zip(y.iter())
            .map(|(x, y)| x * y).sum()
    }
}

#[test]
fn test_ddot_direct() {
    let vx: [f64; 3] = [1.0,2.0,3.0];
    let vy: [f64; 3] = [3.0,2.0,1.0];
    let mut r1: f64 = 0.0;
    ddot_direct(vx.len(), &vx, &vy, &mut r1);
    assert_eq!(r1, 10.0);
    r1 = 0.0;
    ddot_direct(vx.len(), &vx, &vx, &mut r1);
    assert_eq!(r1, 14.0);
}

#[test]
fn test_ddot_idiomatic() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = ddot_idiomatic(&vx, &vy);
    assert_eq!(r, 10.0);
    let r = ddot_idiomatic(&vx, &vx);
    assert_eq!(r, 14.0);
}
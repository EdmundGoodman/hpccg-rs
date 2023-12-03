pub fn waxpby_direct(n: usize, alpha: f64, x: &[f64], beta: f64, y: &[f64], w: &mut [f64]) -> u32 {
    if alpha == 1.0 {
        for i in 0..n {
            w[i] = x[i] + beta * y[i];
        }
    } else if beta == 1.0 {
        for i in 0..n {
            w[i] = alpha * x[i] + y[i];
        }
    } else {
        for i in 0..n {
            w[i] = alpha * x[i] + beta * y[i];
        }
    }
    return 0;
}

pub fn waxpby_idiomatic(alpha: f64, x: &Vec<f64>, beta: f64, y: &Vec<f64>) -> Vec<f64> {
    if alpha == 1.0 {
        x.iter().zip(y.iter())
            .map(|(x, y)| x + beta * y)
            .collect()
    } else if beta == 1.0 {
        x.iter().zip(y.iter())
            .map(|(x, y)| alpha * x + y)
            .collect()
    } else {
        x.iter().zip(y.iter())
            .map(|(x, y)| alpha * x + beta * y)
            .collect()
    }
}


#[test]
fn test_waxpby_direct() {
    let vx: [f64; 3] = [1.0,2.0,3.0];
    let vy: [f64; 3] = [3.0,2.0,1.0];
    let alpha = 4.0;
    let beta = 5.0;
    let mut w: [f64; 3] = [0.0; 3];
    waxpby_direct(vx.len(), alpha, &vx, beta, &vy, &mut w);
    assert_eq!(w, [19.0, 18.0, 17.0]);
    let alpha = 1.0;
    waxpby_direct(vx.len(), alpha, &vx, beta, &vy, &mut w);
    assert_eq!(w, [16.0, 12.0, 8.0]);
}

#[test]
fn test_waxpby_idiomatic() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let alpha = 4.0;
    let beta = 5.0;
    let r = waxpby_idiomatic(alpha, &vx, beta, &vy);
    assert_eq!(r, vec![19.0, 18.0, 17.0]);
    let alpha = 1.0;
    let r = waxpby_idiomatic(alpha, &vx, beta, &vy);
    assert_eq!(r, vec![16.0, 12.0, 8.0]);
}
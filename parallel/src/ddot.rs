use rayon::prelude::*;

pub fn ddot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    if std::ptr::eq(x, y) {
        x.par_iter().map(|x| x * x).sum()
    } else {
        x.par_iter().zip(y.par_iter())
            .map(|(x, y)| x * y).sum()
    }
}

#[test]
fn test_ddot() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = ddot(&vx, &vy);
    assert_eq!(r, 10.0);
    let r = ddot(&vx, &vx);
    assert_eq!(r, 14.0);
}
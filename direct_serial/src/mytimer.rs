use libc::{rusage,RUSAGE_SELF};

pub fn mytimer() -> f64 {
    unsafe {
        let mut ruse: rusage = std::mem::zeroed();
        libc::getrusage(RUSAGE_SELF, &mut ruse);
        let seconds = ruse.ru_utime.tv_sec as f64;
        let micro_seconds = ruse.ru_utime.tv_usec as f64;
        seconds + micro_seconds/1000000.0
    }
}

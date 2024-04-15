use std::mem::MaybeUninit;
use std::time::{SystemTime, UNIX_EPOCH};

/// Alias function to allow switching timers.
pub fn mytimer() -> f64 {
    // wall_mytimer()
    // sysconf_mytimer()
    // getrusage_mytimer()
    getmpi_mytimer()
}

/// A function to get the wall clock time in seconds since the UNIX epoch.
#[allow(dead_code)]
pub fn wall_mytimer() -> f64 {
    let tp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = tp.as_secs() as f64;
    let micro_seconds = tp.subsec_micros() as f64;
    seconds + micro_seconds / 1_000_000.0
}

/// A function to get the system clock time in seconds.
#[allow(dead_code)]
pub fn sysconf_mytimer() -> f64 {
    unsafe {
        let mut ts: MaybeUninit<libc::tms> = MaybeUninit::uninit();
        let clock_tick = libc::sysconf(libc::_SC_CLK_TCK) as f64;
        libc::times(ts.as_mut_ptr());
        (ts.assume_init().tms_utime as f64) / clock_tick
    }
}

/// A function to get the CPU time (user and system) in seconds.v
pub fn getrusage_mytimer() -> f64 {
    unsafe {
        let mut ruse: MaybeUninit<libc::rusage> = MaybeUninit::uninit();
        libc::getrusage(libc::RUSAGE_SELF, ruse.as_mut_ptr());
        let seconds = ruse.assume_init().ru_utime.tv_sec as f64;
        let micro_seconds = ruse.assume_init().ru_utime.tv_usec as f64;
        seconds + micro_seconds / 1_000_000.0
    }
}

/// A function to use MPI bindings to get the wall time in seconds.
#[allow(dead_code)]
pub fn getmpi_mytimer() -> f64 {
    mpi::time()
}

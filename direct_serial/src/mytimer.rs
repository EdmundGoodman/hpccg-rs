use std::time::{SystemTime, UNIX_EPOCH};

/// Alias function to allow switching timers.
pub fn mytimer() -> f64 {
    // wall_mytimer()
    // sysconf_mytimer()
    getrusage_mytimer()
}


/// A function to get the wall clock time in seconds since the UNIX epoch.
///
/// # Return values
///  * The wall clock time in seconds since the UNIX epoch
pub fn wall_mytimer() -> f64 {
    let tp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap();
    ((tp.as_secs()) as f64) + ((tp.subsec_micros()) as f64 / 1_000_000.0)
}

/// A function to get the system clock time in seconds.
///
/// # Return values
///  * The system clock time in seconds
pub fn sysconf_mytimer() -> f64 {
    unsafe {
        // use std::mem::MaybeUninit;
        // let mut ts: MaybeUninit<libc::tms> = MaybeUninit::uninit();
        // let clock_tick = libc::sysconf(libc::_SC_CLK_TCK) as f64;
        // libc::times(ts.as_mut_ptr());
        // (ts.assume_init().tms_utime as f64) / clock_tick;
        let mut ts: libc::tms = std::mem::zeroed();
        let clock_tick = libc::sysconf(libc::_SC_CLK_TCK) as f64;
        libc::times(&mut ts);
        (ts.tms_utime as f64) / clock_tick
    }
}

/// A function to get the CPU time (user and system) in seconds.
///
/// # Return values
///  * The CPU time (user and system) in seconds
pub fn getrusage_mytimer() -> f64 {
    unsafe {
        let mut ruse: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut ruse);
        let seconds = ruse.ru_utime.tv_sec as f64;
        let micro_seconds = ruse.ru_utime.tv_usec as f64;
        seconds + micro_seconds/1000000.0
    }
}

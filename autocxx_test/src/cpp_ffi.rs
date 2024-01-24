use autocxx::prelude::*;

pub use ffi::{ddot, waxpby};

include_cpp! {
    #include "ddot.hpp"
    #include "waxpby.hpp"
    safety!(unsafe_ffi)
    generate!("ddot")
    generate!("waxpby")
}

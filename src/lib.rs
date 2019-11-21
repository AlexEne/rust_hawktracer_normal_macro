#[allow(dead_code)]
#[cfg(feature = "profiling_enabled")]
extern crate rust_hawktracer_sys;
#[cfg(feature = "profiling_enabled")]
pub use rust_hawktracer_sys::*;
#[cfg(feature = "profiling_enabled")]
use std::thread_local;
#[cfg(not(feature = "profiling_enabled"))]
mod dummy_structs;
#[cfg(not(feature = "profiling_enabled"))]
pub use dummy_structs::*;

#[macro_export]
#[cfg(feature = "profiling_enabled")]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        thread_local! {
            static tracepoint_id: u64 = add_cached_mapping(concat!(stringify!($name), "\0").as_ptr() as _);
        };
        
        tracepoint_id.with(|id| {
            ScopedTracepoint::start_trace_id(*id);
        });
        
        let $name = ScopedTracepoint {};
    };
}

#[macro_export]
#[cfg(not(feature = "profiling_enabled"))]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        ()
    };
}


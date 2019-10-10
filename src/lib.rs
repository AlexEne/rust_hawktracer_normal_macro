#[allow(dead_code)]
extern crate rust_hawktracer_sys;

#[macro_export]
#[cfg(feature = "profiling_enabled")]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        let tracepoint_name = concat!(stringify!($name), "\0");
        ScopedTracepoint::start_trace(tracepoint_name.as_ptr() as _);
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
cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod futex;
        pub use futex::Parker;
    } else if #[cfg(windows)] {
        pub use crate::sys::thread_parker::Parker;
    } else {
        mod generic;
        pub use generic::Parker;
    }
}

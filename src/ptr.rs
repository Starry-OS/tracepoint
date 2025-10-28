//! A trait to convert various types to u64 representation.
//! This is useful for passing arguments to tracepoints in a uniform way.
//!

/// A trait to convert various types to u64 representation.
pub trait AsU64 {
    /// Convert the value to u64.
    fn to_u64(self) -> u64;
}

macro_rules! impl_basic {
    ($($t:ty),+) => {
        $(
            impl AsU64 for $t {
                fn to_u64(self) -> u64 {
                    self as u64
                }
            }
        )+
    };
}

impl_basic!(
    u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, bool, char
);

impl<T> AsU64 for &T {
    fn to_u64(self) -> u64 {
        self as *const T as u64
    }
}

impl<T> AsU64 for &mut T {
    fn to_u64(self) -> u64 {
        self as *mut T as u64
    }
}

impl<T> AsU64 for *const T {
    fn to_u64(self) -> u64 {
        self as u64
    }
}
impl<T> AsU64 for *mut T {
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl AsU64 for &str {
    fn to_u64(self) -> u64 {
        self.as_ptr() as u64
    }
}

impl AsU64 for &[u8] {
    fn to_u64(self) -> u64 {
        self.as_ptr() as u64
    }
}

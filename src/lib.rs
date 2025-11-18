//! # TrustMeBro
//!
//! A Rust macro that wraps arbitrary tokens in an `unsafe` block.
//!
//! ## Usage
//!
//! ```rust
//! use trustmebro::trustmebro;
//!
//! let mut x = 5;
//! let ptr = &mut x as *mut i32;
//!
//! trustmebro! {
//!     *ptr = 10;
//! }
//!
//! assert_eq!(x, 10);
//! ```

/// Expand tokens into an `unsafe` block.
/// Use like: `trustmebro! { /* arbitrary tokens */ }`
#[macro_export]
macro_rules! trustmebro {
    ( $($t:tt)* ) => {{
        unsafe { $($t)* }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trustmebro_macro() {
        let mut x = 5;
        let ptr = &mut x as *mut i32;

        trustmebro! {
            *ptr = 10;
        }

        assert_eq!(x, 10);
    }

    #[test]
    fn test_trustmebro_with_return() {
        let result = trustmebro! {
            let x = 42;
            x * 2
        };

        assert_eq!(result, 84);
    }
}

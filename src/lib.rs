//! # Lorust - API Documentation
//!
//! Lorust is the Rust version of **Lodash**, which is a modern Javascript utilty library delivering modularity, performance & extras.
//!
//! ## Usage
//!
//! Depend on `lorust` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lorust = "0.1.0"
//! ```

mod function;
#[cfg(feature = "function")]
pub use function::*;

mod lang;
#[cfg(feature = "lang")]
pub use lang::*;

mod math;
#[cfg(feature = "math")]
pub use math::*;

mod object;
#[cfg(feature = "object")]
pub use object::*;

mod string;
#[cfg(feature = "string")]
pub use string::*;

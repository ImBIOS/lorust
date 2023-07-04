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
//!
//! ## Functions Categorization
//!
//! We follow the flat function structure of **Lodash**, but the functions can be categorized into following areas:
//!
//! | Category     | Description                                                  |
//! |--------------|--------------------------------------------------------------|
//! | `string`     | Utility functions to deal with Strings                       |
//! | `array`      | Utility functions to deal with Arrays (Not yet support)      |
//! | `math`       | Utility functions to deal with Maths (Not yet support)       |
//! | `object`     | Utility functions to deal with Objects (Not yet support)     |
//!

mod math;
#[cfg(feature = "math")]
pub use math::*;

mod object;
#[cfg(feature = "object")]
pub use object::*;

mod string;
#[cfg(feature = "string")]
pub use string::*;

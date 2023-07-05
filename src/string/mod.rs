mod capitalize;
#[cfg(feature = "capitalize")]
pub use capitalize::*;

mod deburr;
#[cfg(feature = "deburr")]
pub use deburr::*;

mod ends_with;
#[cfg(feature = "ends_with")]
pub use ends_with::*;

mod kebab_case;
#[cfg(feature = "kebab_case")]
pub use kebab_case::*;

mod words;
#[cfg(feature = "words")]
pub use words::*;

pub mod geom;
pub mod matrix;

mod skip_nth;
mod traits;

pub use skip_nth::SkipNth;
pub use traits::{Gcd, IteratorExt};

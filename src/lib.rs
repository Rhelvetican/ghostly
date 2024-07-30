//! # Yeah I don't know why am I making this but cool bananas!
//!
//! A part of the Xeonlib project.

#[cfg(test)]
mod test;

pub mod ghost;
pub mod ghost_ref;

pub mod prelude {
    pub use crate::ghost::Ghost;
    pub use crate::ghost_ref::GhostRef;
}

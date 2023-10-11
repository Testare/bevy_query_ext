#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations)]

mod base;
mod extensions;
mod or_const;

/// Prelude module - Contains only the parts of the crate that are useful to consumers
/// Everything in this module can also be imported from the crate directly, but you
/// can import `bevy_query_ext::prelude::*` over `bevy_query_ext::*` to avoid
/// importing our internal modules.
pub mod prelude {
    pub use super::extensions::{
        AsDeref, AsDerefCloned, AsDerefClonedOfClonedOrDefault, AsDerefClonedOrDefault,
        AsDerefCopied, AsDerefCopiedOfClonedOrDefault, AsDerefCopiedOfCopiedOrDefault,
        AsDerefCopiedOrDefault, AsDerefMut, Cloned, ClonedOrDefault, Copied, CopiedOrDefault,
        OrDefault,
    };
    pub use super::or_const::{
        AsDerefOrBool, AsDerefOrChar, AsDerefOrI128, AsDerefOrI16, AsDerefOrI32, AsDerefOrI64,
        AsDerefOrI8, AsDerefOrIsize, AsDerefOrU128, AsDerefOrU16, AsDerefOrU32, AsDerefOrU64,
        AsDerefOrU8, AsDerefOrUsize, OrBool, OrChar, OrI128, OrI16, OrI32, OrI64, OrI8, OrIsize,
        OrU128, OrU16, OrU32, OrU64, OrU8, OrUsize,
    };
}
#[doc(inline)]
pub use self::prelude::*;

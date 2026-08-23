//! Foundational types shared by every Any Code crate.
//!
//! Two invariants live here because everything else is built on them and they are
//! expensive to retrofit (see PRD §77 Event Architecture, §90 Security Model):
//!
//! 1. Every meaningful action becomes an append-only [`Event`].
//! 2. Every piece of data carries a [`Trust`] tag. Untrusted data never becomes
//!    an instruction.

pub mod event;
pub mod trust;

pub use event::{Event, EventScope};
pub use trust::Trust;

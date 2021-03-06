//! BC Bypass
//!
//! This crate expedites the mechanisms through which Rust's borrow checker can
//! be bypassed. It provides simple functions which enable the creation of both
//! mutable and immutable references to a value which has already been borrowed
//! mutably.

pub mod unrestricted;

/// Duplicate Reference as Mutable
///
/// This function returns a mutable reference to the value behind another
/// reference, mutable or immutable.
pub unsafe fn dup_mut<'a, T: ?Sized>(brw: &T) -> &'a mut T {
    let tmp: *const T = brw;
    &mut *(tmp as *mut T)
}

/// Duplicate Reference as Immutable
///
/// This function returns an immutable reference to the value behind another
/// reference, mutable or immutable.
pub unsafe fn dup_imm<'a, T: ?Sized>(brw: &T) -> &'a T {
    let tmp: *const T = brw;
    &*tmp
}


//! Unrestricted Reference Duplications
//!
//! This submodule contains the same functions as its parent module, but
//! they are not marked as unsafe. Using this submodule in production codebases
//! is strongly discouraged.

/// Duplicate Reference as Mutable
///
/// This function creates a mutable reference to the value behind another
/// reference, mutable or immutable.
pub fn dup_mut<'a, T: ?Sized>(brw: &T) -> &'a mut T {
    let tmp: *const T = brw;
    unsafe { &mut *(tmp as *mut T) }
}

/// Duplicate Reference as Immutable
///
/// This function creates an immutable reference to the value behind another
/// reference, mutable or immutable.
pub fn dup_imm<'a, T: ?Sized>(brw: &T) -> &'a T {
    let tmp: *const T = brw;
    unsafe { &*tmp }
}


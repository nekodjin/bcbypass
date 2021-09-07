pub mod unrestricted;

pub unsafe fn dup_mut<'a, T>(brw: &T) -> &'a mut T {
    let tmp: *const T = brw;
    &mut *(tmp as *mut T)
}

pub unsafe fn dup_imm<'a, T>(brw: &T) -> &'a T {
    let tmp: *const T = brw;
    &*tmp
}


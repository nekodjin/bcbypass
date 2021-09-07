pub fn dup_mut<'a, T>(brw: &T) -> &'a mut T {
    let tmp: *const T = brw;
    unsafe { &mut *(tmp as *mut T) }
}

pub fn dup_imm<'a, T>(brw: &T) -> &'a T {
    let tmp: *const T = brw;
    unsafe { &*tmp }
}


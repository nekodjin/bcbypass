pub fn dup_mut<'a, T>(brw: &mut T) -> &'a mut T {
    let tmp: *mut T = brw;
    unsafe { &mut *tmp }
}

pub fn dup_imm<'a, T>(brw: &mut T) -> &'a T {
    let tmp: *const T = brw;
    unsafe { &*tmp }
}


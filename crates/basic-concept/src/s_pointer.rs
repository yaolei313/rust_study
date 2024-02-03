pub fn study_pointer() {
    let mut a = [1, 2];
    let p = a.as_mut_ptr();
    let p1_address = p as usize;
    let p2_address = p1_address + 4;
    let p2 = p2_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(a[1], 3);
}

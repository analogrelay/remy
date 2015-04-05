pub fn bcd_to_int(bcd: isize) -> isize {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
}

pub fn int_to_bcd(int: isize) -> isize {
    let mut v = if int > 99 {
        int - 100
    } else {
        int
    };
    if v > 99 || v < -99 {
        panic!("bcd overflow!");
    }
    if v < 0 {
        // Wrap around
        v = v + 100;
    }
    let h = (v / 10) as u8;
    let l = (v % 10) as u8;
    
    ((h << 4) | l) as isize
}

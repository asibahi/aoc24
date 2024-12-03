use core::slice::from_raw_parts;

advent_of_code::solution!(3);

const fn parse_mul_const(ioo: &[u8]) -> Option<(&[u8], u64)> {
    let mut length = 8;
    let lhs;
    let rhs;

    let mut cursor;

    if ioo.len() < length {
        return None;
    }
    if ioo[1] == b'u' && ioo[2] == b'l' && ioo[3] == b'(' {
        let d4 = ioo[4];
        let d5 = ioo[5];
        let d6 = ioo[6];
        if d4.is_ascii_digit() && d5.is_ascii_digit() && d6.is_ascii_digit() {
            lhs = 100 * (d4 - b'0') as u64 + 10 * (d5 - b'0') as u64 + (d6 - b'0') as u64;
            cursor = 7;
            length += 2;
        } else if d4.is_ascii_digit() && d5.is_ascii_digit() {
            lhs = 10 * (d4 - b'0') as u64 + (d5 - b'0') as u64;
            cursor = 6;
            length += 1;
        } else if d4.is_ascii_digit() {
            lhs = (d4 - b'0') as u64;
            cursor = 5;
        } else {
            return None;
        }
        if ioo[cursor] != b',' || ioo.len() < length {
            return None;
        }
        let d4 = ioo[cursor + 1];
        let d5 = ioo[cursor + 2];
        let d6 = ioo[cursor + 3];
        if d4.is_ascii_digit() && d5.is_ascii_digit() && d6.is_ascii_digit() {
            rhs = 100 * (d4 - b'0') as u64 + 10 * (d5 - b'0') as u64 + (d6 - b'0') as u64;
            cursor += 4;
            length += 2;
        } else if d4.is_ascii_digit() && d5.is_ascii_digit() {
            rhs = 10 * (d4 - b'0') as u64 + (d5 - b'0') as u64;
            cursor += 3;
            length += 1;
        } else if d4.is_ascii_digit() {
            rhs = (d4 - b'0') as u64;
            cursor += 2;
        } else {
            return None;
        }
        if ioo[cursor] != b')' || ioo.len() < length {
            return None;
        }
    } else {
        return None;
    }

    let rem = unsafe {
        let l = ioo.len();
        let p = ioo.as_ptr();

        from_raw_parts(p.wrapping_add(length), l - length)
    };
    Some((rem, lhs * rhs))
}

pub fn part_one(_: &str) -> Option<u64> {
    const INPUT: &[u8] = include_bytes!("../../data/inputs/03.txt");
    const ANSWER: u64 = const {
        let mut ioo = INPUT;
        let mut acc = 0;

        while !ioo.is_empty() {
            if ioo[0] != b'm' {
                ioo = unsafe {
                    let l = ioo.len();
                    let p = ioo.as_ptr();

                    from_raw_parts(p.wrapping_add(1), l - 1)
                };
                continue;
            }

            if let Some((rem, res)) = parse_mul_const(ioo) {
                ioo = rem;
                acc += res;
            } else {
                ioo = unsafe {
                    let l = ioo.len();
                    let p = ioo.as_ptr();

                    from_raw_parts(p.wrapping_add(1), l - 1)
                };
            }
        }

        acc
    };

    Some(ANSWER)
}

pub fn part_two(_: &str) -> Option<u64> {
    const INPUT: &[u8] = include_bytes!("../../data/inputs/03.txt");
    const ANSWER: u64 = const {
        let mut ioo = INPUT;
        let mut acc = 0;

        while !ioo.is_empty() {
            while !ioo.is_empty() {
                if ioo[0] != b'm' && ioo[0] != b'd' {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(1), l - 1)
                    };
                    continue;
                }

                if let Some((rem, res)) = parse_mul_const(ioo) {
                    ioo = rem;
                    acc += res;
                } else if let Some(()) = 'd: {
                    if ioo.len() < 7 {
                        break 'd None;
                    }

                    let s = b"don't()";
                    let mut idx = 0;
                    while idx < 7 {
                        if ioo[idx] != s[idx] {
                            break 'd None 
                        }
                        idx += 1;
                    }

                    Some(())
                } {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(7), l - 7)
                    };
                    break;
                } else {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(1), l - 1)
                    };
                }
            }

            while !ioo.is_empty() {
                if ioo[0] != b'd' {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(1), l - 1)
                    };
                    continue;
                }

                if let Some((rem, res)) = parse_mul_const(ioo) {
                    ioo = rem;
                    acc += res;
                } else if let Some(()) = 'd: {
                    if ioo.len() < 4 {
                        break 'd None;
                    }

                    let s = b"do()";
                    let mut idx = 0;
                    while idx < 4 {
                        if ioo[idx] != s[idx] {
                            break 'd None 
                        }
                        idx += 1;
                    }

                    Some(())
                } {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(4), l - 4)
                    };
                    break;
                } else {
                    ioo = unsafe {
                        let l = ioo.len();
                        let p = ioo.as_ptr();

                        from_raw_parts(p.wrapping_add(1), l - 1)
                    };
                }
            }
        }

        acc
    };

    Some(ANSWER)
}

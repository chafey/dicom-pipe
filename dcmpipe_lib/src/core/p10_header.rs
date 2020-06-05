

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn detect(bytes: &[u8] ) -> bool {
    // check length
    if bytes.len() < 128 + 4 {
        return false;
    }

    // check for 128 zeros
    for i in 0..128 {
        if bytes[i] != 0 {
            return false;
        }
    }

    // check for DICM
    if  bytes[128] =='D' as u8 && 
        bytes[129] == 'I' as u8 &&
        bytes[130] == 'C' as u8 && 
        bytes[131] == 'M' as u8 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::detect;

    #[test]
    fn p10_header_returns_true() {
        let mut p10_header: Vec<u8> = vec![];
        p10_header.resize(134, 0);
        p10_header[128] = 'D' as u8;
        p10_header[129] = 'I' as u8;
        p10_header[130] = 'C' as u8;
        p10_header[131] = 'M' as u8;

        assert_eq!(true, detect(&p10_header));
    }

    #[test]
    fn non_zero_preamble_returns_false() {
        let mut non_zero_preamble: Vec<u8> = vec![];
        non_zero_preamble.resize(134, 0);
        non_zero_preamble[0] = 1;
        non_zero_preamble[128] = 'D' as u8;
        non_zero_preamble[129] = 'I' as u8;
        non_zero_preamble[130] = 'C' as u8;
        non_zero_preamble[131] = 'M' as u8;

        assert_eq!(false, detect(&non_zero_preamble));
    }

    #[test]
    fn zero_preamble_no_dicm_returns_false() {
        let mut zero_preamble_no_dicm: Vec<u8> = vec![];
        zero_preamble_no_dicm.resize(134, 0);

        assert_eq!(false, detect(&zero_preamble_no_dicm));
    }


}
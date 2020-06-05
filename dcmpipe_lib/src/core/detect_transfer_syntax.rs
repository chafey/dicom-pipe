use crate::defn::ts::TSRef;
use crate::defn::constants::{ts};
use crate::defn::vr::{VR};

fn is_vr(bytes: &[u8]) -> bool {
    let vr = ((bytes[0] as u16) << 8) | bytes[1] as u16;
    match VR::from_code(vr) {
        Some(_) => true,
        None => false
    }
}

fn is_little_endian(bytes:& [u8]) -> bool {
    let group = ((bytes[0] as u16) << 8) | bytes[1] as u16;
    group > 2 && group <= 10
}

fn is_big_endian(bytes:& [u8]) -> bool {
    let group = ((bytes[1] as u16) << 8) | bytes[0] as u16;
    group > 2 && group <= 10
}

pub fn detect(bytes: &[u8] ) -> Option<TSRef> {

    if is_little_endian(bytes) {
        if is_vr(&bytes[4..6]) {
            Some(&ts::ExplicitVRLittleEndian)
        } else {
            Some(&ts::ImplicitVRLittleEndian)
        }
    } else if is_big_endian(bytes) {
        if is_vr(&bytes[4..6]) {
            Some(&ts::ExplicitVRBigEndian)
        } else {
            Some(&ts::ImplicitVRBigEndian)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::detect;
    use crate::defn::constants::{ts};

    #[test]
    fn preample_returns_none() {
        let preamble = vec![0,0,0,0,0,0];
        assert_eq!(None, detect(&preamble));
    }

    #[test]
    fn explicit_little_endian() {
        let image_type_ele = vec![0,8,0,8,0x43,0x53];
        assert_eq!(Some(&ts::ExplicitVRLittleEndian), detect(&image_type_ele));
    }

    #[test]
    fn implicit_little_endian() {
        let image_type_ile = vec![0,8,0,8,0,0,0,24];
        assert_eq!(Some(&ts::ImplicitVRLittleEndian), detect(&image_type_ile));
    }

    #[test]
    fn explicit_big_endian() {
        let image_type_ele = vec![8,0,8,0,0x43,0x53];
        assert_eq!(Some(&ts::ExplicitVRBigEndian), detect(&image_type_ele));
    }

    #[test]
    fn implicit_big_endian() {
        let image_type_ile = vec![8,0,8,0,0,0,0,24];
        assert_eq!(Some(&ts::ImplicitVRBigEndian), detect(&image_type_ile));
    }

    #[test]
    fn large_group_returns_none() {
        let large_groups = vec![20,20,20,20,0x43,0x53];
        assert_eq!(None, detect(&large_groups));
    }

}
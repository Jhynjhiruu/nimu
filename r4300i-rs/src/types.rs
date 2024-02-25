#![allow(non_camel_case_types)]

pub type byte = u8;
pub type hword = u16;
pub type word = u32;
pub type dword = u64;
pub type qword = u128;

pub type sbyte = i8;
pub type shword = i16;
pub type sword = i32;
pub type sdword = i64;
pub type sqword = i128;

pub fn lower_byte(x: hword) -> byte {
    x as _
}

pub fn upper_byte(x: hword) -> byte {
    (x >> 8) as _
}

pub fn lower_hword(x: word) -> hword {
    x as _
}

pub fn upper_hword(x: word) -> hword {
    (x >> 16) as _
}

pub fn lower_word(x: dword) -> word {
    x as _
}

pub fn upper_word(x: dword) -> word {
    (x >> 32) as _
}

pub fn lower_dword(x: qword) -> dword {
    x as _
}

pub fn upper_dword(x: qword) -> dword {
    (x >> 64) as _
}

pub fn sign_extend_byte(x: byte) -> hword {
    x as sbyte as shword as _
}

pub fn sign_extend_hword(x: hword) -> word {
    x as shword as sword as _
}

pub fn sign_extend_word(x: word) -> dword {
    x as sword as sdword as _
}

pub fn sign_extend_byte_twice(x: byte) -> word {
    sign_extend_hword(sign_extend_byte(x))
}

pub fn sign_extend_hword_twice(x: hword) -> dword {
    sign_extend_word(sign_extend_hword(x))
}

pub fn sign_extend_byte_thrice(x: byte) -> dword {
    sign_extend_word(sign_extend_hword(sign_extend_byte(x)))
}

pub fn retrieve_byte(val: word, address: word) -> byte {
    (val >> ((3 - (address & 3)) * 8)) as _
}

pub fn merge_byte(source: word, address: word, val: byte) -> word {
    let shift = (3 - (address & 3)) * 8;
    (source & !(0xFF << shift)) | ((val as word) << shift)
}

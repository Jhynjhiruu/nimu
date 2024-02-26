#![allow(unused_braces)]

use std::fmt::Debug;

use crate::types::*;

use modular_bitfield::prelude::*;

use super::Register;

pub trait Cop0Register: Sized + Debug {
    fn from_reg(val: word, reg: Register) -> Option<Self>;
    fn as_reg(&self, reg: Register) -> Option<word>;
}

macro_rules! impl_register {
    ($t:ty: $r:pat) => {
        impl Cop0Register for $t {
            fn from_reg(val: word, reg: Register) -> Option<Self> {
                if matches!(reg, $r) {
                    Some(Self::from(val as u32))
                } else {
                    None
                }
            }

            fn as_reg(&self, reg: Register) -> Option<word> {
                if matches!(reg, $r) {
                    Some((*self).into())
                } else {
                    None
                }
            }
        }
    };
    ($t:ty: $r:pat, $i:ty) => {
        impl Cop0Register for $t {
            fn from_reg(val: word, reg: Register) -> Option<Self> {
                if matches!(reg, $r) {
                    Some(Self::from(val as $i))
                } else {
                    None
                }
            }

            fn as_reg(&self, reg: Register) -> Option<word> {
                if matches!(reg, $r) {
                    Some(<Self as Into<dword>>::into(*self) as word)
                } else {
                    None
                }
            }
        }
    };
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Index {
    pub index: B6,
    #[skip]
    __: B25,
    pub probe: bool,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Random {
    pub random: B6,
    #[skip]
    __: B26,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct EntryLo {
    pub g: bool,
    pub v: bool,
    pub d: bool,
    pub c: B3,
    pub pfn: B20,
    #[skip]
    __: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Context {
    #[skip]
    __: B4,
    pub bad_vpn: B19,
    pub pte_base: B9,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct PageMask {
    #[skip]
    __: B13,
    pub mask: B12,
    #[skip]
    __: B7,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Wired {
    pub wired: B6,
    #[skip]
    __: B26,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct BadVAddr {
    pub bad_vaddr: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Count {
    pub count: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct EntryHi {
    pub asid: B8,
    #[skip]
    __: B4,
    pub g: bool,
    pub vpn: B19,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Compare {
    pub compare: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Status {
    pub ie: bool,
    pub exl: bool,
    pub erl: bool,
    pub ksu: B2,
    pub ux: bool,
    pub sx: bool,
    pub kx: bool,
    pub im: byte,
    pub de: bool,
    pub ce: bool,
    pub ch: bool,
    #[skip]
    __: B1,
    pub sr: bool,
    pub ts: bool,
    pub bev: bool,
    #[skip]
    __: B1,
    pub its: bool,
    pub re: bool,
    pub fr: bool,
    pub rp: bool,
    pub cu: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Cause {
    #[skip]
    __: B2,
    pub exc: B5,
    #[skip]
    __: B1,
    pub ip: byte,
    #[skip]
    __: B12,
    pub ce: B2,
    #[skip]
    __: B1,
    pub bd: bool,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Epc {
    pub epc: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct PrId {
    pub rev: byte,
    pub imp: byte,
    #[skip]
    __: B16,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Config {
    pub k0: B3,
    pub cu: bool,
    #[skip]
    __: B11,
    pub be: bool,
    #[skip]
    __: B8,
    pub ep: B4,
    pub ec: B3,
    #[skip]
    __: B1,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct LLAddr {
    pub ll_addr: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct WatchLo {
    pub w: bool,
    pub r: bool,
    #[skip]
    __: B1,
    pub p_addr: B29,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct WatchHi {
    pub p_addr: B4,
    #[skip]
    __: B28,
}

#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct XContext {
    #[skip]
    __: B4,
    pub bad_vpn: B27,
    pub r: B2,
    pub pte_base: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct CacheErr {
    pub cache_err: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct TagLo {
    #[skip]
    __: B6,
    pub p_state: B2,
    pub p_tag_lo: B20,
    #[skip]
    __: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct TagHi {
    p_tag_hi: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct ErrorEpc {
    pub error_epc: word,
}

impl_register!(Index: Register::Index);
impl_register!(Random: Register::Random);
impl_register!(EntryLo: Register::EntryLo0 | Register::EntryLo1);
impl_register!(Context: Register::Context);
impl_register!(PageMask: Register::PageMask);
impl_register!(Wired: Register::Wired);

impl_register!(BadVAddr: Register::BadVAddr);
impl_register!(Count: Register::Count);
impl_register!(EntryHi: Register::EntryHi);
impl_register!(Compare: Register::Compare);
impl_register!(Status: Register::Status);
impl_register!(Cause: Register::Cause);
impl_register!(Epc: Register::Epc);
impl_register!(PrId: Register::PrId);
impl_register!(Config: Register::Config);
impl_register!(LLAddr: Register::LLAddr);
impl_register!(WatchLo: Register::WatchLo);
impl_register!(WatchHi: Register::WatchHi);
impl_register!(XContext: Register::XContext, u64);

impl_register!(CacheErr: Register::CacheErr);
impl_register!(TagLo: Register::TagLo);
impl_register!(TagHi: Register::TagHi);
impl_register!(ErrorEpc: Register::ErrorEpc);

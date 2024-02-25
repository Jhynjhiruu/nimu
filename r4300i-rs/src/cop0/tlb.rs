use modular_bitfield::prelude::*;

use super::registers::*;

#[bitfield(bits = 128)]
#[derive(Debug, Clone, Copy)]
pub struct TLBEntry {
    pub entry_lo_1: EntryLo,
    pub entry_lo_0: EntryLo,
    pub entry_hi: EntryHi,
    pub page_mask: PageMask,
}

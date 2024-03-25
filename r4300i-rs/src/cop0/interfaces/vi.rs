use crate::types::*;

use modular_bitfield::prelude::*;

use super::DramAddr;

#[derive(Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum BitDepth {
    None,
    Bits16 = 2,
    Bits32,
}

#[derive(Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum AAMode {
    ExtraLines,
    NeededLines,
    AllCovered,
    Disabled,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Ctrl {
    bit_depth: BitDepth,
    gamma_dither: bool,
    gamma: bool,
    divot: bool,
    vbus_clock: bool,
    serrate: bool,
    test_mode: bool,
    aa: AAMode,
    #[skip]
    __: B1,
    kill_we: bool,
    pixel_advance: B4,
    dedither_filter: bool,
    #[skip]
    __: B15,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Width {
    width: B12,
    #[skip]
    __: B20,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct HalfLineCount {
    half_line: B10,
    #[skip]
    __: B22,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Burst {
    hsync: byte,
    colour_burst: byte,
    vsync: B4,
    colour_burst_offset: B10,
    #[skip]
    __: B2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct HSync {
    line_duration: B12,
    #[skip]
    __: B4,
    h_sync_period: B5,
    #[skip]
    __: B11,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Leap {
    leap: B12,
    #[skip]
    __: B4,
    h_sync_period: B12,
    #[skip]
    __: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Start {
    end: B10,
    #[skip]
    __: B6,
    start: B10,
    #[skip]
    __: B6,
}

#[bitfield(bits = 12)]
#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
pub struct Fixed210 {
    fractional: B10,
    int: B2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    scale: Fixed210,
    #[skip]
    __: B4,
    offset: Fixed210,
    #[skip]
    __: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SpanAddr {
    addr: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SpanData {
    data: word,
}

#[derive(Debug)]
pub struct Vi {
    control: Ctrl,
    origin: DramAddr,
    width: Width,
    intr: HalfLineCount,
    current: HalfLineCount,
    burst: Burst,
    v_sync: HalfLineCount,
    h_sync: HSync,
    leap: Leap,
    h_start: Start,
    v_start: Start,
    v_burst: Burst,
    x_scale: Scale,
    y_scale: Scale,
    span_addr: SpanAddr,
    span_data: SpanData,

    raise_interrupt: bool,
    scanline_clocks: u64,
}

impl Vi {
    pub fn new() -> Self {
        Self {
            control: Ctrl::new(),
            origin: DramAddr::new(),
            width: Width::new(),
            intr: HalfLineCount::new(),
            current: HalfLineCount::new(),
            burst: Burst::new(),
            v_sync: HalfLineCount::new(),
            h_sync: HSync::new(),
            leap: Leap::new(),
            h_start: Start::new(),
            v_start: Start::new(),
            v_burst: Burst::new(),
            x_scale: Scale::new(),
            y_scale: Scale::new(),
            span_addr: SpanAddr::new(),
            span_data: SpanData::new(),

            raise_interrupt: false,
            scanline_clocks: 0,
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04400000..=0x04400003 => retrieve_byte(self.control.into(), address),

            0x04400004..=0x04400007 => retrieve_byte(self.origin.into(), address),

            0x04400008..=0x0440000B => retrieve_byte(self.width.into(), address),

            0x0440000C..=0x0440000F => retrieve_byte(self.intr.into(), address),

            0x04400010..=0x04400013 => retrieve_byte(self.current.into(), address),

            0x04400014..=0x04400017 => retrieve_byte(self.burst.into(), address),

            0x04400018..=0x0440001B => retrieve_byte(self.burst.into(), address),

            0x0440001C..=0x0440001F => retrieve_byte(self.h_sync.into(), address),

            0x04400020..=0x04400023 => retrieve_byte(self.leap.into(), address),

            0x04400024..=0x04400027 => retrieve_byte(self.h_start.into(), address),

            0x04400028..=0x0440002B => retrieve_byte(self.v_start.into(), address),

            0x0440002C..=0x0440002F => retrieve_byte(self.v_burst.into(), address),

            0x04400030..=0x04400033 => retrieve_byte(self.x_scale.into(), address),

            0x04400034..=0x04400037 => retrieve_byte(self.y_scale.into(), address),

            0x04400038..=0x0440003B => retrieve_byte(self.span_addr.into(), address),

            0x0440003C..=0x0440003F => retrieve_byte(self.span_data.with_data(0).into(), address),

            _ => {
                eprintln!("unimplemented VI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04400000..=0x04400003 => {
                self.control = merge_byte(self.control.into(), address, val).into();
                println!("vi control: {:?}", self.control);
            }

            0x04400004..=0x04400007 => {
                self.origin = merge_byte(self.origin.into(), address, val).into();
                println!("vi origin: {:08X}", self.origin.addr());
            }

            0x04400008..=0x0440000B => {
                self.width = merge_byte(self.width.into(), address, val).into();
                println!("vi width: {:08X}", self.width.width());
            }

            0x0440000C..=0x0440000F => {
                self.intr = merge_byte(self.intr.into(), address, val).into();
                println!("vi intr: {:08X}", self.intr.half_line());
            }

            0x04400010..=0x04400013 => {
                self.raise_interrupt = false;
                println!("clearing vi interrupt!");
            }

            0x04400014..=0x04400017 => {
                self.burst = merge_byte(self.burst.into(), address, val).into();
                println!("vi burst: {:02X?}", self.burst);
            }

            0x04400018..=0x0440001B => {
                self.v_sync = merge_byte(self.v_sync.into(), address, val).into();
                println!("vi vsync: {:08X}", self.v_sync.half_line());
            }

            0x0440001C..=0x0440001F => {
                self.h_sync = merge_byte(self.h_sync.into(), address, val).into();
                println!("vi hsync: {:08X?}", self.h_sync);
            }

            0x04400020..=0x04400023 => {
                self.leap = merge_byte(self.leap.into(), address, val).into();
                println!("vi leap: {:08X?}", self.leap);
            }

            0x04400024..=0x04400027 => {
                self.h_start = merge_byte(self.h_start.into(), address, val).into();
                println!("vi hstart: {:08X?}", self.h_start);
            }

            0x04400028..=0x0440002B => {
                self.v_start = merge_byte(self.v_start.into(), address, val).into();
                println!("vi vstart: {:08X?}", self.v_start);
            }

            0x0440002C..=0x0440002F => {
                self.v_burst = merge_byte(self.v_burst.into(), address, val).into();
                println!("vi vburst: {:08X?}", self.v_burst);
            }

            0x04400030..=0x04400033 => {
                self.x_scale = merge_byte(self.x_scale.into(), address, val).into();
                println!("vi xscale: {:?}", self.x_scale);
            }

            0x04400034..=0x04400037 => {
                self.y_scale = merge_byte(self.y_scale.into(), address, val).into();
                println!("vi yscale: {:?}", self.y_scale);
            }

            0x04400038..=0x0440003B => {
                self.span_addr = merge_byte(self.span_addr.into(), address, val).into();
                println!("vi spanaddr: {:08X}", self.span_addr.addr());
            }

            0x0440003C..=0x0440003F => {
                self.span_data = merge_byte(self.span_data.into(), address, val).into();
                println!("vi spandata: {:08X}", self.span_data.data());
            }

            _ => {
                eprintln!("unimplemented VI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }

    pub fn has_interrupt(&self) -> bool {
        self.raise_interrupt
    }

    pub fn step(&mut self) {
        let current: u32 = self.current.half_line().into();
        let intr: u32 = self.intr.half_line().into();

        if current == intr {
            self.raise_interrupt = true;
        }

        self.scanline_clocks = self.scanline_clocks + 1;
        let quarter_pixels_displayed: u64 = (((self.scanline_clocks as f64) / 13.6) as u64);

        if quarter_pixels_displayed == self.h_sync.line_duration().into() {
            self.current.set_half_line((self.current.half_line() + 1) & 0x3FF);
            self.scanline_clocks = 0;
        }
    }
}

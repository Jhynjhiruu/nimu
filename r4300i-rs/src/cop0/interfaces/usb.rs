use crate::types::*;

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct PerId {
    id: byte,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct IdComp {
    id: byte,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Rev {
    rev: byte,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct AddInfo {
    iehost: bool,
    reserved1: B2,
    irqnum: B5,
    #[skip]
    __: B24,
}


// Interrupt status
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct OTGISTAT {
    avbuschg: bool,
    reserved1: B1,
    bsesschg: bool,
    sessvldchg: bool,
    reserved4: B1,
    linestatechg: bool,
    onemsec: bool,
    idchg: bool,
    #[skip]
    __: B24,
}


// Interrupt enable
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct OTGICR {
    avbusen: bool,
    reserved1: B1,
    bsessen: bool,
    sessvlden: bool,
    reserved4: B1,
    linestateen: bool,
    onemsecen: bool,
    iden: bool,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct OTGSTAT {
    avbuscld: bool,
    reserved1: B1,
    bsessend: bool,
    sessvld: bool,
    reserved4: B1,
    linestatestable: bool,
    onemsec: bool,
    id: bool,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct OTGCTL {
    reserved0: B2,
    otgen: bool,
    reserved3: B1,
    dmlow: bool,
    dplow: bool,
    reserved6: B1,
    dphigh: bool,
    #[skip]
    __: B24,
}


// USB Interrupt Status
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBISTAT {
    reset: bool,
    error: bool,
    softok: bool,
    tokdne: bool,
    sleep: bool,
    resume: bool,
    attach: bool,
    stall: bool,
    #[skip]
    __: B24,
}

// USB Interrupt Enable
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBINTEN {
    reseten: bool,
    erroren: bool,
    softoken: bool,
    tokdneen: bool,
    sleepen: bool,
    resumeen: bool,
    attachen: bool,
    stallen: bool,
    #[skip]
    __: B24,
}

// USB Error Status
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBERRSTAT {
    piderr: bool,
    cr5eof: bool,
    crc16: bool,
    dfn8: bool,
    btoerr: bool,
    dmaerr: bool,
    reserved6: B1,
    btserr: bool,
    #[skip]
    __: B24,
}

// USB Error Enable
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBERREN {
    piderren: bool,
    cr5eofen: bool,
    crc16en: bool,
    dfn8en: bool,
    btoerren: bool,
    dmaerren: bool,
    reserved6: B1,
    btserren: bool,
    #[skip]
    __: B24,
}

// USB Status
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBSTAT {
    reserved0: B2,
    odd: bool,
    tx: bool,
    endp: B4,
    #[skip]
    __: B24,
}

// USB Control
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBCTL {
    usbensofen: bool,
    oddrst: bool,
    resume: bool,
    hostmodeen: bool,
    reset: bool,
    txsuspendtokenbusy: bool,
    se0: bool,
    jstate: bool,
    #[skip]
    __: B24,
}

// USB Address
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct USBADDR {
    addr: B6,
    lsen: bool,
    #[skip]
    __: B25,
}

// Buffer Descriptor Table Page 1
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct BDTPAGE1 {
    reserved0: B1,
    bdtba: B7,
    #[skip]
    __: B24,
}

// Frame Number Low
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct FRMNUML {
    frm: byte,
    #[skip]
    __: B24,
}

// Frame Number High
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct FRMNUMH {
    frm: B3,
    reserved3: B5,
    #[skip]
    __: B24,
}

// USB Token
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct TOKEN {
    tokenendpt: B4,
    tokenpid: B4,
    #[skip]
    __: B24,
}

// SOF Threshold
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct SOFTHLD {
    cnt: byte,
    #[skip]
    __: B24,
}

// USB Buffer Descriptor Table Page 2
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct BDTPAGE2 {
    bdtba: byte,
    #[skip]
    __: B24,
}

// USB Buffer Descriptor Table Page 3
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct BDTPAGE3 {
    bdtba: byte,
    #[skip]
    __: B24,
}

// Endpoint
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct ENDPT {
    ephshk: bool,
    epstall: bool,
    eptxen: bool,
    eprxen: bool,
    epctldis: bool,
    reserved5: B1,
    retrydis: bool,
    hostwohub: bool,
    #[skip]
    __: B24,
}

// Access Enable
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct ACCESS {
    accessen: bool,
    #[skip]
    __: B31,
}

// Buffer Descriptor
#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct BD {
    reserved0: B2,
    stall: bool,
    dts: bool,
    ninc: bool,
    keep: bool,
    data01: bool,
    own: bool,
    reserved8: B8,
    bc: B10,
    #[skip]
    __: B6,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct BDWrite {
    reserved0: B2,
    bdtkpid: B4,
    data01: bool,
    own: bool,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct ClockSel {
    clock: bool,
    #[skip]
    __: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SecMode {
    sec: bool,
    #[skip]
    __: B31,
}

// BDT Address
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct BDTADDRESS {
    #[skip]
    __: B3,
    odd: bool,
    tx: bool,
    endpoint: B4,
    page1: B7,
    page2: B8,
    page3: B8,
}

#[derive(Debug, Clone, Copy)]
pub struct BDINFO {
    is_odd: bool,
    token: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct StatusFifoEntry {
    entry: USBSTAT,
    is_valid: bool,
}

#[derive(Debug)]
pub struct Usb {
    base_address: word,

    perid: PerId,
    idcomp: IdComp,
    revision: Rev,
    addinfo: AddInfo, 
    otgistat: OTGISTAT,
    otgicr: OTGICR,
    otgstat: OTGSTAT,
    otgctl: OTGCTL,
    usbistat: USBISTAT,
    usbinten: USBINTEN,
    usberrstat: USBERRSTAT,
    usberren: USBERREN,
    usbctl: USBCTL,
    usbaddr: USBADDR,
    bdtpage1: BDTPAGE1,
    frmnuml: FRMNUML,
    frmnumh: FRMNUMH,
    token: TOKEN,
    softhld: SOFTHLD,
    bdtpage2: BDTPAGE2,
    bdtpage3: BDTPAGE3,
    endpt: [ENDPT; Self::NUM_ENDPTS],
    access: ACCESS,
    clock_sel: ClockSel,
    sec_mode: SecMode,

    sram: [u8; Self::SRAM_SIZE],

    byte_time_counter: u32,
    sof_count: u32,
    id_state: bool,
    wait_for_sof: bool,
    token_processing: bool,
    transaction_state: u32,


    bdinfo: [BDINFO; Self::NUM_ENDPTS],

    transfer_timer: u32,
    transfer_state: u32,

    status_fifo: [StatusFifoEntry; Self::STATUS_FIFO_SIZE],

}

impl Usb {
    const SRAM_SIZE: usize = 0x200;

    const SRAM_START: word = 0x80000;
    const SRAM_END: word = Self::SRAM_START + Self::SRAM_SIZE as word;

    const NUM_ENDPTS: usize = 16;

    const NUM_TICKS_BYTE_TIME: u32 = 11; 

    const SOF_COUNTER_INITIAL_COUNT: u32 = 12000;

    const OUT_TOKEN:   u8 = 0x1;
    const IN_TOKEN:    u8 = 0x9;
    const SOF_TOKEN:   u8 = 0x5;
    const SETUP_TOKEN: u8 = 0xB;

    const STATUS_FIFO_SIZE: usize = 4;

    pub fn new(base_address: word) -> Self {
        Self {
            base_address,
            clock_sel: ClockSel::new(),
            sec_mode: SecMode::new(),
            perid: PerId::new().with_id(4),
            idcomp: IdComp::new().with_id(!4),
            revision: Rev::new(),
            addinfo: AddInfo::new(), 
            otgistat: OTGISTAT::new(), 
            otgicr: OTGICR::new(),
            otgstat: OTGSTAT::new().with_linestatestable(true),
            otgctl: OTGCTL::new(),
            usbistat: USBISTAT::new(),
            usbinten: USBINTEN::new(),
            usberrstat: USBERRSTAT::new(),
            usberren: USBERREN::new(),
            usbctl: USBCTL::new(),
            usbaddr: USBADDR::new(),
            bdtpage1: BDTPAGE1::new(),
            frmnuml: FRMNUML::new(),
            frmnumh: FRMNUMH::new(),
            token: TOKEN::new(),
            softhld: SOFTHLD::new(),
            bdtpage2: BDTPAGE2::new(),
            bdtpage3: BDTPAGE3::new(),
            endpt: [0.into(); Self::NUM_ENDPTS],
            access: ACCESS::new(),

            sram: [0; Self::SRAM_SIZE],

            byte_time_counter: Self::NUM_TICKS_BYTE_TIME,
            sof_count: Self::SOF_COUNTER_INITIAL_COUNT,
            id_state: true,
            wait_for_sof: true,
            token_processing: false,
            transaction_state: 0,
            bdinfo: [BDINFO {is_odd: false, token: 0}; Self::NUM_ENDPTS],
            transfer_timer: 0,
            transfer_state: 0,
            status_fifo: [StatusFifoEntry {entry: USBSTAT::new(), is_valid: false}; Self::STATUS_FIFO_SIZE],
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        let int_address = address - self.base_address;
        if (address & 3) == 3 {
            let temp = address & !3;
            println!("Read from USB address: {temp:08X}");
        }

        match int_address {
            0x0000..=0x0003 => retrieve_byte(self.perid.into(), address),

            0x0004..=0x0007 => retrieve_byte(self.idcomp.into(), address),

            0x0008..=0x000B => retrieve_byte(self.revision.into(), address),

            0x000C..=0x000F => retrieve_byte(self.addinfo.into(), address),

            0x0010..=0x0013 => retrieve_byte(self.otgistat.into(), address),

            0x0014..=0x0017 => retrieve_byte(self.otgicr.into(), address),

            0x0018..=0x001B => retrieve_byte(self.otgstat.into(), address),

            0x001C..=0x001F => retrieve_byte(self.otgctl.into(), address),

            0x0020..=0x007F => {
                println!("ignored USB reg read @ {address:08X}");
                0
            }

            0x0080..=0x0083 => retrieve_byte(self.usbistat.into(), address),

            0x0084..=0x0087 => retrieve_byte(self.usbinten.into(), address),

            0x0088..=0x008B => retrieve_byte(self.usberrstat.into(), address),

            0x008C..=0x008F => retrieve_byte(self.usberren.into(), address),

            0x0090..=0x0093 => retrieve_byte(self.status_fifo[0].entry.into(), address),

            0x0094..=0x0097 => retrieve_byte(self.usbctl.into(), address),

            0x0098..=0x009B => retrieve_byte(self.usbaddr.into(), address),

            0x009C..=0x009F => retrieve_byte(self.bdtpage1.into(), address),

            0x00A0..=0x00A3 => retrieve_byte(self.frmnuml.into(), address),

            0x00A4..=0x00A7 => retrieve_byte(self.frmnumh.into(), address),

            0x00A8..=0x00AB => retrieve_byte(self.token.into(), address),

            0x00AC..=0x00AF => retrieve_byte(self.softhld.into(), address),

            0x00B0..=0x00B3 => retrieve_byte(self.bdtpage2.into(), address),

            0x00B4..=0x00B7 => retrieve_byte(self.bdtpage3.into(), address),

            0x00B8..=0x00BF => {
                println!("ignored USB reg read @ {address:08X}");
                0
            }

            0x00C0..=0x00C3 => retrieve_byte(self.endpt[0].into(), address),

            0x00C4..=0x00C7 => retrieve_byte(self.endpt[1].into(), address),

            0x00C8..=0x00CB => retrieve_byte(self.endpt[2].into(), address),

            0x00CC..=0x00CF => retrieve_byte(self.endpt[3].into(), address),

            0x00D0..=0x00D3 => retrieve_byte(self.endpt[4].into(), address),

            0x00D4..=0x00D7 => retrieve_byte(self.endpt[5].into(), address),

            0x00D8..=0x00DB => retrieve_byte(self.endpt[6].into(), address),

            0x00DC..=0x00DF => retrieve_byte(self.endpt[7].into(), address),

            0x00E0..=0x00E3 => retrieve_byte(self.endpt[8].into(), address),

            0x00E4..=0x00E7 => retrieve_byte(self.endpt[9].into(), address),

            0x00E8..=0x00EB => retrieve_byte(self.endpt[10].into(), address),

            0x00EC..=0x00EF => retrieve_byte(self.endpt[11].into(), address),

            0x00F0..=0x00F3 => retrieve_byte(self.endpt[12].into(), address),

            0x00F4..=0x00F7 => retrieve_byte(self.endpt[13].into(), address),

            0x00F8..=0x00FB => retrieve_byte(self.endpt[14].into(), address),

            0x00FC..=0x00FF => retrieve_byte(self.endpt[15].into(), address),

            0x40000..=0x40003 => retrieve_byte(self.clock_sel.into(), address),

            0x40010..=0x40013 => retrieve_byte(self.access.into(), address),

            Self::SRAM_START..=Self::SRAM_END => {
                self.sram[(int_address - Self::SRAM_START) as usize]
            }

            _ => {
                eprintln!("unimplemented USB read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        println!("USB reg write @ {address:08X}: {val:02X}");
        let int_address = address - self.base_address;
        match int_address {
            
            0x0000..=0x0003 => {
            }
            
            0x0004..=0x0007 => {
            }
            
            0x0008..=0x000B => {
            }
            
            0x000C..=0x000F => {
            }
            
            0x0010..=0x0013 => {
                let otgistat_write: OTGISTAT = merge_byte(0, address, val).into();

                if otgistat_write.avbuschg() {
                    self.otgistat.set_avbuschg(false);
                }

                if otgistat_write.bsesschg() {
                    self.otgistat.set_bsesschg(false);
                }

                if otgistat_write.sessvldchg() {
                    self.otgistat.set_sessvldchg(false);
                }

                if otgistat_write.linestatechg() {
                    self.otgistat.set_linestatechg(false);
                }

                if otgistat_write.onemsec() {
                    self.otgistat.set_onemsec(false);
                }

                if otgistat_write.idchg() {
                    self.otgistat.set_idchg(false);
                }
            }
            
            0x0014..=0x0017 => {
                self.otgicr = merge_byte(self.otgicr.into(), address, val).into()
            }
            
            0x0018..=0x001B => {
            }
            
            0x001C..=0x001F => {
                self.otgctl = merge_byte(self.otgctl.into(), address, val).into()
            }

            0x0020..=0x007F => {
                println!("ignored USB reg write @ {address:08X}: {val:02X}");
            }

            0x0080..=0x0083 => {
                let usbistatWrite: USBISTAT = merge_byte(0, address, val).into();

                if usbistatWrite.reset() {
                    self.usbistat.set_reset(false);
                }

                if usbistatWrite.error() {
                    self.usbistat.set_error(false);
                }

                if usbistatWrite.softok() {
                    self.usbistat.set_softok(false);
                }

                if usbistatWrite.tokdne() {
                    self.usbistat.set_tokdne(false);
                }

                if usbistatWrite.sleep() {
                    self.usbistat.set_sleep(false);
                }

                if usbistatWrite.resume() {
                    self.usbistat.set_resume(false);
                }

                if usbistatWrite.attach() {
                    self.usbistat.set_attach(false);
                }

                if usbistatWrite.stall() {
                    self.usbistat.set_stall(false);
                }
            }
            
            0x0084..=0x0087 => {
                self.usbinten = merge_byte(self.usbinten.into(), address, val).into()
            }
            
            0x0088..=0x008B => {
                let usberrstat_write: USBERRSTAT = merge_byte(0, address, val).into();

                if usberrstat_write.piderr() {
                    self.usberrstat.set_piderr(false);
                }

                if usberrstat_write.cr5eof() {
                    self.usberrstat.set_cr5eof(false);
                }

                if usberrstat_write.crc16() {
                    self.usberrstat.set_crc16(false);
                }

                if usberrstat_write.dfn8() {
                    self.usberrstat.set_dfn8(false);
                }

                if usberrstat_write.btoerr() {
                    self.usberrstat.set_btoerr(false);
                }

                if usberrstat_write.dmaerr() {
                    self.usberrstat.set_dmaerr(false);
                }

                if usberrstat_write.btserr() {
                    self.usberrstat.set_btserr(false);
                }
            }
            
            0x008C..=0x008F => {
                self.usberren = merge_byte(self.usberren.into(), address, val).into()
            }
            
            0x0090..=0x0093 => {
            }
            
            0x0094..=0x0097 => {
                self.usbctl = merge_byte(self.usbctl.into(), address, val).into()
            }
            
            0x0098..=0x009B => {
                self.usbaddr = merge_byte(self.usbaddr.into(), address, val).into()
            }
            
            0x009C..=0x009F => {
                self.bdtpage1 = merge_byte(self.bdtpage1.into(), address, val).into()
            }
            
            0x00A0..=0x00A3 => {
            }

            0x00A4..=0x00A7 => {
            }

            0x00A8..=0x00AB => {
                self.token = merge_byte(self.token.into(), address, val).into();
                if (address & 3) == 3 {
                    self.token_processing = true;
                }
            }
            
            0x00AC..=0x00AF => {
                self.softhld = merge_byte(self.softhld.into(), address, val).into()
            }

            0x00B0..=0x00B3 => {
                self.bdtpage2 = merge_byte(self.bdtpage2.into(), address, val).into()
            }
            
            0x00B4..=0x00B7 => {
                self.bdtpage3 = merge_byte(self.bdtpage3.into(), address, val).into()
            }

            0x00B8..=0x00BF => {
                println!("ignored USB reg write @ {address:08X}: {val:02X}")
            }

            0x00C0..=0x00C3 => {
                self.endpt[0] = merge_byte(self.endpt[0].into(), address, val).into()
            }

            0x00C4..=0x00C7 => {
                self.endpt[1] = merge_byte(self.endpt[1].into(), address, val).into()
            }

            0x00C8..=0x00CB => {
                self.endpt[2] = merge_byte(self.endpt[2].into(), address, val).into()
            }

            0x00CC..=0x00CF => {
                self.endpt[3] = merge_byte(self.endpt[3].into(), address, val).into()
            }

            0x00D0..=0x00D3 => {
                self.endpt[4] = merge_byte(self.endpt[4].into(), address, val).into()
            }

            0x00D4..=0x00D7 => {
                self.endpt[5] = merge_byte(self.endpt[5].into(), address, val).into()
            }

            0x00D8..=0x00DB => {
                self.endpt[6] = merge_byte(self.endpt[6].into(), address, val).into()
            }

            0x00DC..=0x00DF => {
                self.endpt[7] = merge_byte(self.endpt[7].into(), address, val).into()
            }

            0x00E0..=0x00E3 => {
                self.endpt[8] = merge_byte(self.endpt[8].into(), address, val).into()
            }

            0x00E4..=0x00E7 => {
                self.endpt[9] = merge_byte(self.endpt[9].into(), address, val).into()
            }

            0x00E8..=0x00EB => {
                self.endpt[10] = merge_byte(self.endpt[10].into(), address, val).into()
            }

            0x00EC..=0x00EF => {
                self.endpt[11] = merge_byte(self.endpt[11].into(), address, val).into()
            }

            0x00F0..=0x00F3 => {
                self.endpt[12] = merge_byte(self.endpt[12].into(), address, val).into()
            }

            0x00F4..=0x00F7 => {
                self.endpt[13] = merge_byte(self.endpt[13].into(), address, val).into()
            }

            0x00F8..=0x00FB => {
                self.endpt[14] = merge_byte(self.endpt[14].into(), address, val).into()
            }

            0x00FC..=0x00FF => {
                self.endpt[15] = merge_byte(self.endpt[15].into(), address, val).into()
            }

            0x40000..=0x40003 => {
                self.clock_sel = merge_byte(self.clock_sel.into(), address, val).into()
            }

            0x40010..=0x40013 => {
                self.access = merge_byte(self.access.into(), address, val).into()
            }

            Self::SRAM_START..=Self::SRAM_END => {
                self.sram[(int_address - Self::SRAM_START) as usize] = val
            }

            _ => {
                eprintln!("unimplemented USB write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }

    pub fn get_framenum(&self) -> u32 {
        return self.frmnuml.frm() as u32 | ((self.frmnumh.frm() as u32) << 8);
    }

    pub fn transmit_sof(&self) {
        let framenum: u32 = self.get_framenum();
        let data: [u8;3] = [Self::SOF_TOKEN | (!Self::SOF_TOKEN << 4), (framenum>>3) as u8, ((framenum & 7) << 5) as u8];

        self.start_transmit(&data);
    }

    pub fn start_transmit(&self, data: &[u8]) {
        for i in 0..data.len() {
            let byte = data[i];
            println!("USB transmit: {byte:02x}");
        }
    }

    pub fn status_fifo_push(&mut self, status: USBSTAT) {
        for i in 0..self.status_fifo.len()-1 {
            if !self.status_fifo[i].is_valid {
                self.status_fifo[i].entry = status;
                self.status_fifo[i].is_valid = true;
                return;
            }
        }

        for i in 0..self.status_fifo.len()-2 {
            self.status_fifo[i] = self.status_fifo[i+1];
        }

        self.status_fifo[3].entry = status;
        self.status_fifo[3].is_valid = true;
    }

    pub fn status_fifo_pop(&mut self) {
        for i in 0..self.status_fifo.len()-2 {
            self.status_fifo[i] = self.status_fifo[i+1];
        }

        self.status_fifo[3].entry = USBSTAT::new();
        self.status_fifo[3].is_valid = false;
    }

    pub fn is_tx(&self, tokenpid: u8) -> bool {
        match tokenpid {
            Self::SOF_TOKEN   => { return true; }
            Self::OUT_TOKEN   => { return true; }
            Self::SETUP_TOKEN => { return true; }
            _ => { return false; }
        }
    }

    pub fn bdt_address(&self, endpoint: usize) -> u32 {
        BDTADDRESS::new().with_page3(self.bdtpage3.bdtba())
                         .with_page2(self.bdtpage2.bdtba())
                         .with_page1(self.bdtpage1.bdtba())
                         .with_endpoint(endpoint as u8)
                         .with_tx(self.is_tx(self.token.tokenpid().into()))
                         .with_odd(self.bdinfo[endpoint].is_odd.into())
                         .into()
    }

    pub fn step_host(&mut self) {
        if (self.sof_count == self.softhld.cnt().into()) {
            self.usbistat.set_softok(true);
        }

        if self.sof_count == 0 && self.usbctl.usbensofen() {
            self.transmit_sof();
        }

        self.usbctl.set_txsuspendtokenbusy(self.token_processing);

        if self.token_processing {
            let sram_addr: usize = (self.bdt_address(self.token.tokenendpt().into()) - self.base_address - Self::SRAM_START) as usize;
            match self.transfer_state {
                0 => {
                    let mut bd: BD = BD::new();
                    for i in 0..3 {
                        bd = merge_byte(bd.into(), (sram_addr + i) as u32, self.sram[sram_addr + i]).into();
                    }
                    let tokenpid: u8 = self.token.tokenpid().into();
                    let endpoint: usize = self.token.tokenendpt().into();

                    match tokenpid {
                        Self::OUT_TOKEN => {
                            // Handle out token
                        }

                        Self::SETUP_TOKEN => {
                            // Handle setup token
                        }

                        Self::IN_TOKEN => {
                            // Handle in token
                        }

                        _ => {
                            eprintln!("unknown tokenpid: {tokenpid:02x}");
                            unreachable!();
                        }
                    }

                }

                1 => {
                    // Wait for "transfer" to finish.

                    // Once transfer finishes
                    // TODO: handle BD modification
                    self.transfer_state = 0;
                    self.token_processing = false;
                }

                _ => {
                    eprintln!("unreachable USB transfer state!");
                    unreachable!();
                }
            }
        }
    }

    pub fn step_device(&mut self) {

    }

    pub fn step(&mut self) {
        if self.usbctl.hostmodeen() {
            self.step_host();
        } else {
            self.step_device();
        }

        if self.sof_count == 0 {
            if self.otgicr.onemsecen() {
                self.otgistat.set_onemsec(true);
            }
        }

        if self.sof_count == 0 {
            self.sof_count = Self::SOF_COUNTER_INITIAL_COUNT;
        }

        if self.byte_time_counter == 0 {
            self.byte_time_counter = Self::NUM_TICKS_BYTE_TIME;
            self.sof_count -= 1;
        }

        self.byte_time_counter -= 1;
    }
}

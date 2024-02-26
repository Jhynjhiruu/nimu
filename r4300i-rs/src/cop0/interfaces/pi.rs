use crate::types::*;

use modular_bitfield::prelude::*;
use soft_aes::aes::aes_dec_cbc;

use super::DramAddr;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct CartAddr {
    addr: word,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct TransferLen {
    len: B24,
    #[skip]
    __: B8,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Status {
    reset: bool,
    clear: bool,
    error: bool,
    #[skip]
    __: B29,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Latency {
    latency: byte,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct PulseWidth {
    width: byte,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct PageSize {
    width: B4,
    #[skip]
    __: B28,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Release {
    width: B2,
    #[skip]
    __: B30,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FlashCtrl {
    data_phase_size: B10,
    multi_cycle: bool,
    ecc: bool,
    device_id: B2,
    buf: B1,
    wait_ready: bool,
    command: byte,
    phase: B4,
    read_data_phase: bool,
    write_data_phase: bool,
    interrupt: bool,
    run: bool,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct AesCtrl {
    chain: bool,
    iv: B7,
    #[skip]
    __: B1,
    data: B7,
    len: B6,
    #[skip]
    __: B8,
    interrupt: bool,
    run: bool,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Access {
    buf: bool,
    flash: bool,
    atb: bool,
    aes: bool,
    dma: bool,
    gpio: bool,
    ioc: bool,
    error: bool,
    #[skip]
    __: B24,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Gpio {
    data: B4,
    output_enable: B4,
    #[skip]
    __: B14,
    clock_multiplier: B3,
    system_clock_rate: B2,
    memory_size: B1,
    board_config: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct IDEConfig {
    pio_assertion_time: B5,
    pio_deassertion_time: B5,
    pio_cycle_end_time: B6,
    dma_assertion_time: B5,
    dma_deassertion_time: B5,
    dma_cycle_end_time: B5,
    reset: bool,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct FlashAddress {
    addr: B29,
    #[skip]
    __: B3,
}

#[derive(Debug)]
pub struct Pi {
    dram_addr: DramAddr,
    cart_addr: CartAddr,
    read_len: TransferLen,
    write_len: TransferLen,
    buffer_read_len: TransferLen,
    buffer_write_len: TransferLen,

    status: Status,
    dma_busy: bool,
    io_busy: bool,
    error: bool,

    dom1_latency: Latency,
    dom1_pulse_width: PulseWidth,
    dom1_page_size: PageSize,
    dom1_release: Release,

    dom2_latency: Latency,
    dom2_pulse_width: PulseWidth,
    dom2_page_size: PageSize,
    dom2_release: Release,

    flash_ctrl: FlashCtrl,
    flash_double_error: bool,
    flash_single_error: bool,
    flash_interrupt: bool,
    flash_busy: bool,

    aes_ctrl: AesCtrl,
    aes_interrupt: bool,
    aes_busy: bool,
    last_block: [byte; 0x10],

    access: Access,

    gpio: Gpio,
    ide_config: IDEConfig,
    flash_addr: FlashAddress,

    buf: [byte; 0x800],
    ide: [[byte; 4]; 4],

    nand: Vec<byte>,
    spare: Vec<byte>,
}

#[derive(Debug, Clone, Copy)]
pub enum Dma {
    Read(word, word, word),
    Write(word, word, word),
    BufRead(word, word, word),
    BufWrite(word, word, word),
}

impl Pi {
    pub fn new(mut nand: Vec<byte>, mut spare: Vec<byte>) -> Self {
        let nand_size = if nand.len() < (96 * 1024 * 1024) {
            64 * 1024 * 1024
        } else {
            128 * 1024 * 1024
        };
        nand.resize(nand_size, 0);
        spare.resize(nand_size / 1024, 0);
        Self {
            dram_addr: DramAddr::new(),
            cart_addr: CartAddr::new(),
            read_len: TransferLen::new(),
            write_len: TransferLen::new(),
            buffer_read_len: TransferLen::new(),
            buffer_write_len: TransferLen::new(),

            status: Status::new(),
            dma_busy: false,
            io_busy: false,
            error: false,

            dom1_latency: Latency::new(),
            dom1_pulse_width: PulseWidth::new(),
            dom1_page_size: PageSize::new(),
            dom1_release: Release::new(),

            dom2_latency: Latency::new(),
            dom2_pulse_width: PulseWidth::new(),
            dom2_page_size: PageSize::new(),
            dom2_release: Release::new(),

            flash_ctrl: FlashCtrl::new(),
            flash_double_error: false,
            flash_single_error: false,
            flash_interrupt: false,
            flash_busy: false,

            aes_ctrl: AesCtrl::new(),
            aes_interrupt: false,
            aes_busy: false,
            last_block: [0; 0x10],

            access: Access::new(),

            gpio: Gpio::new(),
            ide_config: IDEConfig::new(),
            flash_addr: FlashAddress::new(),

            buf: [0; 0x800],
            ide: [[0; 4]; 4],

            nand,
            spare,
        }
    }

    pub fn retrieve_nand(&self) -> Vec<byte> {
        self.nand.clone()
    }

    pub fn retrieve_spare(&self) -> Vec<byte> {
        self.spare.clone()
    }

    pub fn dma_queued(&self) -> bool {
        self.dma_busy
    }

    pub fn dma_params(&self) -> Dma {
        let dram_addr = self.dram_addr.addr();
        let cart_addr = self.cart_addr.addr();

        let read = self.read_len.len();
        let write = self.write_len.len();
        let buf_read = self.buffer_read_len.len();
        let buf_write = self.buffer_write_len.len();

        match (read, write, buf_read, buf_write) {
            (l, 0, 0, 0) => Dma::Read(dram_addr, cart_addr, (l + 1) & 0x00FFFFFF),
            (0, l, 0, 0) => Dma::Write(dram_addr, cart_addr, (l + 1) & 0x00FFFFFF),
            (0, 0, l, 0) => Dma::BufRead(dram_addr, cart_addr, (l + 1) & 0x00FFFFFF),
            (0, 0, 0, l) => Dma::BufWrite(dram_addr, cart_addr, (l + 1) & 0x00FFFFFF),
            _ => unreachable!(),
        }
    }

    pub fn clear_dma(&mut self) {
        self.dma_busy = false;
        self.write_len.set_len(0);
        self.read_len.set_len(0);
    }

    pub fn bus_read(&self, address: word, length: word) -> &[byte] {
        match address {
            0x00000000..=0x04FFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
            0x05000000..=0x05FFFFFF => match address {
                _ => unimplemented!("pi bus domain 2 read {address:08X}"),
            },
            0x06000000..=0x07FFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
            0x08000000..=0x0FFFFFFF => match address {
                _ => unimplemented!("pi bus domain 2 read {address:08X}"),
            },
            0x10000000..=0xFFFFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
        }
    }

    pub fn bus_write(&mut self, address: word, length: word, data: &[byte]) {
        match address {
            0x00000000..=0x04FFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
            0x05000000..=0x05FFFFFF => match address {
                _ => unimplemented!("pi bus domain 2 read {address:08X}"),
            },
            0x06000000..=0x07FFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
            0x08000000..=0x0FFFFFFF => match address {
                _ => unimplemented!("pi bus domain 2 read {address:08X}"),
            },
            0x10000000..=0xFFFFFFFF => match address {
                _ => unimplemented!("pi bus domain 1 read {address:08X}"),
            },
        }
    }

    pub fn buf_read(&self, address: word, length: word) -> &[byte] {
        match address {
            0x00..=0x400 => &self.buf[address as usize..(address + length) as usize],
            _ => unimplemented!("pi buf read {address:08X}"),
        }
    }

    pub fn buf_write(&mut self, address: word, length: word, data: &[byte]) {
        match address {
            0x00..=0x400 => {
                self.buf[address as usize..(address + length) as usize].copy_from_slice(data)
            }
            _ => unimplemented!("pi bus domain 1 read {address:08X}"),
        }
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04600000..=0x04600003 => retrieve_byte(self.dram_addr.into(), address),

            0x04600004..=0x04600007 => retrieve_byte(self.cart_addr.into(), address),

            0x04600008..=0x0460000B => retrieve_byte(self.read_len.into(), address),

            0x0460000C..=0x0460000F => retrieve_byte(self.write_len.into(), address),

            0x04600010..=0x04600013 => retrieve_byte(
                self.status
                    .with_reset(self.dma_busy)
                    .with_clear(self.io_busy)
                    .with_error(self.error)
                    .into(),
                address,
            ),

            0x04600014..=0x04600017 => retrieve_byte(self.dom1_latency.into(), address),

            0x04600018..=0x0460001B => retrieve_byte(self.dom1_pulse_width.into(), address),

            0x0460001C..=0x0460001F => retrieve_byte(self.dom1_page_size.into(), address),

            0x04600020..=0x04600023 => retrieve_byte(self.dom1_release.into(), address),

            0x04600024..=0x04600027 => retrieve_byte(self.dom2_latency.into(), address),

            0x04600028..=0x0460002B => retrieve_byte(self.dom2_pulse_width.into(), address),

            0x0460002C..=0x0460002F => retrieve_byte(self.dom2_page_size.into(), address),

            0x04600030..=0x04600033 => retrieve_byte(self.dom2_release.into(), address),

            0x04600048..=0x0460004B => retrieve_byte(
                self.flash_ctrl
                    .with_multi_cycle(self.flash_double_error)
                    .with_ecc(self.flash_single_error)
                    .with_interrupt(self.flash_interrupt)
                    .with_run(self.flash_busy)
                    .into(),
                address,
            ),

            0x04600050..=0x04600053 => retrieve_byte(
                self.aes_ctrl
                    .with_interrupt(self.aes_interrupt)
                    .with_run(self.aes_busy)
                    .into(),
                address,
            ),

            0x04600054..=0x04600057 => retrieve_byte(self.access.into(), address),

            0x04600058..=0x0460005B => retrieve_byte(self.buffer_read_len.into(), address),

            0x0460005C..=0x0460005F => retrieve_byte(self.buffer_write_len.into(), address),

            0x04600060..=0x04600063 => retrieve_byte(self.gpio.into(), address),

            0x04600064..=0x04600067 => retrieve_byte(self.ide_config.into(), address),

            0x04600070..=0x04600073 => retrieve_byte(self.flash_addr.into(), address),

            0x04610000..=0x046107FF => self.buf[(address - 0x04610000) as usize],

            0x04620000..=0x04620003 => {
                if (address & 3) == 0 {
                    println!(
                        "\n----------------\nide: {:08X} {:08X} {:08X} {:08X}\n----------------\n",
                        u32::from_be_bytes(self.ide[0]),
                        u32::from_be_bytes(self.ide[1]),
                        u32::from_be_bytes(self.ide[2]),
                        u32::from_be_bytes(self.ide[3]),
                    );
                }
                0
            }

            0x04680000..=0x04680003 => self.ide[0][(address & 3) as usize],
            0x046A0000..=0x046A0003 => self.ide[1][(address & 3) as usize],
            0x046C0000..=0x046C0003 => self.ide[2][(address & 3) as usize],
            0x046E0000..=0x046E0003 => self.ide[3][(address & 3) as usize],

            0x046FFFE0..=0x046FFFFF => 0,

            _ => {
                eprintln!("unimplemented PI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04600000..=0x04600003 => {
                self.dram_addr = merge_byte(self.dram_addr.into(), address, val).into()
            }

            0x04600004..=0x04600007 => {
                self.cart_addr = merge_byte(self.cart_addr.into(), address, val).into()
            }

            0x04600008..=0x0460000B => {
                self.read_len = merge_byte(self.read_len.into(), address, val).into();
                self.dma_busy = true;
            }

            0x0460000C..=0x0460000F => {
                self.write_len = merge_byte(self.write_len.into(), address, val).into();
                self.dma_busy = true;
            }

            0x04600010..=0x04600013 => {
                self.status = merge_byte(self.status.into(), address, val).into();
                self.status.set_error(false);
            }

            0x04600014..=0x04600017 => {
                self.dom1_latency = merge_byte(self.dom1_latency.into(), address, val).into()
            }

            0x04600018..=0x0460001B => {
                self.dom1_pulse_width =
                    merge_byte(self.dom1_pulse_width.into(), address, val).into()
            }

            0x0460001C..=0x0460001F => {
                self.dom1_page_size = merge_byte(self.dom1_page_size.into(), address, val).into()
            }

            0x04600020..=0x04600023 => {
                self.dom1_release = merge_byte(self.dom1_release.into(), address, val).into()
            }

            0x04600024..=0x04600027 => {
                self.dom2_latency = merge_byte(self.dom2_latency.into(), address, val).into()
            }

            0x04600028..=0x0460002B => {
                self.dom2_pulse_width =
                    merge_byte(self.dom2_pulse_width.into(), address, val).into()
            }

            0x0460002C..=0x0460002F => {
                self.dom2_page_size = merge_byte(self.dom2_page_size.into(), address, val).into()
            }

            0x04600030..=0x04600033 => {
                self.dom2_release = merge_byte(self.dom2_release.into(), address, val).into()
            }

            0x04600048..=0x0460004B => {
                self.flash_ctrl = merge_byte(self.flash_ctrl.into(), address, val).into();
                if (address & 3) == 3 {
                    //println!("{:#X?}\n{:08X}", self.flash_ctrl, self.flash_addr.addr());
                    if self.flash_ctrl.run() {
                        //self.flash_busy = true;
                        let nand_addr = self.flash_addr.addr() as usize;
                        let spare_addr = (nand_addr / 0x4000) * 0x10;
                        //println!("nand: {nand_addr:08X}, spare: {spare_addr:08X}");
                        let buf_addr = (self.flash_ctrl.buf() as usize) * 0x200;
                        let oob_addr = 0x400 + (self.flash_ctrl.buf() as usize) * 0x10;
                        self.buf[buf_addr..buf_addr + 0x200]
                            .copy_from_slice(&self.nand[nand_addr..nand_addr + 0x200]);
                        self.buf[oob_addr..oob_addr + 0x10]
                            .copy_from_slice(&self.spare[spare_addr..spare_addr + 0x10]);
                    }
                }
            }

            0x04600050..=0x04600053 => {
                self.aes_ctrl = merge_byte(self.aes_ctrl.into(), address, val).into();
                if (address & 3) == 3 {
                    //println!("{:#X?}", self.aes_ctrl);

                    if self.aes_ctrl.run() {
                        //self.aes_busy = true;
                        let iv = if self.aes_ctrl.chain() {
                            &self.last_block
                        } else {
                            let iv = self.aes_ctrl.iv() as usize;
                            &self.buf[iv * 16..(iv + 1) * 16]
                        }
                        .try_into()
                        .unwrap();
                        let key = &self.buf[0x4C0..0x4D0];
                        let enc_offset = self.aes_ctrl.data() as usize;
                        let enc_len = self.aes_ctrl.len() as usize;
                        let enc = &self.buf[enc_offset * 16..(enc_offset + enc_len + 1) * 16];
                        //println!("key: {key:02X?}, iv: {iv:02X?}");
                        let dec = aes_dec_cbc(enc, key, iv, None).expect("decryption failed");

                        self.last_block.copy_from_slice(
                            &enc[(enc_offset + enc_len) * 16..(enc_offset + enc_len + 1) * 16],
                        );

                        self.buf[enc_offset * 16..(enc_offset + enc_len + 1) * 16]
                            .copy_from_slice(&dec);
                    }
                }
            }

            0x04600054..=0x04600057 => {
                self.access = merge_byte(self.access.into(), address, val).into()
            }

            0x04600058..=0x0460005B => {
                self.buffer_read_len = merge_byte(self.buffer_read_len.into(), address, val).into();
                self.dma_busy = true;
            }

            0x0460005C..=0x0460005F => {
                self.buffer_write_len =
                    merge_byte(self.buffer_write_len.into(), address, val).into();
                self.dma_busy = true;
            }

            0x04600060..=0x04600063 => {
                self.gpio = merge_byte(self.gpio.into(), address, val).into();
                if (address & 3) == 0 {
                    println!(
                        "\n----------------\ngpio: power: {}, error: {}, user0: {}, user1: {}\n----------------\n",
                        self.gpio.data() & 0b0001 != 0,
                        self.gpio.data() & 0b0010 != 0,
                        self.gpio.data() & 0b0100 != 0,
                        self.gpio.data() & 0b1000 != 0,
                    );
                }
            }

            0x04600064..=0x04600067 => {
                self.ide_config = merge_byte(self.ide_config.into(), address, val).into()
            }

            0x04600070..=0x04600073 => {
                self.flash_addr = merge_byte(self.flash_addr.into(), address, val).into()
            }

            0x04610000..=0x046107FF => self.buf[(address - 0x04610000) as usize] = val,

            0x04680000..=0x04680003 => self.ide[0][(address & 3) as usize] = val,
            0x046A0000..=0x046A0003 => self.ide[1][(address & 3) as usize] = val,
            0x046C0000..=0x046C0003 => self.ide[2][(address & 3) as usize] = val,
            0x046E0000..=0x046E0003 => self.ide[3][(address & 3) as usize] = val,

            0x046FFFE0..=0x046FFFFF => {
                println!("sim IDE3 write: {address:08X} {val:02X}")
            }

            _ => {
                eprintln!("unimplemented PI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}

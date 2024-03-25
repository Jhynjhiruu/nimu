use crate::{types::*, SecureTrapType};

use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Mode {
    init_length: B7,
    clear_init_mode: bool,
    set_init_mode: bool,
    clear_ebus_test_mode: bool,
    set_ebus_test_mode: bool,
    clear_dp_interrupt: bool,
    #[skip]
    __: B20,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Intr {
    sp: bool,
    si: bool,
    ai: bool,
    vi: bool,
    pi: bool,
    dp: bool,
    #[skip]
    __: B26,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct IntrMask {
    sp: bool,
    si: bool,
    ai: bool,
    vi: bool,
    pi: bool,
    dp: bool,
    #[skip]
    __: B26,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct IntrMaskWrite {
    clear_sp: bool,
    set_sp: bool,
    clear_si: bool,
    set_si: bool,
    clear_ai: bool,
    set_ai: bool,
    clear_vi: bool,
    set_vi: bool,
    clear_pi: bool,
    set_pi: bool,
    clear_dp: bool,
    set_dp: bool,
    #[skip]
    __: B20,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Ctrl {
    #[skip]
    __: B8,
    clock_divider_mode: B3,
    cold_reset: bool,
    warm_reset: bool,
    bus_error_on_non_mem: bool,
    write_error_on_non_mem: bool,
    secure_trap_on_non_mem: bool,
    bus_error_on_pif: bool,
    write_error_on_pif: bool,
    secure_trap_on_pif: bool,
    #[skip]
    __: B13,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SecMode {
    secure_exit: bool,
    map: bool,
    app: bool,
    timer: bool,
    fatal: bool,
    emulation: bool,
    button: bool,
    enable_button: bool,
    enable_iram: bool,
    #[skip]
    __: B23,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct SecTimer {
    start: hword,
    prescale: hword,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct AVCtrl {
    standby: bool,
    divreset: bool,
    frange: B2,
    mdiv: B5,
    ndiv: B7,
    pdiv: B3,
    dac_power: bool,
    venc_vntpl: bool,
    venc_vmpal: bool,
    venc_vtrap: bool,
    venc_test: bool,
    pll_bypass: bool,
    av_reset: bool,
    #[skip]
    __: B6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct EIntr {
    #[skip]
    __: B6,
    pi_flash: bool,
    pi_aes: bool,
    pi_ide: bool,
    pi_error: bool,
    usb0: bool,
    usb1: bool,
    button: bool,
    module: bool,
    #[skip]
    __: B18,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct EIntrMask {
    #[skip]
    __: B6,
    pi_flash: bool,
    pi_aes: bool,
    pi_ide: bool,
    pi_error: bool,
    usb0: bool,
    usb1: bool,
    button: bool,
    module: bool,
    #[skip]
    __: B18,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct EIntrMaskWrite {
    #[skip]
    __: B12,
    clear_flash: bool,
    set_flash: bool,
    clear_aes: bool,
    set_aes: bool,
    clear_ide: bool,
    set_ide: bool,
    clear_error: bool,
    set_error: bool,
    clear_usb0: bool,
    set_usb0: bool,
    clear_usb1: bool,
    set_usb1: bool,
    clear_button: bool,
    set_button: bool,
    clear_module: bool,
    set_module: bool,
    #[skip]
    __: B4,
}

#[derive(Debug)]
pub struct Mi {
    mode: Mode,
    init_mode: bool,
    ebus_test_mode: bool,

    intr: Intr,
    intr_mask: IntrMask,

    ctrl: Ctrl,

    sec_mode: SecMode,

    sec_timer: SecTimer,
    sec_timer_count: hword,

    sec_vtimer: SecTimer,
    sec_vtimer_count: hword,

    pub mapping_changed: bool,

    av_control: AVCtrl,

    eintr: EIntr,
    eintr_mask: EIntrMask,

    raise_interrupt: bool,
    raise_extended_interrupt: bool,
}

impl Mi {
    pub fn new() -> Self {
        Self {
            mode: Mode::new(),
            init_mode: false,
            ebus_test_mode: false,
            intr: Intr::new(),
            intr_mask: IntrMask::new(),
            ctrl: Ctrl::new(),
            sec_mode: SecMode::new().with_map(true).with_secure_exit(true),
            sec_timer: SecTimer::new(),
            sec_timer_count: 0,
            sec_vtimer: SecTimer::new(),
            sec_vtimer_count: 0,
            mapping_changed: false,
            av_control: AVCtrl::new(),
            eintr: EIntr::new(),
            eintr_mask: EIntrMask::new(),
            raise_interrupt: false,
            raise_extended_interrupt: false,
        }
    }

    pub fn get_sec_mode_map(&self) -> bool {
        self.sec_mode.map()
    }

    pub fn is_secure_mode(&self) -> bool {
        self.sec_mode.secure_exit()
    }

    pub fn set_flash_intr(&mut self, state: bool) {
        self.eintr.set_pi_flash(state);
    }

    pub fn get_flash_intr(&self) -> bool {
        self.eintr.pi_flash()
    }

    pub fn set_vi_intr(&mut self, state: bool) {
        self.intr.set_vi(state);
    }

    pub fn set_pi_intr(&mut self, state: bool) {
        self.intr.set_pi(state);
    }

    pub fn set_md_intr(&mut self, state: bool) {
        self.eintr.set_module(state);
    }

    pub fn get_md_intr(&mut self) -> bool {
        self.eintr.module()
    }

    pub fn set_secure_trap(&mut self, trap: SecureTrapType) {
        match trap {
            SecureTrapType::Button => self.sec_mode.set_button(true),
            SecureTrapType::Emulation => self.sec_mode.set_emulation(true),
            SecureTrapType::Fatal => self.sec_mode.set_fatal(true),
            SecureTrapType::Timer => self.sec_mode.set_timer(true),
            SecureTrapType::App => self.sec_mode.set_app(true),
        }

        self.sec_mode.set_secure_exit(true);
    }

    pub fn has_interrupt(&self) -> bool {
        self.raise_interrupt
    }

    pub fn has_extended_interrupt(&self) -> bool {
        self.raise_extended_interrupt
    }

    pub fn get_intr(&self) -> u32 {
        self.intr.into()
    }

    pub fn get_eintr(&self) -> u32 {
        self.eintr.into()
    }

    pub fn read_phys_addr(&mut self, address: word) -> byte {
        match address {
            0x04300000..=0x04300003 => retrieve_byte(
                self.mode
                    .with_clear_init_mode(self.init_mode)
                    .with_set_init_mode(self.ebus_test_mode)
                    .into(),
                address,
            ),

            0x04300008..=0x0430000B => retrieve_byte(self.intr.into(), address),

            0x0430000C..=0x0430000F => retrieve_byte(self.intr_mask.into(), address),

            0x04300010..=0x04300013 => retrieve_byte(
                self.ctrl
                    .with_cold_reset(false)
                    .with_warm_reset(false)
                    .into(),
                address,
            ),

            0x04300014..=0x04300017 => retrieve_byte(self.sec_mode.into(), address),

            0x04300018..=0x0430001B => retrieve_byte(
                self.sec_timer.with_prescale(self.sec_timer_count).into(),
                address,
            ),

            0x0430001C..=0x0430001F => retrieve_byte(
                self.sec_vtimer.with_prescale(self.sec_vtimer_count).into(),
                address,
            ),

            0x04300030..=0x04300033 => retrieve_byte(self.av_control.into(), address),

            0x04300038..=0x0430003B => retrieve_byte(self.eintr.into(), address),

            0x0430003C..=0x0430003F => retrieve_byte(self.eintr_mask.into(), address),
            _ => {
                eprintln!("unimplemented MI read: {address:08X}");
                unimplemented!()
            }
        }
    }

    pub fn write_phys_addr(&mut self, address: word, val: byte) {
        match address {
            0x04300000..=0x04300003 => {
                self.mode = merge_byte(self.mode.into(), address, val).into();

                if self.mode.set_init_mode() {
                    self.mode.set_set_init_mode(false);
                    self.init_mode = true;
                }
                if self.mode.clear_init_mode() {
                    self.mode.set_clear_init_mode(false);
                    self.init_mode = false;
                }
                if self.mode.set_ebus_test_mode() {
                    self.mode.set_set_ebus_test_mode(false);
                    self.ebus_test_mode = true;
                }
                if self.mode.clear_ebus_test_mode() {
                    self.mode.set_clear_ebus_test_mode(false);
                    self.ebus_test_mode = false;
                }
                if self.mode.clear_dp_interrupt() {
                    self.mode.set_clear_dp_interrupt(false);
                    // clear the interrupt
                }
            }

            0x04300008..=0x0430000B => {
                self.intr = merge_byte(self.intr.into(), address, val).into()
            }

            0x0430000C..=0x0430000F => {
                let intr_mask_write: IntrMaskWrite = merge_byte(0, address, val).into();
                if intr_mask_write.clear_sp() {
                    self.intr_mask.set_sp(false);
                }
                if intr_mask_write.set_sp() {
                    self.intr_mask.set_sp(true);
                }
                if intr_mask_write.clear_si() {
                    self.intr_mask.set_si(false);
                }
                if intr_mask_write.set_si() {
                    self.intr_mask.set_si(true);
                }
                if intr_mask_write.clear_ai() {
                    self.intr_mask.set_ai(false);
                }
                if intr_mask_write.set_ai() {
                    self.intr_mask.set_ai(true);
                }
                if intr_mask_write.clear_vi() {
                    self.intr_mask.set_vi(false);
                }
                if intr_mask_write.set_vi() {
                    self.intr_mask.set_vi(true);
                }
                if intr_mask_write.clear_pi() {
                    self.intr_mask.set_pi(false);
                }
                if intr_mask_write.set_pi() {
                    self.intr_mask.set_pi(true);
                }
                if intr_mask_write.clear_dp() {
                    self.intr_mask.set_dp(false);
                }
                if intr_mask_write.set_dp() {
                    self.intr_mask.set_dp(true);
                }

                if address & 3 == 3 {
                    println!(
                        "write intr_mask: {:#?}, {:08X}",
                        self.intr_mask,
                        u32::from_le_bytes(self.intr_mask.bytes)
                    );
                }
            }

            0x04300010..=0x04300013 => {
                self.ctrl = merge_byte(self.ctrl.into(), address, val).into()
            }

            0x04300014..=0x04300017 => {
                let new_sec_mode: SecMode = merge_byte(self.sec_mode.into(), address, val).into();
                if new_sec_mode.map() != self.sec_mode.map() {
                    self.mapping_changed = true;
                }
                self.sec_mode = new_sec_mode;
            }

            0x04300018..=0x0430001B => {
                self.sec_timer = merge_byte(self.sec_timer.into(), address, val).into();
                self.sec_timer_count = self.sec_timer.prescale();
            }

            0x0430001C..=0x0430001F => {
                self.sec_vtimer = merge_byte(self.sec_vtimer.into(), address, val).into();
                self.sec_vtimer_count = self.sec_vtimer.prescale();
            }

            0x04300030..=0x04300033 => {
                self.av_control = merge_byte(self.av_control.into(), address, val).into()
            }

            0x04300038..=0x0430003B => {
                eprintln!("write to read_only register MI_EINTR_REG (0x04300038)")
            }

            0x0430003C..=0x0430003F => {
                let eintr_mask_write: EIntrMaskWrite = merge_byte(0, address, val).into();
                if eintr_mask_write.clear_flash() {
                    self.eintr_mask.set_pi_flash(false);
                }
                if eintr_mask_write.set_flash() {
                    self.eintr_mask.set_pi_flash(true);
                }
                if eintr_mask_write.clear_aes() {
                    self.eintr_mask.set_pi_aes(false);
                }
                if eintr_mask_write.set_aes() {
                    self.eintr_mask.set_pi_aes(true);
                }
                if eintr_mask_write.clear_ide() {
                    self.eintr_mask.set_pi_ide(false);
                }
                if eintr_mask_write.set_ide() {
                    self.eintr_mask.set_pi_ide(true);
                }
                if eintr_mask_write.clear_error() {
                    self.eintr_mask.set_pi_error(false);
                }
                if eintr_mask_write.set_error() {
                    self.eintr_mask.set_pi_error(true);
                }
                if eintr_mask_write.clear_usb0() {
                    self.eintr_mask.set_usb0(false);
                }
                if eintr_mask_write.set_usb0() {
                    self.eintr_mask.set_usb0(true);
                }
                if eintr_mask_write.clear_usb1() {
                    self.eintr_mask.set_usb1(false);
                }
                if eintr_mask_write.set_usb1() {
                    self.eintr_mask.set_usb1(true);
                }
                if eintr_mask_write.clear_button() {
                    self.eintr_mask.set_button(false);
                }
                if eintr_mask_write.set_button() {
                    self.eintr_mask.set_button(true);
                }
                if eintr_mask_write.clear_module() {
                    self.eintr_mask.set_module(false);
                }
                if eintr_mask_write.set_module() {
                    self.eintr_mask.set_module(true);
                }

                if address & 3 == 3 {
                    println!(
                        "write eintr_mask: {:#?}, {:08X}",
                        self.eintr_mask,
                        u32::from_le_bytes(self.eintr_mask.bytes)
                    );
                }
            }

            _ => {
                eprintln!("unimplemented MI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }

    pub fn step(&mut self) {
        let intr: u32 = self.intr.into();
        let mask: u32 = self.intr_mask.into();

        let eintr: u32 = self.eintr.into();
        let emask: u32 = self.eintr_mask.into();

        self.raise_interrupt = ((intr & mask) != 0);
        self.raise_extended_interrupt = ((eintr & emask) != 0);
    }
}

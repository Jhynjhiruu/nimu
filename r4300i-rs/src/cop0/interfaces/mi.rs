use crate::types::*;

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
    #[skip]
    __: B20,
}

#[derive(Debug)]
pub struct Mi {
    mode: Mode,
    init_mode: bool,
    ebus_test_mode: bool,

    intr_mask: IntrMask,

    ctrl: Ctrl,

    sec_mode: SecMode,

    sec_timer: SecTimer,
    sec_timer_count: hword,

    sec_vtimer: SecTimer,
    sec_vtimer_count: hword,

    pub mapping_changed: bool,

    eintr: EIntr,
    eintr_mask: EIntrMask,
}

impl Mi {
    pub fn new() -> Self {
        Self {
            mode: Mode::new(),
            init_mode: false,
            ebus_test_mode: false,
            intr_mask: IntrMask::new(),
            ctrl: Ctrl::new(),
            sec_mode: SecMode::new().with_map(true),
            sec_timer: SecTimer::new(),
            sec_timer_count: 0,
            sec_vtimer: SecTimer::new(),
            sec_vtimer_count: 0,
            mapping_changed: false,
            eintr: EIntr::new(),
            eintr_mask: EIntrMask::new(),
        }
    }

    pub fn get_sec_mode_map(&self) -> bool {
        self.sec_mode.map()
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

            0x04300038..=0x0430003B => {
                eprintln!("write to read_only register MI_EINTR_REG (0x04300038)")
            }

            0x0430003C..=0x0430003F => {
                self.eintr_mask = merge_byte(self.eintr_mask.into(), address, val).into()
            }

            _ => {
                eprintln!("unimplemented MI write: {address:08X} {val:02X}");
                unimplemented!()
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sbycr: Sbycr,
    _reserved1: [u8; 0x0e],
    mstpcra: Mstpcra,
    sckdivcr: Sckdivcr,
    _reserved3: [u8; 0x02],
    sckscr: Sckscr,
    _reserved4: [u8; 0x03],
    pllcr: Pllcr,
    pllccr2: Pllccr2,
    _reserved6: [u8; 0x05],
    memwait: Memwait,
    mosccr: Mosccr,
    _reserved8: [u8; 0x03],
    hococr: Hococr,
    _reserved9: [u8; 0x01],
    mococr: Mococr,
    _reserved10: [u8; 0x03],
    oscsf: Oscsf,
    _reserved11: [u8; 0x01],
    ckocr: Ckocr,
    trckcr: Trckcr,
    ostdcr: Ostdcr,
    ostdsr: Ostdsr,
    _reserved15: [u8; 0x0e],
    slcdsckcr: Slcdsckcr,
    _reserved16: [u8; 0x10],
    mocoutcr: Mocoutcr,
    hocoutcr: Hocoutcr,
    _reserved18: [u8; 0x2f],
    snzcr: Snzcr,
    _reserved19: [u8; 0x01],
    snzedcr: Snzedcr,
    _reserved20: [u8; 0x03],
    snzreqcr: Snzreqcr,
    _reserved21: [u8; 0x02],
    flstop: Flstop,
    _reserved22: [u8; 0x01],
    opccr: Opccr,
    _reserved23: [u8; 0x01],
    moscwtcr: Moscwtcr,
    _reserved24: [u8; 0x02],
    hocowtcr: Hocowtcr,
    _reserved25: [u8; 0x04],
    sopccr: Sopccr,
    _reserved26: [u8; 0x15],
    rstsr1: Rstsr1,
    _reserved27: [u8; 0x04],
    bkracr: Bkracr,
    _reserved28: [u8; 0x09],
    usbckcr: Usbckcr,
    _reserved29: [u8; 0x0f],
    lvdcr1: (),
    _reserved30: [u8; 0x01],
    lvdsr: (),
    _reserved31: [u8; 0x031d],
    prcr: Prcr,
    _reserved32: [u8; 0x0e],
    syocdcr: Syocdcr,
    _reserved33: [u8; 0x01],
    rstsr0: Rstsr0,
    rstsr2: Rstsr2,
    _reserved35: [u8; 0x01],
    momcr: Momcr,
    _reserved36: [u8; 0x03],
    lvcmpcr: Lvcmpcr,
    lvdlvlr: Lvdlvlr,
    _reserved38: [u8; 0x01],
    lvdcr0: [Lvdcr0; 2],
    _reserved39: [u8; 0x03],
    vbtcr1: Vbtcr1,
    _reserved40: [u8; 0x60],
    sosccr: Sosccr,
    somcr: Somcr,
    _reserved42: [u8; 0x0e],
    lococr: Lococr,
    _reserved43: [u8; 0x01],
    locoutcr: Locoutcr,
    _reserved44: [u8; 0x1d],
    vbtcr2: Vbtcr2,
    vbtsr: Vbtsr,
    vbtcmpcr: Vbtcmpcr,
    _reserved47: [u8; 0x01],
    vbtlvdicr: Vbtlvdicr,
    _reserved48: [u8; 0x01],
    vbtwctlr: Vbtwctlr,
    _reserved49: [u8; 0x01],
    vbtwch0otsr: Vbtwch0otsr,
    vbtwch1otsr: Vbtwch1otsr,
    vbtwch2otsr: Vbtwch2otsr,
    vbtictlr: Vbtictlr,
    vbtoctlr: Vbtoctlr,
    vbtwter: Vbtwter,
    vbtwegr: Vbtwegr,
    vbtwfr: Vbtwfr,
    _reserved57: [u8; 0x40],
    vbtbkr: [Vbtbkr; 512],
}
impl RegisterBlock {
    #[doc = "0x0c - Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &Sbycr {
        &self.sbycr
    }
    #[doc = "0x1c - Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(&self) -> &Mstpcra {
        &self.mstpcra
    }
    #[doc = "0x20 - System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(&self) -> &Sckdivcr {
        &self.sckdivcr
    }
    #[doc = "0x26 - System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(&self) -> &Sckscr {
        &self.sckscr
    }
    #[doc = "0x2a - PLL Control Register"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &Pllcr {
        &self.pllcr
    }
    #[doc = "0x2b - PLL Clock Control Register2"]
    #[inline(always)]
    pub const fn pllccr2(&self) -> &Pllccr2 {
        &self.pllccr2
    }
    #[doc = "0x31 - Memory Wait Cycle Control Register"]
    #[inline(always)]
    pub const fn memwait(&self) -> &Memwait {
        &self.memwait
    }
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(&self) -> &Mosccr {
        &self.mosccr
    }
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(&self) -> &Hococr {
        &self.hococr
    }
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(&self) -> &Mococr {
        &self.mococr
    }
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &Oscsf {
        &self.oscsf
    }
    #[doc = "0x3e - Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(&self) -> &Ckocr {
        &self.ckocr
    }
    #[doc = "0x3f - Trace Clock Control Register"]
    #[inline(always)]
    pub const fn trckcr(&self) -> &Trckcr {
        &self.trckcr
    }
    #[doc = "0x40 - Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn ostdcr(&self) -> &Ostdcr {
        &self.ostdcr
    }
    #[doc = "0x41 - Oscillation Stop Detection Status Register"]
    #[inline(always)]
    pub const fn ostdsr(&self) -> &Ostdsr {
        &self.ostdsr
    }
    #[doc = "0x50 - Segment LCD Source Clock Control Register"]
    #[inline(always)]
    pub const fn slcdsckcr(&self) -> &Slcdsckcr {
        &self.slcdsckcr
    }
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(&self) -> &Mocoutcr {
        &self.mocoutcr
    }
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(&self) -> &Hocoutcr {
        &self.hocoutcr
    }
    #[doc = "0x92 - Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(&self) -> &Snzcr {
        &self.snzcr
    }
    #[doc = "0x94 - Snooze End Control Register"]
    #[inline(always)]
    pub const fn snzedcr(&self) -> &Snzedcr {
        &self.snzedcr
    }
    #[doc = "0x98 - Snooze Request Control Register"]
    #[inline(always)]
    pub const fn snzreqcr(&self) -> &Snzreqcr {
        &self.snzreqcr
    }
    #[doc = "0x9e - Flash Operation Control Register"]
    #[inline(always)]
    pub const fn flstop(&self) -> &Flstop {
        &self.flstop
    }
    #[doc = "0xa0 - Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(&self) -> &Opccr {
        &self.opccr
    }
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn moscwtcr(&self) -> &Moscwtcr {
        &self.moscwtcr
    }
    #[doc = "0xa5 - High-Speed On-Chip Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn hocowtcr(&self) -> &Hocowtcr {
        &self.hocowtcr
    }
    #[doc = "0xaa - Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(&self) -> &Sopccr {
        &self.sopccr
    }
    #[doc = "0xc0 - Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(&self) -> &Rstsr1 {
        &self.rstsr1
    }
    #[doc = "0xc6 - Backup Register Access Control Register"]
    #[inline(always)]
    pub const fn bkracr(&self) -> &Bkracr {
        &self.bkracr
    }
    #[doc = "0xd0 - USB Clock Control register"]
    #[inline(always)]
    pub const fn usbckcr(&self) -> &Usbckcr {
        &self.usbckcr
    }
    #[doc = "0xe0 - Voltage Monitor %s Circuit Control Register 1"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `LVD1CR1` register.</div>"]
    #[inline(always)]
    pub const fn lvdcr1(&self, n: usize) -> &Lvdcr1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0 - Voltage Monitor %s Circuit Control Register 1"]
    #[inline(always)]
    pub fn lvdcr1_iter(&self) -> impl Iterator<Item = &Lvdcr1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0xe0 - Voltage Monitor 1 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd1cr1(&self) -> &Lvdcr1 {
        self.lvdcr1(0)
    }
    #[doc = "0xe2 - Voltage Monitor 2 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd2cr1(&self) -> &Lvdcr1 {
        self.lvdcr1(1)
    }
    #[doc = "0xe1 - Voltage Monitor %s Circuit Status Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `LVD1SR` register.</div>"]
    #[inline(always)]
    pub const fn lvdsr(&self, n: usize) -> &Lvdsr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(225)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe1 - Voltage Monitor %s Circuit Status Register"]
    #[inline(always)]
    pub fn lvdsr_iter(&self) -> impl Iterator<Item = &Lvdsr> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(225)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0xe1 - Voltage Monitor 1 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd1sr(&self) -> &Lvdsr {
        self.lvdsr(0)
    }
    #[doc = "0xe3 - Voltage Monitor 2 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd2sr(&self) -> &Lvdsr {
        self.lvdsr(1)
    }
    #[doc = "0x3fe - Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &Prcr {
        &self.prcr
    }
    #[doc = "0x40e - System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(&self) -> &Syocdcr {
        &self.syocdcr
    }
    #[doc = "0x410 - Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(&self) -> &Rstsr0 {
        &self.rstsr0
    }
    #[doc = "0x411 - Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(&self) -> &Rstsr2 {
        &self.rstsr2
    }
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(&self) -> &Momcr {
        &self.momcr
    }
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(&self) -> &Lvcmpcr {
        &self.lvcmpcr
    }
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(&self) -> &Lvdlvlr {
        &self.lvdlvlr
    }
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `LVD1CR0` register.</div>"]
    #[inline(always)]
    pub const fn lvdcr0(&self, n: usize) -> &Lvdcr0 {
        &self.lvdcr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub fn lvdcr0_iter(&self) -> impl Iterator<Item = &Lvdcr0> {
        self.lvdcr0.iter()
    }
    #[doc = "0x41a - Voltage Monitor 1 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd1cr0(&self) -> &Lvdcr0 {
        self.lvdcr0(0)
    }
    #[doc = "0x41b - Voltage Monitor 2 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd2cr0(&self) -> &Lvdcr0 {
        self.lvdcr0(1)
    }
    #[doc = "0x41f - VBATT Control Register1"]
    #[inline(always)]
    pub const fn vbtcr1(&self) -> &Vbtcr1 {
        &self.vbtcr1
    }
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(&self) -> &Sosccr {
        &self.sosccr
    }
    #[doc = "0x481 - Sub Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &Somcr {
        &self.somcr
    }
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(&self) -> &Lococr {
        &self.lococr
    }
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(&self) -> &Locoutcr {
        &self.locoutcr
    }
    #[doc = "0x4b0 - VBATT Control Register2"]
    #[inline(always)]
    pub const fn vbtcr2(&self) -> &Vbtcr2 {
        &self.vbtcr2
    }
    #[doc = "0x4b1 - VBATT Status Register"]
    #[inline(always)]
    pub const fn vbtsr(&self) -> &Vbtsr {
        &self.vbtsr
    }
    #[doc = "0x4b2 - VBATT Comparator Control Register"]
    #[inline(always)]
    pub const fn vbtcmpcr(&self) -> &Vbtcmpcr {
        &self.vbtcmpcr
    }
    #[doc = "0x4b4 - VBATT Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn vbtlvdicr(&self) -> &Vbtlvdicr {
        &self.vbtlvdicr
    }
    #[doc = "0x4b6 - VBATT Wakeup function Control Register"]
    #[inline(always)]
    pub const fn vbtwctlr(&self) -> &Vbtwctlr {
        &self.vbtwctlr
    }
    #[doc = "0x4b8 - VBATT Wakeup I/O 0 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch0otsr(&self) -> &Vbtwch0otsr {
        &self.vbtwch0otsr
    }
    #[doc = "0x4b9 - VBATT Wakeup I/O 1 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch1otsr(&self) -> &Vbtwch1otsr {
        &self.vbtwch1otsr
    }
    #[doc = "0x4ba - VBATT Wakeup I/O 2 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch2otsr(&self) -> &Vbtwch2otsr {
        &self.vbtwch2otsr
    }
    #[doc = "0x4bb - VBATT Input Control Register"]
    #[inline(always)]
    pub const fn vbtictlr(&self) -> &Vbtictlr {
        &self.vbtictlr
    }
    #[doc = "0x4bc - VBATT Output Control Register"]
    #[inline(always)]
    pub const fn vbtoctlr(&self) -> &Vbtoctlr {
        &self.vbtoctlr
    }
    #[doc = "0x4bd - VBATT Wakeup Trigger source Enable Register"]
    #[inline(always)]
    pub const fn vbtwter(&self) -> &Vbtwter {
        &self.vbtwter
    }
    #[doc = "0x4be - VBATT Wakeup Trigger source Edge Register"]
    #[inline(always)]
    pub const fn vbtwegr(&self) -> &Vbtwegr {
        &self.vbtwegr
    }
    #[doc = "0x4bf - VBATT Wakeup trigger source Flag Register"]
    #[inline(always)]
    pub const fn vbtwfr(&self) -> &Vbtwfr {
        &self.vbtwfr
    }
    #[doc = "0x500..0x700 - VBATT Backup Register \\[%s\\]"]
    #[inline(always)]
    pub const fn vbtbkr(&self, n: usize) -> &Vbtbkr {
        &self.vbtbkr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x700 - VBATT Backup Register \\[%s\\]"]
    #[inline(always)]
    pub fn vbtbkr_iter(&self) -> impl Iterator<Item = &Vbtbkr> {
        self.vbtbkr.iter()
    }
}
#[doc = "VBTCR1 (rw) register accessor: VBATT Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcr1`] module"]
#[doc(alias = "VBTCR1")]
pub type Vbtcr1 = crate::Reg<vbtcr1::Vbtcr1Spec>;
#[doc = "VBATT Control Register1"]
pub mod vbtcr1;
#[doc = "VBTCR2 (rw) register accessor: VBATT Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcr2`] module"]
#[doc(alias = "VBTCR2")]
pub type Vbtcr2 = crate::Reg<vbtcr2::Vbtcr2Spec>;
#[doc = "VBATT Control Register2"]
pub mod vbtcr2;
#[doc = "VBTSR (rw) register accessor: VBATT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtsr`] module"]
#[doc(alias = "VBTSR")]
pub type Vbtsr = crate::Reg<vbtsr::VbtsrSpec>;
#[doc = "VBATT Status Register"]
pub mod vbtsr;
#[doc = "VBTCMPCR (rw) register accessor: VBATT Comparator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcmpcr`] module"]
#[doc(alias = "VBTCMPCR")]
pub type Vbtcmpcr = crate::Reg<vbtcmpcr::VbtcmpcrSpec>;
#[doc = "VBATT Comparator Control Register"]
pub mod vbtcmpcr;
#[doc = "VBTLVDICR (rw) register accessor: VBATT Pin Low Voltage Detect Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtlvdicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtlvdicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtlvdicr`] module"]
#[doc(alias = "VBTLVDICR")]
pub type Vbtlvdicr = crate::Reg<vbtlvdicr::VbtlvdicrSpec>;
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
pub mod vbtlvdicr;
#[doc = "VBTWCTLR (rw) register accessor: VBATT Wakeup function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwctlr`] module"]
#[doc(alias = "VBTWCTLR")]
pub type Vbtwctlr = crate::Reg<vbtwctlr::VbtwctlrSpec>;
#[doc = "VBATT Wakeup function Control Register"]
pub mod vbtwctlr;
#[doc = "VBTWCH0OTSR (rw) register accessor: VBATT Wakeup I/O 0 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch0otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch0otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch0otsr`] module"]
#[doc(alias = "VBTWCH0OTSR")]
pub type Vbtwch0otsr = crate::Reg<vbtwch0otsr::Vbtwch0otsrSpec>;
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
pub mod vbtwch0otsr;
#[doc = "VBTWCH1OTSR (rw) register accessor: VBATT Wakeup I/O 1 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch1otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch1otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch1otsr`] module"]
#[doc(alias = "VBTWCH1OTSR")]
pub type Vbtwch1otsr = crate::Reg<vbtwch1otsr::Vbtwch1otsrSpec>;
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
pub mod vbtwch1otsr;
#[doc = "VBTWCH2OTSR (rw) register accessor: VBATT Wakeup I/O 2 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch2otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch2otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch2otsr`] module"]
#[doc(alias = "VBTWCH2OTSR")]
pub type Vbtwch2otsr = crate::Reg<vbtwch2otsr::Vbtwch2otsrSpec>;
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
pub mod vbtwch2otsr;
#[doc = "VBTICTLR (rw) register accessor: VBATT Input Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtictlr`] module"]
#[doc(alias = "VBTICTLR")]
pub type Vbtictlr = crate::Reg<vbtictlr::VbtictlrSpec>;
#[doc = "VBATT Input Control Register"]
pub mod vbtictlr;
#[doc = "VBTOCTLR (rw) register accessor: VBATT Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtoctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtoctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtoctlr`] module"]
#[doc(alias = "VBTOCTLR")]
pub type Vbtoctlr = crate::Reg<vbtoctlr::VbtoctlrSpec>;
#[doc = "VBATT Output Control Register"]
pub mod vbtoctlr;
#[doc = "VBTWTER (rw) register accessor: VBATT Wakeup Trigger source Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwter`] module"]
#[doc(alias = "VBTWTER")]
pub type Vbtwter = crate::Reg<vbtwter::VbtwterSpec>;
#[doc = "VBATT Wakeup Trigger source Enable Register"]
pub mod vbtwter;
#[doc = "VBTWEGR (rw) register accessor: VBATT Wakeup Trigger source Edge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwegr`] module"]
#[doc(alias = "VBTWEGR")]
pub type Vbtwegr = crate::Reg<vbtwegr::VbtwegrSpec>;
#[doc = "VBATT Wakeup Trigger source Edge Register"]
pub mod vbtwegr;
#[doc = "VBTWFR (rw) register accessor: VBATT Wakeup trigger source Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwfr`] module"]
#[doc(alias = "VBTWFR")]
pub type Vbtwfr = crate::Reg<vbtwfr::VbtwfrSpec>;
#[doc = "VBATT Wakeup trigger source Flag Register"]
pub mod vbtwfr;
#[doc = "VBTBKR (rw) register accessor: VBATT Backup Register \\[%s\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtbkr`] module"]
#[doc(alias = "VBTBKR")]
pub type Vbtbkr = crate::Reg<vbtbkr::VbtbkrSpec>;
#[doc = "VBATT Backup Register \\[%s\\]"]
pub mod vbtbkr;
#[doc = "SCKDIVCR (rw) register accessor: System Clock Division Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckdivcr`] module"]
#[doc(alias = "SCKDIVCR")]
pub type Sckdivcr = crate::Reg<sckdivcr::SckdivcrSpec>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKSCR (rw) register accessor: System Clock Source Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckscr`] module"]
#[doc(alias = "SCKSCR")]
pub type Sckscr = crate::Reg<sckscr::SckscrSpec>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "PLLCR (rw) register accessor: PLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcr`] module"]
#[doc(alias = "PLLCR")]
pub type Pllcr = crate::Reg<pllcr::PllcrSpec>;
#[doc = "PLL Control Register"]
pub mod pllcr;
#[doc = "PLLCCR2 (rw) register accessor: PLL Clock Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pllccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllccr2`] module"]
#[doc(alias = "PLLCCR2")]
pub type Pllccr2 = crate::Reg<pllccr2::Pllccr2Spec>;
#[doc = "PLL Clock Control Register2"]
pub mod pllccr2;
#[doc = "MEMWAIT (rw) register accessor: Memory Wait Cycle Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memwait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memwait`] module"]
#[doc(alias = "MEMWAIT")]
pub type Memwait = crate::Reg<memwait::MemwaitSpec>;
#[doc = "Memory Wait Cycle Control Register"]
pub mod memwait;
#[doc = "MOSCCR (rw) register accessor: Main Clock Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosccr`] module"]
#[doc(alias = "MOSCCR")]
pub type Mosccr = crate::Reg<mosccr::MosccrSpec>;
#[doc = "Main Clock Oscillator Control Register"]
pub mod mosccr;
#[doc = "HOCOCR (rw) register accessor: High-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hococr`] module"]
#[doc(alias = "HOCOCR")]
pub type Hococr = crate::Reg<hococr::HococrSpec>;
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub mod hococr;
#[doc = "MOCOCR (rw) register accessor: Middle-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mococr`] module"]
#[doc(alias = "MOCOCR")]
pub type Mococr = crate::Reg<mococr::MococrSpec>;
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub mod mococr;
#[doc = "OSCSF (r) register accessor: Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsf`] module"]
#[doc(alias = "OSCSF")]
pub type Oscsf = crate::Reg<oscsf::OscsfSpec>;
#[doc = "Oscillation Stabilization Flag Register"]
pub mod oscsf;
#[doc = "CKOCR (rw) register accessor: Clock Out Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckocr`] module"]
#[doc(alias = "CKOCR")]
pub type Ckocr = crate::Reg<ckocr::CkocrSpec>;
#[doc = "Clock Out Control Register"]
pub mod ckocr;
#[doc = "TRCKCR (rw) register accessor: Trace Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trckcr`] module"]
#[doc(alias = "TRCKCR")]
pub type Trckcr = crate::Reg<trckcr::TrckcrSpec>;
#[doc = "Trace Clock Control Register"]
pub mod trckcr;
#[doc = "OSTDCR (rw) register accessor: Oscillation Stop Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ostdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostdcr`] module"]
#[doc(alias = "OSTDCR")]
pub type Ostdcr = crate::Reg<ostdcr::OstdcrSpec>;
#[doc = "Oscillation Stop Detection Control Register"]
pub mod ostdcr;
#[doc = "OSTDSR (rw) register accessor: Oscillation Stop Detection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ostdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostdsr`] module"]
#[doc(alias = "OSTDSR")]
pub type Ostdsr = crate::Reg<ostdsr::OstdsrSpec>;
#[doc = "Oscillation Stop Detection Status Register"]
pub mod ostdsr;
#[doc = "SLCDSCKCR (rw) register accessor: Segment LCD Source Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slcdsckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slcdsckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slcdsckcr`] module"]
#[doc(alias = "SLCDSCKCR")]
pub type Slcdsckcr = crate::Reg<slcdsckcr::SlcdsckcrSpec>;
#[doc = "Segment LCD Source Clock Control Register"]
pub mod slcdsckcr;
#[doc = "MOCOUTCR (rw) register accessor: MOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mocoutcr`] module"]
#[doc(alias = "MOCOUTCR")]
pub type Mocoutcr = crate::Reg<mocoutcr::MocoutcrSpec>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: HOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hocoutcr`] module"]
#[doc(alias = "HOCOUTCR")]
pub type Hocoutcr = crate::Reg<hocoutcr::HocoutcrSpec>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "MOSCWTCR (rw) register accessor: Main Clock Oscillator Wait Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moscwtcr`] module"]
#[doc(alias = "MOSCWTCR")]
pub type Moscwtcr = crate::Reg<moscwtcr::MoscwtcrSpec>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "HOCOWTCR (rw) register accessor: High-Speed On-Chip Oscillator Wait Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hocowtcr`] module"]
#[doc(alias = "HOCOWTCR")]
pub type Hocowtcr = crate::Reg<hocowtcr::HocowtcrSpec>;
#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub mod hocowtcr;
#[doc = "USBCKCR (rw) register accessor: USB Clock Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbckcr`] module"]
#[doc(alias = "USBCKCR")]
pub type Usbckcr = crate::Reg<usbckcr::UsbckcrSpec>;
#[doc = "USB Clock Control register"]
pub mod usbckcr;
#[doc = "MOMCR (rw) register accessor: Main Clock Oscillator Mode Oscillation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`momcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@momcr`] module"]
#[doc(alias = "MOMCR")]
pub type Momcr = crate::Reg<momcr::MomcrSpec>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "SOSCCR (rw) register accessor: Sub-Clock Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosccr`] module"]
#[doc(alias = "SOSCCR")]
pub type Sosccr = crate::Reg<sosccr::SosccrSpec>;
#[doc = "Sub-Clock Oscillator Control Register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: Sub Clock Oscillator Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`somcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@somcr`] module"]
#[doc(alias = "SOMCR")]
pub type Somcr = crate::Reg<somcr::SomcrSpec>;
#[doc = "Sub Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "LOCOCR (rw) register accessor: Low-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lococr`] module"]
#[doc(alias = "LOCOCR")]
pub type Lococr = crate::Reg<lococr::LococrSpec>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: LOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`locoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locoutcr`] module"]
#[doc(alias = "LOCOUTCR")]
pub type Locoutcr = crate::Reg<locoutcr::LocoutcrSpec>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
#[doc = "SBYCR (rw) register accessor: Standby Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbycr`] module"]
#[doc(alias = "SBYCR")]
pub type Sbycr = crate::Reg<sbycr::SbycrSpec>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "MSTPCRA (rw) register accessor: Module Stop Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcra`] module"]
#[doc(alias = "MSTPCRA")]
pub type Mstpcra = crate::Reg<mstpcra::MstpcraSpec>;
#[doc = "Module Stop Control Register A"]
pub mod mstpcra;
#[doc = "SNZCR (rw) register accessor: Snooze Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzcr`] module"]
#[doc(alias = "SNZCR")]
pub type Snzcr = crate::Reg<snzcr::SnzcrSpec>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR (rw) register accessor: Snooze End Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzedcr`] module"]
#[doc(alias = "SNZEDCR")]
pub type Snzedcr = crate::Reg<snzedcr::SnzedcrSpec>;
#[doc = "Snooze End Control Register"]
pub mod snzedcr;
#[doc = "SNZREQCR (rw) register accessor: Snooze Request Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzreqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzreqcr`] module"]
#[doc(alias = "SNZREQCR")]
pub type Snzreqcr = crate::Reg<snzreqcr::SnzreqcrSpec>;
#[doc = "Snooze Request Control Register"]
pub mod snzreqcr;
#[doc = "FLSTOP (rw) register accessor: Flash Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flstop`] module"]
#[doc(alias = "FLSTOP")]
pub type Flstop = crate::Reg<flstop::FlstopSpec>;
#[doc = "Flash Operation Control Register"]
pub mod flstop;
#[doc = "OPCCR (rw) register accessor: Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opccr`] module"]
#[doc(alias = "OPCCR")]
pub type Opccr = crate::Reg<opccr::OpccrSpec>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "SOPCCR (rw) register accessor: Sub Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sopccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopccr`] module"]
#[doc(alias = "SOPCCR")]
pub type Sopccr = crate::Reg<sopccr::SopccrSpec>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "SYOCDCR (rw) register accessor: System Control OCD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syocdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syocdcr`] module"]
#[doc(alias = "SYOCDCR")]
pub type Syocdcr = crate::Reg<syocdcr::SyocdcrSpec>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
#[doc = "LVCMPCR (rw) register accessor: Voltage Monitor Circuit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvcmpcr`] module"]
#[doc(alias = "LVCMPCR")]
pub type Lvcmpcr = crate::Reg<lvcmpcr::LvcmpcrSpec>;
#[doc = "Voltage Monitor Circuit Control Register"]
pub mod lvcmpcr;
#[doc = "LVDLVLR (rw) register accessor: Voltage Detection Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdlvlr`] module"]
#[doc(alias = "LVDLVLR")]
pub type Lvdlvlr = crate::Reg<lvdlvlr::LvdlvlrSpec>;
#[doc = "Voltage Detection Level Select Register"]
pub mod lvdlvlr;
#[doc = "LVDCR0 (rw) register accessor: Voltage Monitor %s Circuit Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdcr0`] module"]
#[doc(alias = "LVDCR0")]
pub type Lvdcr0 = crate::Reg<lvdcr0::Lvdcr0Spec>;
#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub mod lvdcr0;
#[doc = "LVDCR1 (rw) register accessor: Voltage Monitor %s Circuit Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdcr1`] module"]
#[doc(alias = "LVDCR1")]
pub type Lvdcr1 = crate::Reg<lvdcr1::Lvdcr1Spec>;
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub mod lvdcr1;
#[doc = "LVDSR (rw) register accessor: Voltage Monitor %s Circuit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdsr`] module"]
#[doc(alias = "LVDSR")]
pub type Lvdsr = crate::Reg<lvdsr::LvdsrSpec>;
#[doc = "Voltage Monitor %s Circuit Status Register"]
pub mod lvdsr;
#[doc = "PRCR (rw) register accessor: Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prcr`] module"]
#[doc(alias = "PRCR")]
pub type Prcr = crate::Reg<prcr::PrcrSpec>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "RSTSR0 (rw) register accessor: Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr0`] module"]
#[doc(alias = "RSTSR0")]
pub type Rstsr0 = crate::Reg<rstsr0::Rstsr0Spec>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: Reset Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr2`] module"]
#[doc(alias = "RSTSR2")]
pub type Rstsr2 = crate::Reg<rstsr2::Rstsr2Spec>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "RSTSR1 (rw) register accessor: Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr1`] module"]
#[doc(alias = "RSTSR1")]
pub type Rstsr1 = crate::Reg<rstsr1::Rstsr1Spec>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
#[doc = "BKRACR (rw) register accessor: Backup Register Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkracr`] module"]
#[doc(alias = "BKRACR")]
pub type Bkracr = crate::Reg<bkracr::BkracrSpec>;
#[doc = "Backup Register Access Control Register"]
pub mod bkracr;

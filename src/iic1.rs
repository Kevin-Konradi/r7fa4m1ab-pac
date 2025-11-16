#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iccr1: Iccr1,
    iccr2: Iccr2,
    icmr1: Icmr1,
    icmr2: Icmr2,
    icmr3: Icmr3,
    icfer: Icfer,
    icser: Icser,
    icier: Icier,
    icsr1: Icsr1,
    icsr2: Icsr2,
    sarl: (),
    _reserved11: [u8; 0x01],
    saru: (),
    _reserved12: [u8; 0x05],
    icbrl: Icbrl,
    icbrh: Icbrh,
    icdrt: Icdrt,
    icdrr: Icdrr,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Bus Control Register 1"]
    #[inline(always)]
    pub const fn iccr1(&self) -> &Iccr1 {
        &self.iccr1
    }
    #[doc = "0x01 - I2C Bus Control Register 2"]
    #[inline(always)]
    pub const fn iccr2(&self) -> &Iccr2 {
        &self.iccr2
    }
    #[doc = "0x02 - I2C Bus Mode Register 1"]
    #[inline(always)]
    pub const fn icmr1(&self) -> &Icmr1 {
        &self.icmr1
    }
    #[doc = "0x03 - I2C Bus Mode Register 2"]
    #[inline(always)]
    pub const fn icmr2(&self) -> &Icmr2 {
        &self.icmr2
    }
    #[doc = "0x04 - I2C Bus Mode Register 3"]
    #[inline(always)]
    pub const fn icmr3(&self) -> &Icmr3 {
        &self.icmr3
    }
    #[doc = "0x05 - I2C Bus Function Enable Register"]
    #[inline(always)]
    pub const fn icfer(&self) -> &Icfer {
        &self.icfer
    }
    #[doc = "0x06 - I2C Bus Status Enable Register"]
    #[inline(always)]
    pub const fn icser(&self) -> &Icser {
        &self.icser
    }
    #[doc = "0x07 - I2C Bus Interrupt Enable Register"]
    #[inline(always)]
    pub const fn icier(&self) -> &Icier {
        &self.icier
    }
    #[doc = "0x08 - I2C Bus Status Register 1"]
    #[inline(always)]
    pub const fn icsr1(&self) -> &Icsr1 {
        &self.icsr1
    }
    #[doc = "0x09 - I2C Bus Status Register 2"]
    #[inline(always)]
    pub const fn icsr2(&self) -> &Icsr2 {
        &self.icsr2
    }
    #[doc = "0x0a - Slave Address Register L%s"]
    #[inline(always)]
    pub const fn sarl(&self, n: usize) -> &Sarl {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(10)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0a - Slave Address Register L%s"]
    #[inline(always)]
    pub fn sarl_iter(&self) -> impl Iterator<Item = &Sarl> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(10)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x0b - Slave Address Register U%s"]
    #[inline(always)]
    pub const fn saru(&self, n: usize) -> &Saru {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(11)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0b - Slave Address Register U%s"]
    #[inline(always)]
    pub fn saru_iter(&self) -> impl Iterator<Item = &Saru> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(11)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x10 - I2C Bus Bit Rate Low-Level Register"]
    #[inline(always)]
    pub const fn icbrl(&self) -> &Icbrl {
        &self.icbrl
    }
    #[doc = "0x11 - I2C Bus Bit Rate High-Level Register"]
    #[inline(always)]
    pub const fn icbrh(&self) -> &Icbrh {
        &self.icbrh
    }
    #[doc = "0x12 - I2C Bus Transmit Data Register"]
    #[inline(always)]
    pub const fn icdrt(&self) -> &Icdrt {
        &self.icdrt
    }
    #[doc = "0x13 - I2C Bus Receive Data Register"]
    #[inline(always)]
    pub const fn icdrr(&self) -> &Icdrr {
        &self.icdrr
    }
}
#[doc = "ICCR1 (rw) register accessor: I2C Bus Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`iccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccr1`] module"]
#[doc(alias = "ICCR1")]
pub type Iccr1 = crate::Reg<iccr1::Iccr1Spec>;
#[doc = "I2C Bus Control Register 1"]
pub mod iccr1;
#[doc = "ICCR2 (rw) register accessor: I2C Bus Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`iccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccr2`] module"]
#[doc(alias = "ICCR2")]
pub type Iccr2 = crate::Reg<iccr2::Iccr2Spec>;
#[doc = "I2C Bus Control Register 2"]
pub mod iccr2;
#[doc = "ICMR1 (rw) register accessor: I2C Bus Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr1`] module"]
#[doc(alias = "ICMR1")]
pub type Icmr1 = crate::Reg<icmr1::Icmr1Spec>;
#[doc = "I2C Bus Mode Register 1"]
pub mod icmr1;
#[doc = "ICMR2 (rw) register accessor: I2C Bus Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr2`] module"]
#[doc(alias = "ICMR2")]
pub type Icmr2 = crate::Reg<icmr2::Icmr2Spec>;
#[doc = "I2C Bus Mode Register 2"]
pub mod icmr2;
#[doc = "ICMR3 (rw) register accessor: I2C Bus Mode Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr3`] module"]
#[doc(alias = "ICMR3")]
pub type Icmr3 = crate::Reg<icmr3::Icmr3Spec>;
#[doc = "I2C Bus Mode Register 3"]
pub mod icmr3;
#[doc = "ICFER (rw) register accessor: I2C Bus Function Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icfer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icfer`] module"]
#[doc(alias = "ICFER")]
pub type Icfer = crate::Reg<icfer::IcferSpec>;
#[doc = "I2C Bus Function Enable Register"]
pub mod icfer;
#[doc = "ICSER (rw) register accessor: I2C Bus Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icser`] module"]
#[doc(alias = "ICSER")]
pub type Icser = crate::Reg<icser::IcserSpec>;
#[doc = "I2C Bus Status Enable Register"]
pub mod icser;
#[doc = "ICIER (rw) register accessor: I2C Bus Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icier`] module"]
#[doc(alias = "ICIER")]
pub type Icier = crate::Reg<icier::IcierSpec>;
#[doc = "I2C Bus Interrupt Enable Register"]
pub mod icier;
#[doc = "ICSR1 (rw) register accessor: I2C Bus Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr1`] module"]
#[doc(alias = "ICSR1")]
pub type Icsr1 = crate::Reg<icsr1::Icsr1Spec>;
#[doc = "I2C Bus Status Register 1"]
pub mod icsr1;
#[doc = "ICSR2 (rw) register accessor: I2C Bus Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr2`] module"]
#[doc(alias = "ICSR2")]
pub type Icsr2 = crate::Reg<icsr2::Icsr2Spec>;
#[doc = "I2C Bus Status Register 2"]
pub mod icsr2;
#[doc = "SARL (rw) register accessor: Slave Address Register L%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sarl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sarl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sarl`] module"]
#[doc(alias = "SARL")]
pub type Sarl = crate::Reg<sarl::SarlSpec>;
#[doc = "Slave Address Register L%s"]
pub mod sarl;
#[doc = "SARU (rw) register accessor: Slave Address Register U%s\n\nYou can [`read`](crate::Reg::read) this register and get [`saru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saru`] module"]
#[doc(alias = "SARU")]
pub type Saru = crate::Reg<saru::SaruSpec>;
#[doc = "Slave Address Register U%s"]
pub mod saru;
#[doc = "ICBRL (rw) register accessor: I2C Bus Bit Rate Low-Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icbrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icbrl`] module"]
#[doc(alias = "ICBRL")]
pub type Icbrl = crate::Reg<icbrl::IcbrlSpec>;
#[doc = "I2C Bus Bit Rate Low-Level Register"]
pub mod icbrl;
#[doc = "ICBRH (rw) register accessor: I2C Bus Bit Rate High-Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icbrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icbrh`] module"]
#[doc(alias = "ICBRH")]
pub type Icbrh = crate::Reg<icbrh::IcbrhSpec>;
#[doc = "I2C Bus Bit Rate High-Level Register"]
pub mod icbrh;
#[doc = "ICDRT (rw) register accessor: I2C Bus Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdrt`] module"]
#[doc(alias = "ICDRT")]
pub type Icdrt = crate::Reg<icdrt::IcdrtSpec>;
#[doc = "I2C Bus Transmit Data Register"]
pub mod icdrt;
#[doc = "ICDRR (r) register accessor: I2C Bus Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdrr`] module"]
#[doc(alias = "ICDRR")]
pub type Icdrr = crate::Reg<icdrr::IcdrrSpec>;
#[doc = "I2C Bus Receive Data Register"]
pub mod icdrr;

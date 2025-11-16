#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_eidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
    _reserved_3_eorr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Output data register"]
    #[inline(always)]
    pub const fn podr(&self) -> &Podr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Port Control Register 1"]
    #[inline(always)]
    pub const fn pcntr1(&self) -> &Pcntr1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - Data direction register"]
    #[inline(always)]
    pub const fn pdr(&self) -> &Pdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x04 - Event input data register"]
    #[inline(always)]
    pub const fn eidr(&self) -> &Eidr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Port Control Register 2"]
    #[inline(always)]
    pub const fn pcntr2(&self) -> &Pcntr2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - Input data register"]
    #[inline(always)]
    pub const fn pidr(&self) -> &Pidr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - Output set register"]
    #[inline(always)]
    pub const fn porr(&self) -> &Porr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Port Control Register 3"]
    #[inline(always)]
    pub const fn pcntr3(&self) -> &Pcntr3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - Output reset register"]
    #[inline(always)]
    pub const fn posr(&self) -> &Posr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0c - Event output set register"]
    #[inline(always)]
    pub const fn eorr(&self) -> &Eorr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Port Control Register 4"]
    #[inline(always)]
    pub const fn pcntr4(&self) -> &Pcntr4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - Event output reset register"]
    #[inline(always)]
    pub const fn eosr(&self) -> &Eosr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
}
#[doc = "PCNTR1 (rw) register accessor: Port Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr1`] module"]
#[doc(alias = "PCNTR1")]
pub type Pcntr1 = crate::Reg<pcntr1::Pcntr1Spec>;
#[doc = "Port Control Register 1"]
pub mod pcntr1;
#[doc = "PODR (rw) register accessor: Output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`podr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podr`] module"]
#[doc(alias = "PODR")]
pub type Podr = crate::Reg<podr::PodrSpec>;
#[doc = "Output data register"]
pub mod podr;
#[doc = "PDR (rw) register accessor: Data direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`] module"]
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
#[doc = "Data direction register"]
pub mod pdr;
#[doc = "PCNTR2 (r) register accessor: Port Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr2`] module"]
#[doc(alias = "PCNTR2")]
pub type Pcntr2 = crate::Reg<pcntr2::Pcntr2Spec>;
#[doc = "Port Control Register 2"]
pub mod pcntr2;
#[doc = "EIDR (r) register accessor: Event input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`eidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eidr`] module"]
#[doc(alias = "EIDR")]
pub type Eidr = crate::Reg<eidr::EidrSpec>;
#[doc = "Event input data register"]
pub mod eidr;
#[doc = "PIDR (r) register accessor: Input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr`] module"]
#[doc(alias = "PIDR")]
pub type Pidr = crate::Reg<pidr::PidrSpec>;
#[doc = "Input data register"]
pub mod pidr;
#[doc = "PCNTR3 (w) register accessor: Port Control Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr3`] module"]
#[doc(alias = "PCNTR3")]
pub type Pcntr3 = crate::Reg<pcntr3::Pcntr3Spec>;
#[doc = "Port Control Register 3"]
pub mod pcntr3;
#[doc = "PORR (w) register accessor: Output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porr`] module"]
#[doc(alias = "PORR")]
pub type Porr = crate::Reg<porr::PorrSpec>;
#[doc = "Output set register"]
pub mod porr;
#[doc = "POSR (w) register accessor: Output reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posr`] module"]
#[doc(alias = "POSR")]
pub type Posr = crate::Reg<posr::PosrSpec>;
#[doc = "Output reset register"]
pub mod posr;
#[doc = "PCNTR4 (rw) register accessor: Port Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr4`] module"]
#[doc(alias = "PCNTR4")]
pub type Pcntr4 = crate::Reg<pcntr4::Pcntr4Spec>;
#[doc = "Port Control Register 4"]
pub mod pcntr4;
#[doc = "EORR (rw) register accessor: Event output set register\n\nYou can [`read`](crate::Reg::read) this register and get [`eorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eorr`] module"]
#[doc(alias = "EORR")]
pub type Eorr = crate::Reg<eorr::EorrSpec>;
#[doc = "Event output set register"]
pub mod eorr;
#[doc = "EOSR (rw) register accessor: Event output reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`eosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eosr`] module"]
#[doc(alias = "EOSR")]
pub type Eosr = crate::Reg<eosr::EosrSpec>;
#[doc = "Event output reset register"]
pub mod eosr;

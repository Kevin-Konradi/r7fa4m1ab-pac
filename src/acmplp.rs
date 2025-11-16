#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    compmdr: Compmdr,
    compfir: Compfir,
    compocr: Compocr,
    _reserved3: [u8; 0x01],
    compsel0: Compsel0,
    compsel1: Compsel1,
}
impl RegisterBlock {
    #[doc = "0x00 - ACMPLP Mode Setting Register"]
    #[inline(always)]
    pub const fn compmdr(&self) -> &Compmdr {
        &self.compmdr
    }
    #[doc = "0x01 - ACMPLP Filter Control Register"]
    #[inline(always)]
    pub const fn compfir(&self) -> &Compfir {
        &self.compfir
    }
    #[doc = "0x02 - ACMPLP Output Control Register"]
    #[inline(always)]
    pub const fn compocr(&self) -> &Compocr {
        &self.compocr
    }
    #[doc = "0x04 - Comparator Input Select Register"]
    #[inline(always)]
    pub const fn compsel0(&self) -> &Compsel0 {
        &self.compsel0
    }
    #[doc = "0x05 - Comparator Reference Voltage Select Register"]
    #[inline(always)]
    pub const fn compsel1(&self) -> &Compsel1 {
        &self.compsel1
    }
}
#[doc = "COMPMDR (rw) register accessor: ACMPLP Mode Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compmdr`] module"]
#[doc(alias = "COMPMDR")]
pub type Compmdr = crate::Reg<compmdr::CompmdrSpec>;
#[doc = "ACMPLP Mode Setting Register"]
pub mod compmdr;
#[doc = "COMPFIR (rw) register accessor: ACMPLP Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compfir`] module"]
#[doc(alias = "COMPFIR")]
pub type Compfir = crate::Reg<compfir::CompfirSpec>;
#[doc = "ACMPLP Filter Control Register"]
pub mod compfir;
#[doc = "COMPOCR (rw) register accessor: ACMPLP Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compocr`] module"]
#[doc(alias = "COMPOCR")]
pub type Compocr = crate::Reg<compocr::CompocrSpec>;
#[doc = "ACMPLP Output Control Register"]
pub mod compocr;
#[doc = "COMPSEL0 (rw) register accessor: Comparator Input Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compsel0`] module"]
#[doc(alias = "COMPSEL0")]
pub type Compsel0 = crate::Reg<compsel0::Compsel0Spec>;
#[doc = "Comparator Input Select Register"]
pub mod compsel0;
#[doc = "COMPSEL1 (rw) register accessor: Comparator Reference Voltage Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compsel1`] module"]
#[doc(alias = "COMPSEL1")]
pub type Compsel1 = crate::Reg<compsel1::Compsel1Spec>;
#[doc = "Comparator Reference Voltage Select Register"]
pub mod compsel1;

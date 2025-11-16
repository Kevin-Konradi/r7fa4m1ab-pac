#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    docr: Docr,
    _reserved1: [u8; 0x01],
    dodir: Dodir,
    dodsr: Dodsr,
}
impl RegisterBlock {
    #[doc = "0x00 - DOC Control Register"]
    #[inline(always)]
    pub const fn docr(&self) -> &Docr {
        &self.docr
    }
    #[doc = "0x02 - DOC Data Input Register"]
    #[inline(always)]
    pub const fn dodir(&self) -> &Dodir {
        &self.dodir
    }
    #[doc = "0x04 - DOC Data Setting Register"]
    #[inline(always)]
    pub const fn dodsr(&self) -> &Dodsr {
        &self.dodsr
    }
}
#[doc = "DOCR (rw) register accessor: DOC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`docr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@docr`] module"]
#[doc(alias = "DOCR")]
pub type Docr = crate::Reg<docr::DocrSpec>;
#[doc = "DOC Control Register"]
pub mod docr;
#[doc = "DODIR (rw) register accessor: DOC Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dodir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodir`] module"]
#[doc(alias = "DODIR")]
pub type Dodir = crate::Reg<dodir::DodirSpec>;
#[doc = "DOC Data Input Register"]
pub mod dodir;
#[doc = "DODSR (rw) register accessor: DOC Data Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dodsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodsr`] module"]
#[doc(alias = "DODSR")]
pub type Dodsr = crate::Reg<dodsr::DodsrSpec>;
#[doc = "DOC Data Setting Register"]
pub mod dodsr;

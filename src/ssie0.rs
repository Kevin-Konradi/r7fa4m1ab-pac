#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ssicr: Ssicr,
    ssisr: Ssisr,
    _reserved2: [u8; 0x08],
    ssifcr: Ssifcr,
    ssifsr: Ssifsr,
    ssiftdr: Ssiftdr,
    ssifrdr: Ssifrdr,
    ssitdmr: Ssitdmr,
    ssiscr: Ssiscr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ssicr(&self) -> &Ssicr {
        &self.ssicr
    }
    #[doc = "0x04 - Status Register"]
    #[inline(always)]
    pub const fn ssisr(&self) -> &Ssisr {
        &self.ssisr
    }
    #[doc = "0x10 - FIFO Control Register"]
    #[inline(always)]
    pub const fn ssifcr(&self) -> &Ssifcr {
        &self.ssifcr
    }
    #[doc = "0x14 - FIFO Status Register"]
    #[inline(always)]
    pub const fn ssifsr(&self) -> &Ssifsr {
        &self.ssifsr
    }
    #[doc = "0x18 - Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ssiftdr(&self) -> &Ssiftdr {
        &self.ssiftdr
    }
    #[doc = "0x1c - Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn ssifrdr(&self) -> &Ssifrdr {
        &self.ssifrdr
    }
    #[doc = "0x20 - TDM Mode Register"]
    #[inline(always)]
    pub const fn ssitdmr(&self) -> &Ssitdmr {
        &self.ssitdmr
    }
    #[doc = "0x24 - Status Control Register"]
    #[inline(always)]
    pub const fn ssiscr(&self) -> &Ssiscr {
        &self.ssiscr
    }
}
#[doc = "SSICR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssicr`] module"]
#[doc(alias = "SSICR")]
pub type Ssicr = crate::Reg<ssicr::SsicrSpec>;
#[doc = "Control Register"]
pub mod ssicr;
#[doc = "SSISR (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssisr`] module"]
#[doc(alias = "SSISR")]
pub type Ssisr = crate::Reg<ssisr::SsisrSpec>;
#[doc = "Status Register"]
pub mod ssisr;
#[doc = "SSIFCR (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifcr`] module"]
#[doc(alias = "SSIFCR")]
pub type Ssifcr = crate::Reg<ssifcr::SsifcrSpec>;
#[doc = "FIFO Control Register"]
pub mod ssifcr;
#[doc = "SSIFSR (rw) register accessor: FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifsr`] module"]
#[doc(alias = "SSIFSR")]
pub type Ssifsr = crate::Reg<ssifsr::SsifsrSpec>;
#[doc = "FIFO Status Register"]
pub mod ssifsr;
#[doc = "SSIFTDR (w) register accessor: Transmit FIFO Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiftdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiftdr`] module"]
#[doc(alias = "SSIFTDR")]
pub type Ssiftdr = crate::Reg<ssiftdr::SsiftdrSpec>;
#[doc = "Transmit FIFO Data Register"]
pub mod ssiftdr;
#[doc = "SSIFRDR (r) register accessor: Receive FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifrdr`] module"]
#[doc(alias = "SSIFRDR")]
pub type Ssifrdr = crate::Reg<ssifrdr::SsifrdrSpec>;
#[doc = "Receive FIFO Data Register"]
pub mod ssifrdr;
#[doc = "SSITDMR (rw) register accessor: TDM Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssitdmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssitdmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssitdmr`] module"]
#[doc(alias = "SSITDMR")]
pub type Ssitdmr = crate::Reg<ssitdmr::SsitdmrSpec>;
#[doc = "TDM Mode Register"]
pub mod ssitdmr;
#[doc = "SSISCR (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiscr`] module"]
#[doc(alias = "SSISCR")]
pub type Ssiscr = crate::Reg<ssiscr::SsiscrSpec>;
#[doc = "Status Control Register"]
pub mod ssiscr;

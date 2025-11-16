#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdtrr: Wdtrr,
    _reserved1: [u8; 0x01],
    wdtcr: Wdtcr,
    wdtsr: Wdtsr,
    wdtrcr: Wdtrcr,
    _reserved4: [u8; 0x01],
    wdtcstpr: Wdtcstpr,
}
impl RegisterBlock {
    #[doc = "0x00 - WDT Refresh Register"]
    #[inline(always)]
    pub const fn wdtrr(&self) -> &Wdtrr {
        &self.wdtrr
    }
    #[doc = "0x02 - WDT Control Register"]
    #[inline(always)]
    pub const fn wdtcr(&self) -> &Wdtcr {
        &self.wdtcr
    }
    #[doc = "0x04 - WDT Status Register"]
    #[inline(always)]
    pub const fn wdtsr(&self) -> &Wdtsr {
        &self.wdtsr
    }
    #[doc = "0x06 - WDT Reset Control Register"]
    #[inline(always)]
    pub const fn wdtrcr(&self) -> &Wdtrcr {
        &self.wdtrcr
    }
    #[doc = "0x08 - WDT Count Stop Control Register"]
    #[inline(always)]
    pub const fn wdtcstpr(&self) -> &Wdtcstpr {
        &self.wdtcstpr
    }
}
#[doc = "WDTRR (rw) register accessor: WDT Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtrr`] module"]
#[doc(alias = "WDTRR")]
pub type Wdtrr = crate::Reg<wdtrr::WdtrrSpec>;
#[doc = "WDT Refresh Register"]
pub mod wdtrr;
#[doc = "WDTCR (rw) register accessor: WDT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcr`] module"]
#[doc(alias = "WDTCR")]
pub type Wdtcr = crate::Reg<wdtcr::WdtcrSpec>;
#[doc = "WDT Control Register"]
pub mod wdtcr;
#[doc = "WDTSR (rw) register accessor: WDT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtsr`] module"]
#[doc(alias = "WDTSR")]
pub type Wdtsr = crate::Reg<wdtsr::WdtsrSpec>;
#[doc = "WDT Status Register"]
pub mod wdtsr;
#[doc = "WDTRCR (rw) register accessor: WDT Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtrcr`] module"]
#[doc(alias = "WDTRCR")]
pub type Wdtrcr = crate::Reg<wdtrcr::WdtrcrSpec>;
#[doc = "WDT Reset Control Register"]
pub mod wdtrcr;
#[doc = "WDTCSTPR (rw) register accessor: WDT Count Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcstpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcstpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcstpr`] module"]
#[doc(alias = "WDTCSTPR")]
pub type Wdtcstpr = crate::Reg<wdtcstpr::WdtcstprSpec>;
#[doc = "WDT Count Stop Control Register"]
pub mod wdtcstpr;

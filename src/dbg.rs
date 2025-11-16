#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbgstr: Dbgstr,
    _reserved1: [u8; 0x0c],
    dbgstopcr: Dbgstopcr,
    _reserved2: [u8; 0x0c],
    tracectr: Tracectr,
}
impl RegisterBlock {
    #[doc = "0x00 - Debug Status Register"]
    #[inline(always)]
    pub const fn dbgstr(&self) -> &Dbgstr {
        &self.dbgstr
    }
    #[doc = "0x10 - Debug Stop Control Register"]
    #[inline(always)]
    pub const fn dbgstopcr(&self) -> &Dbgstopcr {
        &self.dbgstopcr
    }
    #[doc = "0x20 - Trace Control Register"]
    #[inline(always)]
    pub const fn tracectr(&self) -> &Tracectr {
        &self.tracectr
    }
}
#[doc = "DBGSTR (r) register accessor: Debug Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstr`] module"]
#[doc(alias = "DBGSTR")]
pub type Dbgstr = crate::Reg<dbgstr::DbgstrSpec>;
#[doc = "Debug Status Register"]
pub mod dbgstr;
#[doc = "DBGSTOPCR (rw) register accessor: Debug Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstopcr`] module"]
#[doc(alias = "DBGSTOPCR")]
pub type Dbgstopcr = crate::Reg<dbgstopcr::DbgstopcrSpec>;
#[doc = "Debug Stop Control Register"]
pub mod dbgstopcr;
#[doc = "TRACECTR (rw) register accessor: Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tracectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracectr`] module"]
#[doc(alias = "TRACECTR")]
pub type Tracectr = crate::Reg<tracectr::TracectrSpec>;
#[doc = "Trace Control Register"]
pub mod tracectr;

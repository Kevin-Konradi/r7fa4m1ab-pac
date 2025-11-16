#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    fcachee: Fcachee,
    _reserved1: [u8; 0x02],
    fcacheiv: Fcacheiv,
    _reserved2: [u8; 0x16],
    flwt: Flwt,
}
impl RegisterBlock {
    #[doc = "0x100 - Flash Cache Enable Register"]
    #[inline(always)]
    pub const fn fcachee(&self) -> &Fcachee {
        &self.fcachee
    }
    #[doc = "0x104 - Flash Cache Invalidate Register"]
    #[inline(always)]
    pub const fn fcacheiv(&self) -> &Fcacheiv {
        &self.fcacheiv
    }
    #[doc = "0x11c - Flash Wait Cycle Register"]
    #[inline(always)]
    pub const fn flwt(&self) -> &Flwt {
        &self.flwt
    }
}
#[doc = "FCACHEE (rw) register accessor: Flash Cache Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcachee::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcachee::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcachee`] module"]
#[doc(alias = "FCACHEE")]
pub type Fcachee = crate::Reg<fcachee::FcacheeSpec>;
#[doc = "Flash Cache Enable Register"]
pub mod fcachee;
#[doc = "FCACHEIV (rw) register accessor: Flash Cache Invalidate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcacheiv`] module"]
#[doc(alias = "FCACHEIV")]
pub type Fcacheiv = crate::Reg<fcacheiv::FcacheivSpec>;
#[doc = "Flash Cache Invalidate Register"]
pub mod fcacheiv;
#[doc = "FLWT (rw) register accessor: Flash Wait Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flwt`] module"]
#[doc(alias = "FLWT")]
pub type Flwt = crate::Reg<flwt::FlwtSpec>;
#[doc = "Flash Wait Cycle Register"]
pub mod flwt;

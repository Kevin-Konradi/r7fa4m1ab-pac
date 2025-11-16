#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dadr0: Dadr0,
    _reserved1: [u8; 0x02],
    dacr: Dacr,
    dadpr: Dadpr,
    daadscr: Daadscr,
    davrefcr: Davrefcr,
}
impl RegisterBlock {
    #[doc = "0x00 - D/A Data Register 0"]
    #[inline(always)]
    pub const fn dadr0(&self) -> &Dadr0 {
        &self.dadr0
    }
    #[doc = "0x04 - D/A Control Register"]
    #[inline(always)]
    pub const fn dacr(&self) -> &Dacr {
        &self.dacr
    }
    #[doc = "0x05 - DADR0 Format Select Register"]
    #[inline(always)]
    pub const fn dadpr(&self) -> &Dadpr {
        &self.dadpr
    }
    #[doc = "0x06 - D/A-A/D Synchronous Start Control Register"]
    #[inline(always)]
    pub const fn daadscr(&self) -> &Daadscr {
        &self.daadscr
    }
    #[doc = "0x07 - D/A VREF Control Register"]
    #[inline(always)]
    pub const fn davrefcr(&self) -> &Davrefcr {
        &self.davrefcr
    }
}
#[doc = "DADR0 (rw) register accessor: D/A Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr0`] module"]
#[doc(alias = "DADR0")]
pub type Dadr0 = crate::Reg<dadr0::Dadr0Spec>;
#[doc = "D/A Data Register 0"]
pub mod dadr0;
#[doc = "DACR (rw) register accessor: D/A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacr`] module"]
#[doc(alias = "DACR")]
pub type Dacr = crate::Reg<dacr::DacrSpec>;
#[doc = "D/A Control Register"]
pub mod dacr;
#[doc = "DADPR (rw) register accessor: DADR0 Format Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dadpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadpr`] module"]
#[doc(alias = "DADPR")]
pub type Dadpr = crate::Reg<dadpr::DadprSpec>;
#[doc = "DADR0 Format Select Register"]
pub mod dadpr;
#[doc = "DAADSCR (rw) register accessor: D/A-A/D Synchronous Start Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daadscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daadscr`] module"]
#[doc(alias = "DAADSCR")]
pub type Daadscr = crate::Reg<daadscr::DaadscrSpec>;
#[doc = "D/A-A/D Synchronous Start Control Register"]
pub mod daadscr;
#[doc = "DAVREFCR (rw) register accessor: D/A VREF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`davrefcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`davrefcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@davrefcr`] module"]
#[doc(alias = "DAVREFCR")]
pub type Davrefcr = crate::Reg<davrefcr::DavrefcrSpec>;
#[doc = "D/A VREF Control Register"]
pub mod davrefcr;

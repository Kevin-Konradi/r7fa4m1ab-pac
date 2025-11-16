#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dtccr: Dtccr,
    _reserved1: [u8; 0x03],
    dtcvbr: Dtcvbr,
    _reserved2: [u8; 0x04],
    dtcst: Dtcst,
    _reserved3: [u8; 0x01],
    dtcsts: Dtcsts,
}
impl RegisterBlock {
    #[doc = "0x00 - DTC Control Register"]
    #[inline(always)]
    pub const fn dtccr(&self) -> &Dtccr {
        &self.dtccr
    }
    #[doc = "0x04 - DTC Vector Base Register"]
    #[inline(always)]
    pub const fn dtcvbr(&self) -> &Dtcvbr {
        &self.dtcvbr
    }
    #[doc = "0x0c - DTC Module Start Register"]
    #[inline(always)]
    pub const fn dtcst(&self) -> &Dtcst {
        &self.dtcst
    }
    #[doc = "0x0e - DTC Status Register"]
    #[inline(always)]
    pub const fn dtcsts(&self) -> &Dtcsts {
        &self.dtcsts
    }
}
#[doc = "DTCCR (rw) register accessor: DTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtccr`] module"]
#[doc(alias = "DTCCR")]
pub type Dtccr = crate::Reg<dtccr::DtccrSpec>;
#[doc = "DTC Control Register"]
pub mod dtccr;
#[doc = "DTCVBR (rw) register accessor: DTC Vector Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcvbr`] module"]
#[doc(alias = "DTCVBR")]
pub type Dtcvbr = crate::Reg<dtcvbr::DtcvbrSpec>;
#[doc = "DTC Vector Base Register"]
pub mod dtcvbr;
#[doc = "DTCST (rw) register accessor: DTC Module Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcst`] module"]
#[doc(alias = "DTCST")]
pub type Dtcst = crate::Reg<dtcst::DtcstSpec>;
#[doc = "DTC Module Start Register"]
pub mod dtcst;
#[doc = "DTCSTS (r) register accessor: DTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcsts`] module"]
#[doc(alias = "DTCSTS")]
pub type Dtcsts = crate::Reg<dtcsts::DtcstsSpec>;
#[doc = "DTC Status Register"]
pub mod dtcsts;

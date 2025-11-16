#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    parioad: Parioad,
    _reserved1: [u8; 0x03],
    sramprcr: Sramprcr,
    _reserved2: [u8; 0xbb],
    eccmode: Eccmode,
    ecc2sts: Ecc2sts,
    ecc1stsen: Ecc1stsen,
    ecc1sts: Ecc1sts,
    eccprcr: Eccprcr,
    _reserved7: [u8; 0x0b],
    eccprcr2: Eccprcr2,
    _reserved8: [u8; 0x03],
    eccetst: Eccetst,
    _reserved9: [u8; 0x03],
    eccoad: Eccoad,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn parioad(&self) -> &Parioad {
        &self.parioad
    }
    #[doc = "0x04 - SRAM Protection Register"]
    #[inline(always)]
    pub const fn sramprcr(&self) -> &Sramprcr {
        &self.sramprcr
    }
    #[doc = "0xc0 - ECC Operating Mode Control Register"]
    #[inline(always)]
    pub const fn eccmode(&self) -> &Eccmode {
        &self.eccmode
    }
    #[doc = "0xc1 - ECC 2-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc2sts(&self) -> &Ecc2sts {
        &self.ecc2sts
    }
    #[doc = "0xc2 - ECC 1-Bit Error Information Update Enable Register"]
    #[inline(always)]
    pub const fn ecc1stsen(&self) -> &Ecc1stsen {
        &self.ecc1stsen
    }
    #[doc = "0xc3 - ECC 1-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc1sts(&self) -> &Ecc1sts {
        &self.ecc1sts
    }
    #[doc = "0xc4 - ECC Protection Register"]
    #[inline(always)]
    pub const fn eccprcr(&self) -> &Eccprcr {
        &self.eccprcr
    }
    #[doc = "0xd0 - ECC Protection Register 2"]
    #[inline(always)]
    pub const fn eccprcr2(&self) -> &Eccprcr2 {
        &self.eccprcr2
    }
    #[doc = "0xd4 - ECC Test Control Register"]
    #[inline(always)]
    pub const fn eccetst(&self) -> &Eccetst {
        &self.eccetst
    }
    #[doc = "0xd8 - SRAM ECC Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn eccoad(&self) -> &Eccoad {
        &self.eccoad
    }
}
#[doc = "PARIOAD (rw) register accessor: SRAM Parity Error Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`parioad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parioad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parioad`] module"]
#[doc(alias = "PARIOAD")]
pub type Parioad = crate::Reg<parioad::ParioadSpec>;
#[doc = "SRAM Parity Error Operation After Detection Register"]
pub mod parioad;
#[doc = "SRAMPRCR (rw) register accessor: SRAM Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sramprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramprcr`] module"]
#[doc(alias = "SRAMPRCR")]
pub type Sramprcr = crate::Reg<sramprcr::SramprcrSpec>;
#[doc = "SRAM Protection Register"]
pub mod sramprcr;
#[doc = "ECCMODE (rw) register accessor: ECC Operating Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccmode`] module"]
#[doc(alias = "ECCMODE")]
pub type Eccmode = crate::Reg<eccmode::EccmodeSpec>;
#[doc = "ECC Operating Mode Control Register"]
pub mod eccmode;
#[doc = "ECC2STS (rw) register accessor: ECC 2-Bit Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc2sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc2sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc2sts`] module"]
#[doc(alias = "ECC2STS")]
pub type Ecc2sts = crate::Reg<ecc2sts::Ecc2stsSpec>;
#[doc = "ECC 2-Bit Error Status Register"]
pub mod ecc2sts;
#[doc = "ECC1STSEN (rw) register accessor: ECC 1-Bit Error Information Update Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc1stsen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1stsen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc1stsen`] module"]
#[doc(alias = "ECC1STSEN")]
pub type Ecc1stsen = crate::Reg<ecc1stsen::Ecc1stsenSpec>;
#[doc = "ECC 1-Bit Error Information Update Enable Register"]
pub mod ecc1stsen;
#[doc = "ECC1STS (rw) register accessor: ECC 1-Bit Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc1sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc1sts`] module"]
#[doc(alias = "ECC1STS")]
pub type Ecc1sts = crate::Reg<ecc1sts::Ecc1stsSpec>;
#[doc = "ECC 1-Bit Error Status Register"]
pub mod ecc1sts;
#[doc = "ECCPRCR (rw) register accessor: ECC Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccprcr`] module"]
#[doc(alias = "ECCPRCR")]
pub type Eccprcr = crate::Reg<eccprcr::EccprcrSpec>;
#[doc = "ECC Protection Register"]
pub mod eccprcr;
#[doc = "ECCPRCR2 (rw) register accessor: ECC Protection Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eccprcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccprcr2`] module"]
#[doc(alias = "ECCPRCR2")]
pub type Eccprcr2 = crate::Reg<eccprcr2::Eccprcr2Spec>;
#[doc = "ECC Protection Register 2"]
pub mod eccprcr2;
#[doc = "ECCETST (rw) register accessor: ECC Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccetst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccetst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccetst`] module"]
#[doc(alias = "ECCETST")]
pub type Eccetst = crate::Reg<eccetst::EccetstSpec>;
#[doc = "ECC Test Control Register"]
pub mod eccetst;
#[doc = "ECCOAD (rw) register accessor: SRAM ECC Error Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccoad`] module"]
#[doc(alias = "ECCOAD")]
pub type Eccoad = crate::Reg<eccoad::EccoadSpec>;
#[doc = "SRAM ECC Error Operation After Detection Register"]
pub mod eccoad;

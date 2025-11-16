#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmsar: Dmsar,
    dmdar: Dmdar,
    dmcra: Dmcra,
    dmcrb: Dmcrb,
    _reserved4: [u8; 0x02],
    dmtmd: Dmtmd,
    _reserved5: [u8; 0x01],
    dmint: Dmint,
    dmamd: Dmamd,
    _reserved7: [u8; 0x02],
    dmofr: Dmofr,
    dmcnt: Dmcnt,
    dmreq: Dmreq,
    dmsts: Dmsts,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Source Address Register"]
    #[inline(always)]
    pub const fn dmsar(&self) -> &Dmsar {
        &self.dmsar
    }
    #[doc = "0x04 - DMA Destination Address Register"]
    #[inline(always)]
    pub const fn dmdar(&self) -> &Dmdar {
        &self.dmdar
    }
    #[doc = "0x08 - DMA Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcra(&self) -> &Dmcra {
        &self.dmcra
    }
    #[doc = "0x0c - DMA Block Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcrb(&self) -> &Dmcrb {
        &self.dmcrb
    }
    #[doc = "0x10 - DMA Transfer Mode Register"]
    #[inline(always)]
    pub const fn dmtmd(&self) -> &Dmtmd {
        &self.dmtmd
    }
    #[doc = "0x13 - DMA Interrupt Setting Register"]
    #[inline(always)]
    pub const fn dmint(&self) -> &Dmint {
        &self.dmint
    }
    #[doc = "0x14 - DMA Address Mode Register"]
    #[inline(always)]
    pub const fn dmamd(&self) -> &Dmamd {
        &self.dmamd
    }
    #[doc = "0x18 - DMA Offset Register"]
    #[inline(always)]
    pub const fn dmofr(&self) -> &Dmofr {
        &self.dmofr
    }
    #[doc = "0x1c - DMA Transfer Enable Register"]
    #[inline(always)]
    pub const fn dmcnt(&self) -> &Dmcnt {
        &self.dmcnt
    }
    #[doc = "0x1d - DMA Software Start Register"]
    #[inline(always)]
    pub const fn dmreq(&self) -> &Dmreq {
        &self.dmreq
    }
    #[doc = "0x1e - DMA Status Register"]
    #[inline(always)]
    pub const fn dmsts(&self) -> &Dmsts {
        &self.dmsts
    }
}
#[doc = "DMSAR (rw) register accessor: DMA Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmsar`] module"]
#[doc(alias = "DMSAR")]
pub type Dmsar = crate::Reg<dmsar::DmsarSpec>;
#[doc = "DMA Source Address Register"]
pub mod dmsar;
#[doc = "DMDAR (rw) register accessor: DMA Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmdar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmdar`] module"]
#[doc(alias = "DMDAR")]
pub type Dmdar = crate::Reg<dmdar::DmdarSpec>;
#[doc = "DMA Destination Address Register"]
pub mod dmdar;
#[doc = "DMCRA (rw) register accessor: DMA Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcra`] module"]
#[doc(alias = "DMCRA")]
pub type Dmcra = crate::Reg<dmcra::DmcraSpec>;
#[doc = "DMA Transfer Count Register"]
pub mod dmcra;
#[doc = "DMCRB (rw) register accessor: DMA Block Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcrb`] module"]
#[doc(alias = "DMCRB")]
pub type Dmcrb = crate::Reg<dmcrb::DmcrbSpec>;
#[doc = "DMA Block Transfer Count Register"]
pub mod dmcrb;
#[doc = "DMTMD (rw) register accessor: DMA Transfer Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmtmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmtmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmtmd`] module"]
#[doc(alias = "DMTMD")]
pub type Dmtmd = crate::Reg<dmtmd::DmtmdSpec>;
#[doc = "DMA Transfer Mode Register"]
pub mod dmtmd;
#[doc = "DMINT (rw) register accessor: DMA Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmint`] module"]
#[doc(alias = "DMINT")]
pub type Dmint = crate::Reg<dmint::DmintSpec>;
#[doc = "DMA Interrupt Setting Register"]
pub mod dmint;
#[doc = "DMAMD (rw) register accessor: DMA Address Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamd`] module"]
#[doc(alias = "DMAMD")]
pub type Dmamd = crate::Reg<dmamd::DmamdSpec>;
#[doc = "DMA Address Mode Register"]
pub mod dmamd;
#[doc = "DMOFR (rw) register accessor: DMA Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmofr`] module"]
#[doc(alias = "DMOFR")]
pub type Dmofr = crate::Reg<dmofr::DmofrSpec>;
#[doc = "DMA Offset Register"]
pub mod dmofr;
#[doc = "DMCNT (rw) register accessor: DMA Transfer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcnt`] module"]
#[doc(alias = "DMCNT")]
pub type Dmcnt = crate::Reg<dmcnt::DmcntSpec>;
#[doc = "DMA Transfer Enable Register"]
pub mod dmcnt;
#[doc = "DMREQ (rw) register accessor: DMA Software Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmreq`] module"]
#[doc(alias = "DMREQ")]
pub type Dmreq = crate::Reg<dmreq::DmreqSpec>;
#[doc = "DMA Software Start Register"]
pub mod dmreq;
#[doc = "DMSTS (rw) register accessor: DMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmsts`] module"]
#[doc(alias = "DMSTS")]
pub type Dmsts = crate::Reg<dmsts::DmstsSpec>;
#[doc = "DMA Status Register"]
pub mod dmsts;

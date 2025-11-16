#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_smr: [u8; 0x01],
    brr: Brr,
    _reserved_2_scr: [u8; 0x01],
    tdr: Tdr,
    _reserved_4_ssr: [u8; 0x01],
    rdr: Rdr,
    scmr: Scmr,
    semr: Semr,
    snfr: Snfr,
    simr1: Simr1,
    simr2: Simr2,
    simr3: Simr3,
    sisr: Sisr,
    spmr: Spmr,
    tdrhl: Tdrhl,
    rdrhl: Rdrhl,
    mddr: Mddr,
    dccr: Dccr,
    _reserved18: [u8; 0x06],
    cdr: Cdr,
    sptr: Sptr,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial mode register (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SmrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Serial Mode Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &Smr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x01 - Bit Rate Register"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF =1)"]
    #[inline(always)]
    pub const fn scr_smci(&self) -> &ScrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SsrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &Ssr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x05 - Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x06 - Smart Card Mode Register"]
    #[inline(always)]
    pub const fn scmr(&self) -> &Scmr {
        &self.scmr
    }
    #[doc = "0x07 - Serial Extended Mode Register"]
    #[inline(always)]
    pub const fn semr(&self) -> &Semr {
        &self.semr
    }
    #[doc = "0x08 - Noise Filter Setting Register"]
    #[inline(always)]
    pub const fn snfr(&self) -> &Snfr {
        &self.snfr
    }
    #[doc = "0x09 - I2C Mode Register 1"]
    #[inline(always)]
    pub const fn simr1(&self) -> &Simr1 {
        &self.simr1
    }
    #[doc = "0x0a - I2C Mode Register 2"]
    #[inline(always)]
    pub const fn simr2(&self) -> &Simr2 {
        &self.simr2
    }
    #[doc = "0x0b - I2C Mode Register 3"]
    #[inline(always)]
    pub const fn simr3(&self) -> &Simr3 {
        &self.simr3
    }
    #[doc = "0x0c - I2C Status Register"]
    #[inline(always)]
    pub const fn sisr(&self) -> &Sisr {
        &self.sisr
    }
    #[doc = "0x0d - SPI Mode Register"]
    #[inline(always)]
    pub const fn spmr(&self) -> &Spmr {
        &self.spmr
    }
    #[doc = "0x0e - Transmit 9-bit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &Tdrhl {
        &self.tdrhl
    }
    #[doc = "0x10 - Receive 9-bit Data Register"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &Rdrhl {
        &self.rdrhl
    }
    #[doc = "0x12 - Modulation Duty Register"]
    #[inline(always)]
    pub const fn mddr(&self) -> &Mddr {
        &self.mddr
    }
    #[doc = "0x13 - Data Compare Match Control Register"]
    #[inline(always)]
    pub const fn dccr(&self) -> &Dccr {
        &self.dccr
    }
    #[doc = "0x1a - Compare Match Data Register"]
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    #[doc = "0x1c - Serial Port Register"]
    #[inline(always)]
    pub const fn sptr(&self) -> &Sptr {
        &self.sptr
    }
}
#[doc = "SMR (rw) register accessor: Serial Mode Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr`] module"]
#[doc(alias = "SMR")]
pub type Smr = crate::Reg<smr::SmrSpec>;
#[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
pub mod smr;
#[doc = "SMR_SMCI (rw) register accessor: Serial mode register (SCMR.SMIF = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`smr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr_smci`] module"]
#[doc(alias = "SMR_SMCI")]
pub type SmrSmci = crate::Reg<smr_smci::SmrSmciSpec>;
#[doc = "Serial mode register (SCMR.SMIF = 1)"]
pub mod smr_smci;
#[doc = "BRR (rw) register accessor: Bit Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "Bit Rate Register"]
pub mod brr;
#[doc = "SCR (rw) register accessor: Serial Control Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Serial Control Register (SCMR.SMIF = 0)"]
pub mod scr;
#[doc = "SCR_SMCI (rw) register accessor: Serial Control Register (SCMR.SMIF =1)\n\nYou can [`read`](crate::Reg::read) this register and get [`scr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr_smci`] module"]
#[doc(alias = "SCR_SMCI")]
pub type ScrSmci = crate::Reg<scr_smci::ScrSmciSpec>;
#[doc = "Serial Control Register (SCMR.SMIF =1)"]
pub mod scr_smci;
#[doc = "TDR (rw) register accessor: Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SSR (rw) register accessor: Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`] module"]
#[doc(alias = "SSR")]
pub type Ssr = crate::Reg<ssr::SsrSpec>;
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
pub mod ssr;
#[doc = "SSR_SMCI (rw) register accessor: Serial Status Register(SCMR.SMIF = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr_smci`] module"]
#[doc(alias = "SSR_SMCI")]
pub type SsrSmci = crate::Reg<ssr_smci::SsrSmciSpec>;
#[doc = "Serial Status Register(SCMR.SMIF = 1)"]
pub mod ssr_smci;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "SCMR (rw) register accessor: Smart Card Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmr`] module"]
#[doc(alias = "SCMR")]
pub type Scmr = crate::Reg<scmr::ScmrSpec>;
#[doc = "Smart Card Mode Register"]
pub mod scmr;
#[doc = "SEMR (rw) register accessor: Serial Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`semr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@semr`] module"]
#[doc(alias = "SEMR")]
pub type Semr = crate::Reg<semr::SemrSpec>;
#[doc = "Serial Extended Mode Register"]
pub mod semr;
#[doc = "SNFR (rw) register accessor: Noise Filter Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snfr`] module"]
#[doc(alias = "SNFR")]
pub type Snfr = crate::Reg<snfr::SnfrSpec>;
#[doc = "Noise Filter Setting Register"]
pub mod snfr;
#[doc = "SIMR1 (rw) register accessor: I2C Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`simr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr1`] module"]
#[doc(alias = "SIMR1")]
pub type Simr1 = crate::Reg<simr1::Simr1Spec>;
#[doc = "I2C Mode Register 1"]
pub mod simr1;
#[doc = "SIMR2 (rw) register accessor: I2C Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`simr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr2`] module"]
#[doc(alias = "SIMR2")]
pub type Simr2 = crate::Reg<simr2::Simr2Spec>;
#[doc = "I2C Mode Register 2"]
pub mod simr2;
#[doc = "SIMR3 (rw) register accessor: I2C Mode Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`simr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr3`] module"]
#[doc(alias = "SIMR3")]
pub type Simr3 = crate::Reg<simr3::Simr3Spec>;
#[doc = "I2C Mode Register 3"]
pub mod simr3;
#[doc = "SISR (r) register accessor: I2C Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sisr`] module"]
#[doc(alias = "SISR")]
pub type Sisr = crate::Reg<sisr::SisrSpec>;
#[doc = "I2C Status Register"]
pub mod sisr;
#[doc = "SPMR (rw) register accessor: SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmr`] module"]
#[doc(alias = "SPMR")]
pub type Spmr = crate::Reg<spmr::SpmrSpec>;
#[doc = "SPI Mode Register"]
pub mod spmr;
#[doc = "TDRHL (rw) register accessor: Transmit 9-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdrhl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdrhl`] module"]
#[doc(alias = "TDRHL")]
pub type Tdrhl = crate::Reg<tdrhl::TdrhlSpec>;
#[doc = "Transmit 9-bit Data Register"]
pub mod tdrhl;
#[doc = "RDRHL (r) register accessor: Receive 9-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdrhl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdrhl`] module"]
#[doc(alias = "RDRHL")]
pub type Rdrhl = crate::Reg<rdrhl::RdrhlSpec>;
#[doc = "Receive 9-bit Data Register"]
pub mod rdrhl;
#[doc = "MDDR (rw) register accessor: Modulation Duty Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mddr`] module"]
#[doc(alias = "MDDR")]
pub type Mddr = crate::Reg<mddr::MddrSpec>;
#[doc = "Modulation Duty Register"]
pub mod mddr;
#[doc = "DCCR (rw) register accessor: Data Compare Match Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccr`] module"]
#[doc(alias = "DCCR")]
pub type Dccr = crate::Reg<dccr::DccrSpec>;
#[doc = "Data Compare Match Control Register"]
pub mod dccr;
#[doc = "CDR (rw) register accessor: Compare Match Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`] module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "Compare Match Data Register"]
pub mod cdr;
#[doc = "SPTR (rw) register accessor: Serial Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sptr`] module"]
#[doc(alias = "SPTR")]
pub type Sptr = crate::Reg<sptr::SptrSpec>;
#[doc = "Serial Port Register"]
pub mod sptr;

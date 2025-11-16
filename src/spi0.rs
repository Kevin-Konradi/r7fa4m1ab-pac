#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spcr: Spcr,
    sslp: Sslp,
    sppcr: Sppcr,
    spsr: Spsr,
    _reserved_4_spdr: [u8; 0x04],
    _reserved5: [u8; 0x02],
    spbr: Spbr,
    spdcr: Spdcr,
    spckd: Spckd,
    sslnd: Sslnd,
    spnd: Spnd,
    spcr2: Spcr2,
    spcmd0: Spcmd0,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    #[inline(always)]
    pub const fn spcr(&self) -> &Spcr {
        &self.spcr
    }
    #[doc = "0x01 - SPI Slave Select Polarity Register"]
    #[inline(always)]
    pub const fn sslp(&self) -> &Sslp {
        &self.sslp
    }
    #[doc = "0x02 - SPI Pin Control Register"]
    #[inline(always)]
    pub const fn sppcr(&self) -> &Sppcr {
        &self.sppcr
    }
    #[doc = "0x03 - SPI Status Register"]
    #[inline(always)]
    pub const fn spsr(&self) -> &Spsr {
        &self.spsr
    }
    #[doc = "0x04 - SPI Data Register ( halfword access )"]
    #[inline(always)]
    pub const fn spdr_ha(&self) -> &SpdrHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &Spdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x0a - SPI Bit Rate Register"]
    #[inline(always)]
    pub const fn spbr(&self) -> &Spbr {
        &self.spbr
    }
    #[doc = "0x0b - SPI Data Control Register"]
    #[inline(always)]
    pub const fn spdcr(&self) -> &Spdcr {
        &self.spdcr
    }
    #[doc = "0x0c - SPI Clock Delay Register"]
    #[inline(always)]
    pub const fn spckd(&self) -> &Spckd {
        &self.spckd
    }
    #[doc = "0x0d - SPI Slave Select Negation Delay Register"]
    #[inline(always)]
    pub const fn sslnd(&self) -> &Sslnd {
        &self.sslnd
    }
    #[doc = "0x0e - SPI Next-Access Delay Register"]
    #[inline(always)]
    pub const fn spnd(&self) -> &Spnd {
        &self.spnd
    }
    #[doc = "0x0f - SPI Control Register 2"]
    #[inline(always)]
    pub const fn spcr2(&self) -> &Spcr2 {
        &self.spcr2
    }
    #[doc = "0x10 - SPI Command Register 0"]
    #[inline(always)]
    pub const fn spcmd0(&self) -> &Spcmd0 {
        &self.spcmd0
    }
}
#[doc = "SPCR (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr`] module"]
#[doc(alias = "SPCR")]
pub type Spcr = crate::Reg<spcr::SpcrSpec>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SSLP (rw) register accessor: SPI Slave Select Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sslp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslp`] module"]
#[doc(alias = "SSLP")]
pub type Sslp = crate::Reg<sslp::SslpSpec>;
#[doc = "SPI Slave Select Polarity Register"]
pub mod sslp;
#[doc = "SPPCR (rw) register accessor: SPI Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sppcr`] module"]
#[doc(alias = "SPPCR")]
pub type Sppcr = crate::Reg<sppcr::SppcrSpec>;
#[doc = "SPI Pin Control Register"]
pub mod sppcr;
#[doc = "SPSR (rw) register accessor: SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsr`] module"]
#[doc(alias = "SPSR")]
pub type Spsr = crate::Reg<spsr::SpsrSpec>;
#[doc = "SPI Status Register"]
pub mod spsr;
#[doc = "SPDR (rw) register accessor: SPI Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr`] module"]
#[doc(alias = "SPDR")]
pub type Spdr = crate::Reg<spdr::SpdrSpec>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPDR_HA (rw) register accessor: SPI Data Register ( halfword access )\n\nYou can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr_ha`] module"]
#[doc(alias = "SPDR_HA")]
pub type SpdrHa = crate::Reg<spdr_ha::SpdrHaSpec>;
#[doc = "SPI Data Register ( halfword access )"]
pub mod spdr_ha;
#[doc = "SPBR (rw) register accessor: SPI Bit Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spbr`] module"]
#[doc(alias = "SPBR")]
pub type Spbr = crate::Reg<spbr::SpbrSpec>;
#[doc = "SPI Bit Rate Register"]
pub mod spbr;
#[doc = "SPDCR (rw) register accessor: SPI Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdcr`] module"]
#[doc(alias = "SPDCR")]
pub type Spdcr = crate::Reg<spdcr::SpdcrSpec>;
#[doc = "SPI Data Control Register"]
pub mod spdcr;
#[doc = "SPCKD (rw) register accessor: SPI Clock Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spckd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spckd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spckd`] module"]
#[doc(alias = "SPCKD")]
pub type Spckd = crate::Reg<spckd::SpckdSpec>;
#[doc = "SPI Clock Delay Register"]
pub mod spckd;
#[doc = "SSLND (rw) register accessor: SPI Slave Select Negation Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sslnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslnd`] module"]
#[doc(alias = "SSLND")]
pub type Sslnd = crate::Reg<sslnd::SslndSpec>;
#[doc = "SPI Slave Select Negation Delay Register"]
pub mod sslnd;
#[doc = "SPND (rw) register accessor: SPI Next-Access Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spnd`] module"]
#[doc(alias = "SPND")]
pub type Spnd = crate::Reg<spnd::SpndSpec>;
#[doc = "SPI Next-Access Delay Register"]
pub mod spnd;
#[doc = "SPCR2 (rw) register accessor: SPI Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr2`] module"]
#[doc(alias = "SPCR2")]
pub type Spcr2 = crate::Reg<spcr2::Spcr2Spec>;
#[doc = "SPI Control Register 2"]
pub mod spcr2;
#[doc = "SPCMD0 (rw) register accessor: SPI Command Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spcmd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcmd0`] module"]
#[doc(alias = "SPCMD0")]
pub type Spcmd0 = crate::Reg<spcmd0::Spcmd0Spec>;
#[doc = "SPI Command Register 0"]
pub mod spcmd0;

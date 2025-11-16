#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtwp: Gtwp,
    gtstr: Gtstr,
    gtstp: Gtstp,
    gtclr: Gtclr,
    gtssr: Gtssr,
    gtpsr: Gtpsr,
    gtcsr: Gtcsr,
    gtupsr: Gtupsr,
    gtdnsr: Gtdnsr,
    gticasr: Gticasr,
    gticbsr: Gticbsr,
    gtcr: Gtcr,
    gtuddtyc: Gtuddtyc,
    gtior: Gtior,
    gtintad: Gtintad,
    gtst: Gtst,
    gtber: Gtber,
    _reserved17: [u8; 0x04],
    gtcnt: Gtcnt,
    gtccra: Gtccra,
    gtccrb: Gtccrb,
    gtccrc: Gtccrc,
    gtccre: Gtccre,
    gtccrd: Gtccrd,
    gtccrf: Gtccrf,
    gtpr: Gtpr,
    gtpbr: Gtpbr,
    _reserved26: [u8; 0x1c],
    gtdtcr: Gtdtcr,
    gtdvu: Gtdvu,
}
impl RegisterBlock {
    #[doc = "0x00 - General PWM Timer Write-Protection Register"]
    #[inline(always)]
    pub const fn gtwp(&self) -> &Gtwp {
        &self.gtwp
    }
    #[doc = "0x04 - General PWM Timer Software Start Register"]
    #[inline(always)]
    pub const fn gtstr(&self) -> &Gtstr {
        &self.gtstr
    }
    #[doc = "0x08 - General PWM Timer Software Stop Register"]
    #[inline(always)]
    pub const fn gtstp(&self) -> &Gtstp {
        &self.gtstp
    }
    #[doc = "0x0c - General PWM Timer Software Clear Register"]
    #[inline(always)]
    pub const fn gtclr(&self) -> &Gtclr {
        &self.gtclr
    }
    #[doc = "0x10 - General PWM Timer Start Source Select Register"]
    #[inline(always)]
    pub const fn gtssr(&self) -> &Gtssr {
        &self.gtssr
    }
    #[doc = "0x14 - General PWM Timer Stop Source Select Register"]
    #[inline(always)]
    pub const fn gtpsr(&self) -> &Gtpsr {
        &self.gtpsr
    }
    #[doc = "0x18 - General PWM Timer Clear Source Select Register"]
    #[inline(always)]
    pub const fn gtcsr(&self) -> &Gtcsr {
        &self.gtcsr
    }
    #[doc = "0x1c - General PWM Timer Up Count Source Select Register"]
    #[inline(always)]
    pub const fn gtupsr(&self) -> &Gtupsr {
        &self.gtupsr
    }
    #[doc = "0x20 - General PWM Timer Down Count Source Select Register"]
    #[inline(always)]
    pub const fn gtdnsr(&self) -> &Gtdnsr {
        &self.gtdnsr
    }
    #[doc = "0x24 - General PWM Timer Input Capture Source Select Register A"]
    #[inline(always)]
    pub const fn gticasr(&self) -> &Gticasr {
        &self.gticasr
    }
    #[doc = "0x28 - General PWM Timer Input Capture Source Select Register B"]
    #[inline(always)]
    pub const fn gticbsr(&self) -> &Gticbsr {
        &self.gticbsr
    }
    #[doc = "0x2c - General PWM Timer Control Register"]
    #[inline(always)]
    pub const fn gtcr(&self) -> &Gtcr {
        &self.gtcr
    }
    #[doc = "0x30 - General PWM Timer Count Direction and Duty Setting Register"]
    #[inline(always)]
    pub const fn gtuddtyc(&self) -> &Gtuddtyc {
        &self.gtuddtyc
    }
    #[doc = "0x34 - General PWM Timer I/O Control Register"]
    #[inline(always)]
    pub const fn gtior(&self) -> &Gtior {
        &self.gtior
    }
    #[doc = "0x38 - General PWM Timer Interrupt Output Setting Register"]
    #[inline(always)]
    pub const fn gtintad(&self) -> &Gtintad {
        &self.gtintad
    }
    #[doc = "0x3c - General PWM Timer Status Register"]
    #[inline(always)]
    pub const fn gtst(&self) -> &Gtst {
        &self.gtst
    }
    #[doc = "0x40 - General PWM Timer Buffer Enable Register"]
    #[inline(always)]
    pub const fn gtber(&self) -> &Gtber {
        &self.gtber
    }
    #[doc = "0x48 - General PWM Timer Counter"]
    #[inline(always)]
    pub const fn gtcnt(&self) -> &Gtcnt {
        &self.gtcnt
    }
    #[doc = "0x4c - General PWM Timer Compare Capture Register A"]
    #[inline(always)]
    pub const fn gtccra(&self) -> &Gtccra {
        &self.gtccra
    }
    #[doc = "0x50 - General PWM Timer Compare Capture Register B"]
    #[inline(always)]
    pub const fn gtccrb(&self) -> &Gtccrb {
        &self.gtccrb
    }
    #[doc = "0x54 - General PWM Timer Compare Capture Register C"]
    #[inline(always)]
    pub const fn gtccrc(&self) -> &Gtccrc {
        &self.gtccrc
    }
    #[doc = "0x58 - General PWM Timer Compare Capture Register E"]
    #[inline(always)]
    pub const fn gtccre(&self) -> &Gtccre {
        &self.gtccre
    }
    #[doc = "0x5c - General PWM Timer Compare Capture Register D"]
    #[inline(always)]
    pub const fn gtccrd(&self) -> &Gtccrd {
        &self.gtccrd
    }
    #[doc = "0x60 - General PWM Timer Compare Capture Register F"]
    #[inline(always)]
    pub const fn gtccrf(&self) -> &Gtccrf {
        &self.gtccrf
    }
    #[doc = "0x64 - General PWM Timer Cycle Setting Register"]
    #[inline(always)]
    pub const fn gtpr(&self) -> &Gtpr {
        &self.gtpr
    }
    #[doc = "0x68 - General PWM Timer Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn gtpbr(&self) -> &Gtpbr {
        &self.gtpbr
    }
    #[doc = "0x88 - General PWM Timer Dead Time Control Register"]
    #[inline(always)]
    pub const fn gtdtcr(&self) -> &Gtdtcr {
        &self.gtdtcr
    }
    #[doc = "0x8c - General PWM Timer Dead Time Value Register U"]
    #[inline(always)]
    pub const fn gtdvu(&self) -> &Gtdvu {
        &self.gtdvu
    }
}
#[doc = "GTWP (rw) register accessor: General PWM Timer Write-Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtwp`] module"]
#[doc(alias = "GTWP")]
pub type Gtwp = crate::Reg<gtwp::GtwpSpec>;
#[doc = "General PWM Timer Write-Protection Register"]
pub mod gtwp;
#[doc = "GTSTR (rw) register accessor: General PWM Timer Software Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtstr`] module"]
#[doc(alias = "GTSTR")]
pub type Gtstr = crate::Reg<gtstr::GtstrSpec>;
#[doc = "General PWM Timer Software Start Register"]
pub mod gtstr;
#[doc = "GTSTP (rw) register accessor: General PWM Timer Software Stop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtstp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtstp`] module"]
#[doc(alias = "GTSTP")]
pub type Gtstp = crate::Reg<gtstp::GtstpSpec>;
#[doc = "General PWM Timer Software Stop Register"]
pub mod gtstp;
#[doc = "GTCLR (w) register accessor: General PWM Timer Software Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtclr`] module"]
#[doc(alias = "GTCLR")]
pub type Gtclr = crate::Reg<gtclr::GtclrSpec>;
#[doc = "General PWM Timer Software Clear Register"]
pub mod gtclr;
#[doc = "GTSSR (rw) register accessor: General PWM Timer Start Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtssr`] module"]
#[doc(alias = "GTSSR")]
pub type Gtssr = crate::Reg<gtssr::GtssrSpec>;
#[doc = "General PWM Timer Start Source Select Register"]
pub mod gtssr;
#[doc = "GTPSR (rw) register accessor: General PWM Timer Stop Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpsr`] module"]
#[doc(alias = "GTPSR")]
pub type Gtpsr = crate::Reg<gtpsr::GtpsrSpec>;
#[doc = "General PWM Timer Stop Source Select Register"]
pub mod gtpsr;
#[doc = "GTCSR (rw) register accessor: General PWM Timer Clear Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcsr`] module"]
#[doc(alias = "GTCSR")]
pub type Gtcsr = crate::Reg<gtcsr::GtcsrSpec>;
#[doc = "General PWM Timer Clear Source Select Register"]
pub mod gtcsr;
#[doc = "GTUPSR (rw) register accessor: General PWM Timer Up Count Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtupsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtupsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtupsr`] module"]
#[doc(alias = "GTUPSR")]
pub type Gtupsr = crate::Reg<gtupsr::GtupsrSpec>;
#[doc = "General PWM Timer Up Count Source Select Register"]
pub mod gtupsr;
#[doc = "GTDNSR (rw) register accessor: General PWM Timer Down Count Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdnsr`] module"]
#[doc(alias = "GTDNSR")]
pub type Gtdnsr = crate::Reg<gtdnsr::GtdnsrSpec>;
#[doc = "General PWM Timer Down Count Source Select Register"]
pub mod gtdnsr;
#[doc = "GTICASR (rw) register accessor: General PWM Timer Input Capture Source Select Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gticasr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gticasr`] module"]
#[doc(alias = "GTICASR")]
pub type Gticasr = crate::Reg<gticasr::GticasrSpec>;
#[doc = "General PWM Timer Input Capture Source Select Register A"]
pub mod gticasr;
#[doc = "GTICBSR (rw) register accessor: General PWM Timer Input Capture Source Select Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gticbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gticbsr`] module"]
#[doc(alias = "GTICBSR")]
pub type Gticbsr = crate::Reg<gticbsr::GticbsrSpec>;
#[doc = "General PWM Timer Input Capture Source Select Register B"]
pub mod gticbsr;
#[doc = "GTCR (rw) register accessor: General PWM Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcr`] module"]
#[doc(alias = "GTCR")]
pub type Gtcr = crate::Reg<gtcr::GtcrSpec>;
#[doc = "General PWM Timer Control Register"]
pub mod gtcr;
#[doc = "GTUDDTYC (rw) register accessor: General PWM Timer Count Direction and Duty Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtuddtyc`] module"]
#[doc(alias = "GTUDDTYC")]
pub type Gtuddtyc = crate::Reg<gtuddtyc::GtuddtycSpec>;
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
pub mod gtuddtyc;
#[doc = "GTIOR (rw) register accessor: General PWM Timer I/O Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtior`] module"]
#[doc(alias = "GTIOR")]
pub type Gtior = crate::Reg<gtior::GtiorSpec>;
#[doc = "General PWM Timer I/O Control Register"]
pub mod gtior;
#[doc = "GTINTAD (rw) register accessor: General PWM Timer Interrupt Output Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtintad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtintad`] module"]
#[doc(alias = "GTINTAD")]
pub type Gtintad = crate::Reg<gtintad::GtintadSpec>;
#[doc = "General PWM Timer Interrupt Output Setting Register"]
pub mod gtintad;
#[doc = "GTST (rw) register accessor: General PWM Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtst`] module"]
#[doc(alias = "GTST")]
pub type Gtst = crate::Reg<gtst::GtstSpec>;
#[doc = "General PWM Timer Status Register"]
pub mod gtst;
#[doc = "GTBER (rw) register accessor: General PWM Timer Buffer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtber`] module"]
#[doc(alias = "GTBER")]
pub type Gtber = crate::Reg<gtber::GtberSpec>;
#[doc = "General PWM Timer Buffer Enable Register"]
pub mod gtber;
#[doc = "GTCNT (rw) register accessor: General PWM Timer Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcnt`] module"]
#[doc(alias = "GTCNT")]
pub type Gtcnt = crate::Reg<gtcnt::GtcntSpec>;
#[doc = "General PWM Timer Counter"]
pub mod gtcnt;
#[doc = "GTCCRA (rw) register accessor: General PWM Timer Compare Capture Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccra`] module"]
#[doc(alias = "GTCCRA")]
pub type Gtccra = crate::Reg<gtccra::GtccraSpec>;
#[doc = "General PWM Timer Compare Capture Register A"]
pub mod gtccra;
#[doc = "GTCCRB (rw) register accessor: General PWM Timer Compare Capture Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrb`] module"]
#[doc(alias = "GTCCRB")]
pub type Gtccrb = crate::Reg<gtccrb::GtccrbSpec>;
#[doc = "General PWM Timer Compare Capture Register B"]
pub mod gtccrb;
#[doc = "GTCCRC (rw) register accessor: General PWM Timer Compare Capture Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrc`] module"]
#[doc(alias = "GTCCRC")]
pub type Gtccrc = crate::Reg<gtccrc::GtccrcSpec>;
#[doc = "General PWM Timer Compare Capture Register C"]
pub mod gtccrc;
#[doc = "GTCCRE (rw) register accessor: General PWM Timer Compare Capture Register E\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccre`] module"]
#[doc(alias = "GTCCRE")]
pub type Gtccre = crate::Reg<gtccre::GtccreSpec>;
#[doc = "General PWM Timer Compare Capture Register E"]
pub mod gtccre;
#[doc = "GTCCRD (rw) register accessor: General PWM Timer Compare Capture Register D\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrd`] module"]
#[doc(alias = "GTCCRD")]
pub type Gtccrd = crate::Reg<gtccrd::GtccrdSpec>;
#[doc = "General PWM Timer Compare Capture Register D"]
pub mod gtccrd;
#[doc = "GTCCRF (rw) register accessor: General PWM Timer Compare Capture Register F\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrf`] module"]
#[doc(alias = "GTCCRF")]
pub type Gtccrf = crate::Reg<gtccrf::GtccrfSpec>;
#[doc = "General PWM Timer Compare Capture Register F"]
pub mod gtccrf;
#[doc = "GTPR (rw) register accessor: General PWM Timer Cycle Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpr`] module"]
#[doc(alias = "GTPR")]
pub type Gtpr = crate::Reg<gtpr::GtprSpec>;
#[doc = "General PWM Timer Cycle Setting Register"]
pub mod gtpr;
#[doc = "GTPBR (rw) register accessor: General PWM Timer Cycle Setting Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpbr`] module"]
#[doc(alias = "GTPBR")]
pub type Gtpbr = crate::Reg<gtpbr::GtpbrSpec>;
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
pub mod gtpbr;
#[doc = "GTDTCR (rw) register accessor: General PWM Timer Dead Time Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdtcr`] module"]
#[doc(alias = "GTDTCR")]
pub type Gtdtcr = crate::Reg<gtdtcr::GtdtcrSpec>;
#[doc = "General PWM Timer Dead Time Control Register"]
pub mod gtdtcr;
#[doc = "GTDVU (rw) register accessor: General PWM Timer Dead Time Value Register U\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdvu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdvu`] module"]
#[doc(alias = "GTDVU")]
pub type Gtdvu = crate::Reg<gtdvu::GtdvuSpec>;
#[doc = "General PWM Timer Dead Time Value Register U"]
pub mod gtdvu;

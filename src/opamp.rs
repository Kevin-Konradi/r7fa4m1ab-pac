#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ampmc: Ampmc,
    amptrm: Amptrm,
    amptrs: Amptrs,
    ampc: Ampc,
    ampmon: Ampmon,
}
impl RegisterBlock {
    #[doc = "0x08 - Operational amplifier mode control register"]
    #[inline(always)]
    pub const fn ampmc(&self) -> &Ampmc {
        &self.ampmc
    }
    #[doc = "0x09 - Operational amplifier trigger mode control register"]
    #[inline(always)]
    pub const fn amptrm(&self) -> &Amptrm {
        &self.amptrm
    }
    #[doc = "0x0a - Operational Amplifier Activation Trigger Select Register"]
    #[inline(always)]
    pub const fn amptrs(&self) -> &Amptrs {
        &self.amptrs
    }
    #[doc = "0x0b - Operational amplifier control register"]
    #[inline(always)]
    pub const fn ampc(&self) -> &Ampc {
        &self.ampc
    }
    #[doc = "0x0c - Operational amplifier monitor register"]
    #[inline(always)]
    pub const fn ampmon(&self) -> &Ampmon {
        &self.ampmon
    }
}
#[doc = "AMPMC (rw) register accessor: Operational amplifier mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampmc`] module"]
#[doc(alias = "AMPMC")]
pub type Ampmc = crate::Reg<ampmc::AmpmcSpec>;
#[doc = "Operational amplifier mode control register"]
pub mod ampmc;
#[doc = "AMPTRM (rw) register accessor: Operational amplifier trigger mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`amptrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amptrm`] module"]
#[doc(alias = "AMPTRM")]
pub type Amptrm = crate::Reg<amptrm::AmptrmSpec>;
#[doc = "Operational amplifier trigger mode control register"]
pub mod amptrm;
#[doc = "AMPTRS (rw) register accessor: Operational Amplifier Activation Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amptrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amptrs`] module"]
#[doc(alias = "AMPTRS")]
pub type Amptrs = crate::Reg<amptrs::AmptrsSpec>;
#[doc = "Operational Amplifier Activation Trigger Select Register"]
pub mod amptrs;
#[doc = "AMPC (rw) register accessor: Operational amplifier control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampc`] module"]
#[doc(alias = "AMPC")]
pub type Ampc = crate::Reg<ampc::AmpcSpec>;
#[doc = "Operational amplifier control register"]
pub mod ampc;
#[doc = "AMPMON (r) register accessor: Operational amplifier monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampmon`] module"]
#[doc(alias = "AMPMON")]
pub type Ampmon = crate::Reg<ampmon::AmpmonSpec>;
#[doc = "Operational amplifier monitor register"]
pub mod ampmon;

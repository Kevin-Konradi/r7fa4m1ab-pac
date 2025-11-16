#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mspmpuoad: Mspmpuoad,
    _reserved1: [u8; 0x02],
    mspmpuctl: Mspmpuctl,
    mspmpupt: Mspmpupt,
    mspmpusa: Mspmpusa,
    mspmpuea: Mspmpuea,
    pspmpuoad: Pspmpuoad,
    _reserved6: [u8; 0x02],
    pspmpuctl: Pspmpuctl,
    pspmpupt: Pspmpupt,
    pspmpusa: Pspmpusa,
    pspmpuea: Pspmpuea,
}
impl RegisterBlock {
    #[doc = "0x00 - Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn mspmpuoad(&self) -> &Mspmpuoad {
        &self.mspmpuoad
    }
    #[doc = "0x04 - Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn mspmpuctl(&self) -> &Mspmpuctl {
        &self.mspmpuctl
    }
    #[doc = "0x06 - Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn mspmpupt(&self) -> &Mspmpupt {
        &self.mspmpupt
    }
    #[doc = "0x08 - Main Stack Pointer (MSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn mspmpusa(&self) -> &Mspmpusa {
        &self.mspmpusa
    }
    #[doc = "0x0c - Main Stack Pointer (MSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn mspmpuea(&self) -> &Mspmpuea {
        &self.mspmpuea
    }
    #[doc = "0x10 - Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn pspmpuoad(&self) -> &Pspmpuoad {
        &self.pspmpuoad
    }
    #[doc = "0x14 - Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn pspmpuctl(&self) -> &Pspmpuctl {
        &self.pspmpuctl
    }
    #[doc = "0x16 - Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn pspmpupt(&self) -> &Pspmpupt {
        &self.pspmpupt
    }
    #[doc = "0x18 - Process Stack Pointer (PSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn pspmpusa(&self) -> &Pspmpusa {
        &self.pspmpusa
    }
    #[doc = "0x1c - Process Stack Pointer (PSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn pspmpuea(&self) -> &Pspmpuea {
        &self.pspmpuea
    }
}
#[doc = "MSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpuoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuoad`] module"]
#[doc(alias = "MSPMPUOAD")]
pub type Mspmpuoad = crate::Reg<mspmpuoad::MspmpuoadSpec>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod mspmpuoad;
#[doc = "MSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuctl`] module"]
#[doc(alias = "MSPMPUCTL")]
pub type Mspmpuctl = crate::Reg<mspmpuctl::MspmpuctlSpec>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod mspmpuctl;
#[doc = "MSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpupt`] module"]
#[doc(alias = "MSPMPUPT")]
pub type Mspmpupt = crate::Reg<mspmpupt::MspmpuptSpec>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod mspmpupt;
#[doc = "MSPMPUSA (rw) register accessor: Main Stack Pointer (MSP) Monitor Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpusa`] module"]
#[doc(alias = "MSPMPUSA")]
pub type Mspmpusa = crate::Reg<mspmpusa::MspmpusaSpec>;
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
pub mod mspmpusa;
#[doc = "MSPMPUEA (rw) register accessor: Main Stack Pointer (MSP) Monitor End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuea`] module"]
#[doc(alias = "MSPMPUEA")]
pub type Mspmpuea = crate::Reg<mspmpuea::MspmpueaSpec>;
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
pub mod mspmpuea;
#[doc = "PSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuoad`] module"]
#[doc(alias = "PSPMPUOAD")]
pub type Pspmpuoad = crate::Reg<pspmpuoad::PspmpuoadSpec>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod pspmpuoad;
#[doc = "PSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuctl`] module"]
#[doc(alias = "PSPMPUCTL")]
pub type Pspmpuctl = crate::Reg<pspmpuctl::PspmpuctlSpec>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod pspmpuctl;
#[doc = "PSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpupt`] module"]
#[doc(alias = "PSPMPUPT")]
pub type Pspmpupt = crate::Reg<pspmpupt::PspmpuptSpec>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod pspmpupt;
#[doc = "PSPMPUSA (rw) register accessor: Process Stack Pointer (PSP) Monitor Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpusa`] module"]
#[doc(alias = "PSPMPUSA")]
pub type Pspmpusa = crate::Reg<pspmpusa::PspmpusaSpec>;
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
pub mod pspmpusa;
#[doc = "PSPMPUEA (rw) register accessor: Process Stack Pointer (PSP) Monitor End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuea`] module"]
#[doc(alias = "PSPMPUEA")]
pub type Pspmpuea = crate::Reg<pspmpuea::PspmpueaSpec>;
#[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
pub mod pspmpuea;

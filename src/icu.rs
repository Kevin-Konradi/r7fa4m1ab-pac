#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    irqcr: [Irqcr; 13],
    _reserved1: [u8; 0x01],
    irqcr14: Irqcr14,
    irqcr15: Irqcr14,
    _reserved3: [u8; 0xf0],
    nmicr: Nmicr,
    _reserved4: [u8; 0x1f],
    nmier: Nmier,
    _reserved5: [u8; 0x0e],
    nmiclr: Nmiclr,
    _reserved6: [u8; 0x0e],
    nmisr: Nmisr,
    _reserved7: [u8; 0x5e],
    wupen: Wupen,
    _reserved8: [u8; 0x5c],
    selsr0: Selsr0,
    _reserved9: [u8; 0x7e],
    delsr: (),
    _reserved10: [u8; 0x80],
    ielsr: [Ielsr; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x0d - IRQ Control Register %s"]
    #[inline(always)]
    pub const fn irqcr(&self, n: usize) -> &Irqcr {
        &self.irqcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0d - IRQ Control Register %s"]
    #[inline(always)]
    pub fn irqcr_iter(&self) -> impl Iterator<Item = &Irqcr> {
        self.irqcr.iter()
    }
    #[doc = "0x0e - IRQ Control Register 14"]
    #[inline(always)]
    pub const fn irqcr14(&self) -> &Irqcr14 {
        &self.irqcr14
    }
    #[doc = "0x0e - IRQ Control Register 15"]
    #[inline(always)]
    pub const fn irqcr15(&self) -> &Irqcr14 {
        &self.irqcr15
    }
    #[doc = "0x100 - NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &Nmicr {
        &self.nmicr
    }
    #[doc = "0x120 - Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &Nmier {
        &self.nmier
    }
    #[doc = "0x130 - Non-Maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(&self) -> &Nmiclr {
        &self.nmiclr
    }
    #[doc = "0x140 - Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &Nmisr {
        &self.nmisr
    }
    #[doc = "0x1a0 - Wake Up Interrupt Enable Register"]
    #[inline(always)]
    pub const fn wupen(&self) -> &Wupen {
        &self.wupen
    }
    #[doc = "0x200 - SYS Event Link Setting Register"]
    #[inline(always)]
    pub const fn selsr0(&self) -> &Selsr0 {
        &self.selsr0
    }
    #[doc = "0x280..0x288 - DMAC Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn delsr(&self, n: usize) -> &Delsr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x288 - DMAC Event Link Setting Register %s"]
    #[inline(always)]
    pub fn delsr_iter(&self) -> impl Iterator<Item = &Delsr> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x300..0x380 - ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(&self, n: usize) -> &Ielsr {
        &self.ielsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x380 - ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub fn ielsr_iter(&self) -> impl Iterator<Item = &Ielsr> {
        self.ielsr.iter()
    }
}
#[doc = "IRQCR (rw) register accessor: IRQ Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`irqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqcr`] module"]
#[doc(alias = "IRQCR")]
pub type Irqcr = crate::Reg<irqcr::IrqcrSpec>;
#[doc = "IRQ Control Register %s"]
pub mod irqcr;
pub use Irqcr as Irqcr14;
pub use irqcr as irqcr14;
#[doc = "NMISR (r) register accessor: Non-Maskable Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisr`] module"]
#[doc(alias = "NMISR")]
pub type Nmisr = crate::Reg<nmisr::NmisrSpec>;
#[doc = "Non-Maskable Interrupt Status Register"]
pub mod nmisr;
#[doc = "NMIER (rw) register accessor: Non-Maskable Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmier`] module"]
#[doc(alias = "NMIER")]
pub type Nmier = crate::Reg<nmier::NmierSpec>;
#[doc = "Non-Maskable Interrupt Enable Register"]
pub mod nmier;
#[doc = "NMICLR (rw) register accessor: Non-Maskable Interrupt Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiclr`] module"]
#[doc(alias = "NMICLR")]
pub type Nmiclr = crate::Reg<nmiclr::NmiclrSpec>;
#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub mod nmiclr;
#[doc = "NMICR (rw) register accessor: NMI Pin Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicr`] module"]
#[doc(alias = "NMICR")]
pub type Nmicr = crate::Reg<nmicr::NmicrSpec>;
#[doc = "NMI Pin Interrupt Control Register"]
pub mod nmicr;
#[doc = "IELSR (rw) register accessor: ICU Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ielsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ielsr`] module"]
#[doc(alias = "IELSR")]
pub type Ielsr = crate::Reg<ielsr::IelsrSpec>;
#[doc = "ICU Event Link Setting Register %s"]
pub mod ielsr;
#[doc = "DELSR (rw) register accessor: DMAC Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`delsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delsr`] module"]
#[doc(alias = "DELSR")]
pub type Delsr = crate::Reg<delsr::DelsrSpec>;
#[doc = "DMAC Event Link Setting Register %s"]
pub mod delsr;
#[doc = "SELSR0 (rw) register accessor: SYS Event Link Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selsr0`] module"]
#[doc(alias = "SELSR0")]
pub type Selsr0 = crate::Reg<selsr0::Selsr0Spec>;
#[doc = "SYS Event Link Setting Register"]
pub mod selsr0;
#[doc = "WUPEN (rw) register accessor: Wake Up Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupen`] module"]
#[doc(alias = "WUPEN")]
pub type Wupen = crate::Reg<wupen::WupenSpec>;
#[doc = "Wake Up Interrupt Enable Register"]
pub mod wupen;

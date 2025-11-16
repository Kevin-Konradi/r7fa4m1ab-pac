#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    elcr: Elcr,
    _reserved1: [u8; 0x01],
    elsegr: (),
    _reserved2: [u8; 0x0e],
    elsr: (),
    _reserved3: [u8; 0x30],
    elsr12: Elsr12,
    _reserved4: [u8; 0x06],
    elsr14: Elsr14,
    _reserved5: [u8; 0x02],
    elsr15: Elsr14,
    _reserved6: [u8; 0x02],
    elsr16: Elsr14,
    _reserved7: [u8; 0x02],
    elsr17: Elsr14,
    _reserved8: [u8; 0x02],
    elsr18: Elsr14,
}
impl RegisterBlock {
    #[doc = "0x00 - Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(&self) -> &Elcr {
        &self.elcr
    }
    #[doc = "0x02 - Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(&self, n: usize) -> &Elsegr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02 - Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub fn elsegr_iter(&self) -> impl Iterator<Item = &Elsegr> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x10..0x24 - Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn elsr(&self, n: usize) -> &Elsr {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x24 - Event Link Setting Register %s"]
    #[inline(always)]
    pub fn elsr_iter(&self) -> impl Iterator<Item = &Elsr> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x40 - Event Link Setting Register 12"]
    #[inline(always)]
    pub const fn elsr12(&self) -> &Elsr12 {
        &self.elsr12
    }
    #[doc = "0x48 - Event Link Setting Register 14"]
    #[inline(always)]
    pub const fn elsr14(&self) -> &Elsr14 {
        &self.elsr14
    }
    #[doc = "0x48 - Event Link Setting Register 15"]
    #[inline(always)]
    pub const fn elsr15(&self) -> &Elsr14 {
        &self.elsr15
    }
    #[doc = "0x48 - Event Link Setting Register 16"]
    #[inline(always)]
    pub const fn elsr16(&self) -> &Elsr14 {
        &self.elsr16
    }
    #[doc = "0x48 - Event Link Setting Register 17"]
    #[inline(always)]
    pub const fn elsr17(&self) -> &Elsr14 {
        &self.elsr17
    }
    #[doc = "0x48 - Event Link Setting Register 18"]
    #[inline(always)]
    pub const fn elsr18(&self) -> &Elsr14 {
        &self.elsr18
    }
}
#[doc = "ELCR (rw) register accessor: Event Link Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`elcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elcr`] module"]
#[doc(alias = "ELCR")]
pub type Elcr = crate::Reg<elcr::ElcrSpec>;
#[doc = "Event Link Controller Register"]
pub mod elcr;
#[doc = "ELSEGR (rw) register accessor: Event Link Software Event Generation Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsegr`] module"]
#[doc(alias = "ELSEGR")]
pub type Elsegr = crate::Reg<elsegr::ElsegrSpec>;
#[doc = "Event Link Software Event Generation Register %s"]
pub mod elsegr;
#[doc = "ELSR (rw) register accessor: Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsr`] module"]
#[doc(alias = "ELSR")]
pub type Elsr = crate::Reg<elsr::ElsrSpec>;
#[doc = "Event Link Setting Register %s"]
pub mod elsr;
pub use Elsr as Elsr12;
pub use Elsr as Elsr14;
pub use elsr as elsr12;
pub use elsr as elsr14;

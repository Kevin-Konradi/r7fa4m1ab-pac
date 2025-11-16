#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcdm0: Lcdm0,
    lcdm1: Lcdm1,
    lcdc0: Lcdc0,
    vlcd: Vlcd,
    _reserved4: [u8; 0xfc],
    seg: [Seg; 38],
}
impl RegisterBlock {
    #[doc = "0x00 - LCD Mode Register 0"]
    #[inline(always)]
    pub const fn lcdm0(&self) -> &Lcdm0 {
        &self.lcdm0
    }
    #[doc = "0x01 - LCD Mode Register 1"]
    #[inline(always)]
    pub const fn lcdm1(&self) -> &Lcdm1 {
        &self.lcdm1
    }
    #[doc = "0x02 - LCD Clock Control Register 0"]
    #[inline(always)]
    pub const fn lcdc0(&self) -> &Lcdc0 {
        &self.lcdc0
    }
    #[doc = "0x03 - LCD Boost Level Control Register"]
    #[inline(always)]
    pub const fn vlcd(&self) -> &Vlcd {
        &self.vlcd
    }
    #[doc = "0x100..0x126 - LCD Display Data Register %s"]
    #[inline(always)]
    pub const fn seg(&self, n: usize) -> &Seg {
        &self.seg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x126 - LCD Display Data Register %s"]
    #[inline(always)]
    pub fn seg_iter(&self) -> impl Iterator<Item = &Seg> {
        self.seg.iter()
    }
}
#[doc = "LCDM0 (rw) register accessor: LCD Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm0`] module"]
#[doc(alias = "LCDM0")]
pub type Lcdm0 = crate::Reg<lcdm0::Lcdm0Spec>;
#[doc = "LCD Mode Register 0"]
pub mod lcdm0;
#[doc = "LCDM1 (rw) register accessor: LCD Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm1`] module"]
#[doc(alias = "LCDM1")]
pub type Lcdm1 = crate::Reg<lcdm1::Lcdm1Spec>;
#[doc = "LCD Mode Register 1"]
pub mod lcdm1;
#[doc = "LCDC0 (rw) register accessor: LCD Clock Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdc0`] module"]
#[doc(alias = "LCDC0")]
pub type Lcdc0 = crate::Reg<lcdc0::Lcdc0Spec>;
#[doc = "LCD Clock Control Register 0"]
pub mod lcdc0;
#[doc = "VLCD (rw) register accessor: LCD Boost Level Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vlcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlcd`] module"]
#[doc(alias = "VLCD")]
pub type Vlcd = crate::Reg<vlcd::VlcdSpec>;
#[doc = "LCD Boost Level Control Register"]
pub mod vlcd;
#[doc = "SEG (rw) register accessor: LCD Display Data Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`seg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg`] module"]
#[doc(alias = "SEG")]
pub type Seg = crate::Reg<seg::SegSpec>;
#[doc = "LCD Display Data Register %s"]
pub mod seg;

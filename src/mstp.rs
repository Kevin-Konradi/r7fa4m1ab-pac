#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mstpcrb: Mstpcrb,
    mstpcrc: Mstpcrc,
    mstpcrd: Mstpcrd,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Stop Control Register B"]
    #[inline(always)]
    pub const fn mstpcrb(&self) -> &Mstpcrb {
        &self.mstpcrb
    }
    #[doc = "0x04 - Module Stop Control Register C"]
    #[inline(always)]
    pub const fn mstpcrc(&self) -> &Mstpcrc {
        &self.mstpcrc
    }
    #[doc = "0x08 - Module Stop Control Register D"]
    #[inline(always)]
    pub const fn mstpcrd(&self) -> &Mstpcrd {
        &self.mstpcrd
    }
}
#[doc = "MSTPCRB (rw) register accessor: Module Stop Control Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrb`] module"]
#[doc(alias = "MSTPCRB")]
pub type Mstpcrb = crate::Reg<mstpcrb::MstpcrbSpec>;
#[doc = "Module Stop Control Register B"]
pub mod mstpcrb;
#[doc = "MSTPCRC (rw) register accessor: Module Stop Control Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrc`] module"]
#[doc(alias = "MSTPCRC")]
pub type Mstpcrc = crate::Reg<mstpcrc::MstpcrcSpec>;
#[doc = "Module Stop Control Register C"]
pub mod mstpcrc;
#[doc = "MSTPCRD (rw) register accessor: Module Stop Control Register D\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrd`] module"]
#[doc(alias = "MSTPCRD")]
pub type Mstpcrd = crate::Reg<mstpcrd::MstpcrdSpec>;
#[doc = "Module Stop Control Register D"]
pub mod mstpcrd;

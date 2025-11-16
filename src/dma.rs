#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmast: Dmast,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAC Module Activation Register"]
    #[inline(always)]
    pub const fn dmast(&self) -> &Dmast {
        &self.dmast
    }
}
#[doc = "DMAST (rw) register accessor: DMAC Module Activation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmast`] module"]
#[doc(alias = "DMAST")]
pub type Dmast = crate::Reg<dmast::DmastSpec>;
#[doc = "DMAC Module Activation Register"]
pub mod dmast;

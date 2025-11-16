#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opscr: Opscr,
}
impl RegisterBlock {
    #[doc = "0x00 - Output Phase Switching Control Register"]
    #[inline(always)]
    pub const fn opscr(&self) -> &Opscr {
        &self.opscr
    }
}
#[doc = "OPSCR (rw) register accessor: Output Phase Switching Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opscr`] module"]
#[doc(alias = "OPSCR")]
pub type Opscr = crate::Reg<opscr::OpscrSpec>;
#[doc = "Output Phase Switching Control Register"]
pub mod opscr;

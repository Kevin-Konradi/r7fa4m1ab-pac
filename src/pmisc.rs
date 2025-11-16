#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x03],
    pwpr: Pwpr,
}
impl RegisterBlock {
    #[doc = "0x03 - Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &Pwpr {
        &self.pwpr
    }
}
#[doc = "PWPR (rw) register accessor: Write-Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwpr`] module"]
#[doc(alias = "PWPR")]
pub type Pwpr = crate::Reg<pwpr::PwprSpec>;
#[doc = "Write-Protect Register"]
pub mod pwpr;

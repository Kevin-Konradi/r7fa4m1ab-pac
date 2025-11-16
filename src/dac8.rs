#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dacs: [Dacs; 2],
    _reserved1: [u8; 0x01],
    dam: Dam,
}
impl RegisterBlock {
    #[doc = "0x00 - D/A Conversion Value Setting Register %s"]
    #[inline(always)]
    pub const fn dacs(&self, n: usize) -> &Dacs {
        &self.dacs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - D/A Conversion Value Setting Register %s"]
    #[inline(always)]
    pub fn dacs_iter(&self) -> impl Iterator<Item = &Dacs> {
        self.dacs.iter()
    }
    #[doc = "0x03 - D/A Converter Mode Register"]
    #[inline(always)]
    pub const fn dam(&self) -> &Dam {
        &self.dam
    }
}
#[doc = "DACS (rw) register accessor: D/A Conversion Value Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dacs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacs`] module"]
#[doc(alias = "DACS")]
pub type Dacs = crate::Reg<dacs::DacsSpec>;
#[doc = "D/A Conversion Value Setting Register %s"]
pub mod dacs;
#[doc = "DAM (rw) register accessor: D/A Converter Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam`] module"]
#[doc(alias = "DAM")]
pub type Dam = crate::Reg<dam::DamSpec>;
#[doc = "D/A Converter Mode Register"]
pub mod dam;

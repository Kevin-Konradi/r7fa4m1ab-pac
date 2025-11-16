#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0228],
    tscdrl: Tscdrl,
    tscdrh: Tscdrh,
}
impl RegisterBlock {
    #[doc = "0x228 - Temperature Sensor Calibration Data Register L"]
    #[inline(always)]
    pub const fn tscdrl(&self) -> &Tscdrl {
        &self.tscdrl
    }
    #[doc = "0x229 - Temperature Sensor Calibration Data Register H"]
    #[inline(always)]
    pub const fn tscdrh(&self) -> &Tscdrh {
        &self.tscdrh
    }
}
#[doc = "TSCDRH (r) register accessor: Temperature Sensor Calibration Data Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscdrh`] module"]
#[doc(alias = "TSCDRH")]
pub type Tscdrh = crate::Reg<tscdrh::TscdrhSpec>;
#[doc = "Temperature Sensor Calibration Data Register H"]
pub mod tscdrh;
#[doc = "TSCDRL (r) register accessor: Temperature Sensor Calibration Data Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscdrl`] module"]
#[doc(alias = "TSCDRL")]
pub type Tscdrl = crate::Reg<tscdrl::TscdrlSpec>;
#[doc = "Temperature Sensor Calibration Data Register L"]
pub mod tscdrl;

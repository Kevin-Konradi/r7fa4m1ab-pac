#[doc = "Register `TSCDRH` reader"]
pub type R = crate::R<TscdrhSpec>;
#[doc = "Field `TSCDRH` reader - The calibration data stores the higher 8 bits of the converted value."]
pub type TscdrhR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The calibration data stores the higher 8 bits of the converted value."]
    #[inline(always)]
    pub fn tscdrh(&self) -> TscdrhR {
        TscdrhR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCDRH")
            .field("tscdrh", &self.tscdrh())
            .finish()
    }
}
#[doc = "Temperature Sensor Calibration Data Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscdrhSpec;
impl crate::RegisterSpec for TscdrhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tscdrh::R`](R) reader structure"]
impl crate::Readable for TscdrhSpec {}
#[doc = "`reset()` method sets TSCDRH to value 0"]
impl crate::Resettable for TscdrhSpec {}

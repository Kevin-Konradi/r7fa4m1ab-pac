#[doc = "Register `TSCDRL` reader"]
pub type R = crate::R<TscdrlSpec>;
#[doc = "Field `TSCDRL` reader - The calibration data stores the lower 8 bits of the converted value."]
pub type TscdrlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The calibration data stores the lower 8 bits of the converted value."]
    #[inline(always)]
    pub fn tscdrl(&self) -> TscdrlR {
        TscdrlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCDRL")
            .field("tscdrl", &self.tscdrl())
            .finish()
    }
}
#[doc = "Temperature Sensor Calibration Data Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscdrlSpec;
impl crate::RegisterSpec for TscdrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tscdrl::R`](R) reader structure"]
impl crate::Readable for TscdrlSpec {}
#[doc = "`reset()` method sets TSCDRL to value 0"]
impl crate::Resettable for TscdrlSpec {}

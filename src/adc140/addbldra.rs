#[doc = "Register `ADDBLDRA` reader"]
pub type R = crate::R<AddbldraSpec>;
#[doc = "Field `ADDBLDRA` reader - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
pub type AddbldraR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
    #[inline(always)]
    pub fn addbldra(&self) -> AddbldraR {
        AddbldraR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDBLDRA")
            .field("addbldra", &self.addbldra())
            .finish()
    }
}
#[doc = "A/D Data Duplexing Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldra::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddbldraSpec;
impl crate::RegisterSpec for AddbldraSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addbldra::R`](R) reader structure"]
impl crate::Readable for AddbldraSpec {}
#[doc = "`reset()` method sets ADDBLDRA to value 0"]
impl crate::Resettable for AddbldraSpec {}

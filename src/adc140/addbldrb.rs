#[doc = "Register `ADDBLDRB` reader"]
pub type R = crate::R<AddbldrbSpec>;
#[doc = "Field `ADDBLDRB` reader - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
pub type AddbldrbR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
    #[inline(always)]
    pub fn addbldrb(&self) -> AddbldrbR {
        AddbldrbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDBLDRB")
            .field("addbldrb", &self.addbldrb())
            .finish()
    }
}
#[doc = "A/D Data Duplexing Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldrb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddbldrbSpec;
impl crate::RegisterSpec for AddbldrbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addbldrb::R`](R) reader structure"]
impl crate::Readable for AddbldrbSpec {}
#[doc = "`reset()` method sets ADDBLDRB to value 0"]
impl crate::Resettable for AddbldrbSpec {}

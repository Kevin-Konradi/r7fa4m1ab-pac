#[doc = "Register `RECR` reader"]
pub type R = crate::R<RecrSpec>;
#[doc = "Field `RECR` reader - Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
pub type RecrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive error count function RECR increments or decrements the counter value according to the error status of the CAN module during reception."]
    #[inline(always)]
    pub fn recr(&self) -> RecrR {
        RecrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECR").field("recr", &self.recr()).finish()
    }
}
#[doc = "Receive Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`recr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RecrSpec;
impl crate::RegisterSpec for RecrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`recr::R`](R) reader structure"]
impl crate::Readable for RecrSpec {}
#[doc = "`reset()` method sets RECR to value 0"]
impl crate::Resettable for RecrSpec {}

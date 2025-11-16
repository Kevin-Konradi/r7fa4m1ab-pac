#[doc = "Register `BCNT0CP%s` reader"]
pub type R = crate::R<Bcnt0cpSpec>;
#[doc = "Field `BCNT0CP` reader - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
pub type Bcnt0cpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt0cp(&self) -> Bcnt0cpR {
        Bcnt0cpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT0CP")
            .field("bcnt0cp", &self.bcnt0cp())
            .finish()
    }
}
#[doc = "BCNT0 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt0cpSpec;
impl crate::RegisterSpec for Bcnt0cpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0cp::R`](R) reader structure"]
impl crate::Readable for Bcnt0cpSpec {}
#[doc = "`reset()` method sets BCNT0CP%s to value 0"]
impl crate::Resettable for Bcnt0cpSpec {}

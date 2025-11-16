#[doc = "Register `BCNT1CP%s` reader"]
pub type R = crate::R<Bcnt1cpSpec>;
#[doc = "Field `BCNT1CP` reader - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
pub type Bcnt1cpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt1cp(&self) -> Bcnt1cpR {
        Bcnt1cpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT1CP")
            .field("bcnt1cp", &self.bcnt1cp())
            .finish()
    }
}
#[doc = "BCNT1 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt1cpSpec;
impl crate::RegisterSpec for Bcnt1cpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt1cp::R`](R) reader structure"]
impl crate::Readable for Bcnt1cpSpec {}
#[doc = "`reset()` method sets BCNT1CP%s to value 0"]
impl crate::Resettable for Bcnt1cpSpec {}

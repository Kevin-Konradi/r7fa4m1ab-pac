#[doc = "Register `BCNT2CP%s` reader"]
pub type R = crate::R<Bcnt2cpSpec>;
#[doc = "Field `BCNT2CP` reader - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
pub type Bcnt2cpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt2cp(&self) -> Bcnt2cpR {
        Bcnt2cpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT2CP")
            .field("bcnt2cp", &self.bcnt2cp())
            .finish()
    }
}
#[doc = "BCNT2 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt2cpSpec;
impl crate::RegisterSpec for Bcnt2cpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt2cp::R`](R) reader structure"]
impl crate::Readable for Bcnt2cpSpec {}
#[doc = "`reset()` method sets BCNT2CP%s to value 0"]
impl crate::Resettable for Bcnt2cpSpec {}

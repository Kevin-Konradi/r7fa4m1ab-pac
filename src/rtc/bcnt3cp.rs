#[doc = "Register `BCNT3CP%s` reader"]
pub type R = crate::R<Bcnt3cpSpec>;
#[doc = "Field `BCNT3CP` reader - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
pub type Bcnt3cpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt3cp(&self) -> Bcnt3cpR {
        Bcnt3cpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT3CP")
            .field("bcnt3cp", &self.bcnt3cp())
            .finish()
    }
}
#[doc = "BCNT3 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt3cpSpec;
impl crate::RegisterSpec for Bcnt3cpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3cp::R`](R) reader structure"]
impl crate::Readable for Bcnt3cpSpec {}
#[doc = "`reset()` method sets BCNT3CP%s to value 0"]
impl crate::Resettable for Bcnt3cpSpec {}

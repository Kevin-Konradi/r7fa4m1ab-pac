#[doc = "Register `SSIFRDR` reader"]
pub type R = crate::R<SsifrdrSpec>;
#[doc = "Field `SSIFRDR` reader - Receive FIFO data."]
pub type SsifrdrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO data."]
    #[inline(always)]
    pub fn ssifrdr(&self) -> SsifrdrR {
        SsifrdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSIFRDR")
            .field("ssifrdr", &self.ssifrdr())
            .finish()
    }
}
#[doc = "Receive FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsifrdrSpec;
impl crate::RegisterSpec for SsifrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifrdr::R`](R) reader structure"]
impl crate::Readable for SsifrdrSpec {}
#[doc = "`reset()` method sets SSIFRDR to value 0"]
impl crate::Resettable for SsifrdrSpec {}

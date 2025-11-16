#[doc = "Register `TSR` reader"]
pub type R = crate::R<TsrSpec>;
#[doc = "Field `TSR` reader - Free-running counter value for the time stamp function"]
pub type TsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Free-running counter value for the time stamp function"]
    #[inline(always)]
    pub fn tsr(&self) -> TsrR {
        TsrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSR").field("tsr", &self.tsr()).finish()
    }
}
#[doc = "Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsrSpec;
impl crate::RegisterSpec for TsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TsrSpec {}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TsrSpec {}

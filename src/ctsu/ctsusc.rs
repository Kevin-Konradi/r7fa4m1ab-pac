#[doc = "Register `CTSUSC` reader"]
pub type R = crate::R<CtsuscSpec>;
#[doc = "Field `CTSUSC` reader - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
pub type CtsuscR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsusc(&self) -> CtsuscR {
        CtsuscR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSC")
            .field("ctsusc", &self.ctsusc())
            .finish()
    }
}
#[doc = "CTSU Sensor Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsuscSpec;
impl crate::RegisterSpec for CtsuscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsusc::R`](R) reader structure"]
impl crate::Readable for CtsuscSpec {}
#[doc = "`reset()` method sets CTSUSC to value 0"]
impl crate::Resettable for CtsuscSpec {}

#[doc = "Register `CTSURC` reader"]
pub type R = crate::R<CtsurcSpec>;
#[doc = "Field `CTSURC` reader - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
pub type CtsurcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsurc(&self) -> CtsurcR {
        CtsurcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSURC")
            .field("ctsurc", &self.ctsurc())
            .finish()
    }
}
#[doc = "CTSU Reference Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsurc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsurcSpec;
impl crate::RegisterSpec for CtsurcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsurc::R`](R) reader structure"]
impl crate::Readable for CtsurcSpec {}
#[doc = "`reset()` method sets CTSURC to value 0"]
impl crate::Resettable for CtsurcSpec {}

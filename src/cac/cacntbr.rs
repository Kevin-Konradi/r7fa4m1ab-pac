#[doc = "Register `CACNTBR` reader"]
pub type R = crate::R<CacntbrSpec>;
#[doc = "Field `CACNTBR` reader - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
pub type CacntbrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
    #[inline(always)]
    pub fn cacntbr(&self) -> CacntbrR {
        CacntbrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACNTBR")
            .field("cacntbr", &self.cacntbr())
            .finish()
    }
}
#[doc = "CAC Counter Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cacntbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacntbrSpec;
impl crate::RegisterSpec for CacntbrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cacntbr::R`](R) reader structure"]
impl crate::Readable for CacntbrSpec {}
#[doc = "`reset()` method sets CACNTBR to value 0"]
impl crate::Resettable for CacntbrSpec {}

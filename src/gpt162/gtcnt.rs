#[doc = "Register `GTCNT` reader"]
pub type R = crate::R<GtcntSpec>;
#[doc = "Register `GTCNT` writer"]
pub type W = crate::W<GtcntSpec>;
#[doc = "Field `GTCNT` reader - Counter"]
pub type GtcntR = crate::FieldReader<u32>;
#[doc = "Field `GTCNT` writer - Counter"]
pub type GtcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    pub fn gtcnt(&self) -> GtcntR {
        GtcntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCNT")
            .field("gtcnt", &self.gtcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    pub fn gtcnt(&mut self) -> GtcntW<GtcntSpec> {
        GtcntW::new(self, 0)
    }
}
#[doc = "General PWM Timer Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcntSpec;
impl crate::RegisterSpec for GtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcnt::R`](R) reader structure"]
impl crate::Readable for GtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`gtcnt::W`](W) writer structure"]
impl crate::Writable for GtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCNT to value 0"]
impl crate::Resettable for GtcntSpec {}

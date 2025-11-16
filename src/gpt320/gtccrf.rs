#[doc = "Register `GTCCRF` reader"]
pub type R = crate::R<GtccrfSpec>;
#[doc = "Register `GTCCRF` writer"]
pub type W = crate::W<GtccrfSpec>;
#[doc = "Field `GTCCRF` reader - Compare Capture Register F"]
pub type GtccrfR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRF` writer - Compare Capture Register F"]
pub type GtccrfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register F"]
    #[inline(always)]
    pub fn gtccrf(&self) -> GtccrfR {
        GtccrfR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRF")
            .field("gtccrf", &self.gtccrf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register F"]
    #[inline(always)]
    pub fn gtccrf(&mut self) -> GtccrfW<GtccrfSpec> {
        GtccrfW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register F\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccrfSpec;
impl crate::RegisterSpec for GtccrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrf::R`](R) reader structure"]
impl crate::Readable for GtccrfSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccrf::W`](W) writer structure"]
impl crate::Writable for GtccrfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRF to value 0xffff_ffff"]
impl crate::Resettable for GtccrfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

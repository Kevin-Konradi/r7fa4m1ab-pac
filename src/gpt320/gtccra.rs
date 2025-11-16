#[doc = "Register `GTCCRA` reader"]
pub type R = crate::R<GtccraSpec>;
#[doc = "Register `GTCCRA` writer"]
pub type W = crate::W<GtccraSpec>;
#[doc = "Field `GTCCRA` reader - Compare Capture Register A"]
pub type GtccraR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRA` writer - Compare Capture Register A"]
pub type GtccraW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register A"]
    #[inline(always)]
    pub fn gtccra(&self) -> GtccraR {
        GtccraR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRA")
            .field("gtccra", &self.gtccra())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register A"]
    #[inline(always)]
    pub fn gtccra(&mut self) -> GtccraW<GtccraSpec> {
        GtccraW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccraSpec;
impl crate::RegisterSpec for GtccraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccra::R`](R) reader structure"]
impl crate::Readable for GtccraSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccra::W`](W) writer structure"]
impl crate::Writable for GtccraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRA to value 0xffff_ffff"]
impl crate::Resettable for GtccraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

#[doc = "Register `GTCCRE` reader"]
pub type R = crate::R<GtccreSpec>;
#[doc = "Register `GTCCRE` writer"]
pub type W = crate::W<GtccreSpec>;
#[doc = "Field `GTCCRE` reader - Compare Capture Register E"]
pub type GtccreR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRE` writer - Compare Capture Register E"]
pub type GtccreW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register E"]
    #[inline(always)]
    pub fn gtccre(&self) -> GtccreR {
        GtccreR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRE")
            .field("gtccre", &self.gtccre())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register E"]
    #[inline(always)]
    pub fn gtccre(&mut self) -> GtccreW<GtccreSpec> {
        GtccreW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register E\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccreSpec;
impl crate::RegisterSpec for GtccreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccre::R`](R) reader structure"]
impl crate::Readable for GtccreSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccre::W`](W) writer structure"]
impl crate::Writable for GtccreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRE to value 0xffff"]
impl crate::Resettable for GtccreSpec {
    const RESET_VALUE: u32 = 0xffff;
}

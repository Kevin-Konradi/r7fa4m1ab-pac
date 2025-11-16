#[doc = "Register `GTCCRC` reader"]
pub type R = crate::R<GtccrcSpec>;
#[doc = "Register `GTCCRC` writer"]
pub type W = crate::W<GtccrcSpec>;
#[doc = "Field `GTCCRC` reader - Compare Capture Register C"]
pub type GtccrcR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRC` writer - Compare Capture Register C"]
pub type GtccrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register C"]
    #[inline(always)]
    pub fn gtccrc(&self) -> GtccrcR {
        GtccrcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRC")
            .field("gtccrc", &self.gtccrc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register C"]
    #[inline(always)]
    pub fn gtccrc(&mut self) -> GtccrcW<GtccrcSpec> {
        GtccrcW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccrcSpec;
impl crate::RegisterSpec for GtccrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrc::R`](R) reader structure"]
impl crate::Readable for GtccrcSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccrc::W`](W) writer structure"]
impl crate::Writable for GtccrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRC to value 0xffff"]
impl crate::Resettable for GtccrcSpec {
    const RESET_VALUE: u32 = 0xffff;
}

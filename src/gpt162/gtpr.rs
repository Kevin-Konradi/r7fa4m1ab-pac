#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GtprSpec>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GtprSpec>;
#[doc = "Field `GTPR` reader - Cycle Setting Register"]
pub type GtprR = crate::FieldReader<u32>;
#[doc = "Field `GTPR` writer - Cycle Setting Register"]
pub type GtprW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle Setting Register"]
    #[inline(always)]
    pub fn gtpr(&self) -> GtprR {
        GtprR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPR").field("gtpr", &self.gtpr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cycle Setting Register"]
    #[inline(always)]
    pub fn gtpr(&mut self) -> GtprW<GtprSpec> {
        GtprW::new(self, 0)
    }
}
#[doc = "General PWM Timer Cycle Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtprSpec;
impl crate::RegisterSpec for GtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GtprSpec {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTPR to value 0xffff"]
impl crate::Resettable for GtprSpec {
    const RESET_VALUE: u32 = 0xffff;
}

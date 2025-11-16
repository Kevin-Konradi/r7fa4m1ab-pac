#[doc = "Register `GTCCRD` reader"]
pub type R = crate::R<GtccrdSpec>;
#[doc = "Register `GTCCRD` writer"]
pub type W = crate::W<GtccrdSpec>;
#[doc = "Field `GTCCRD` reader - Compare Capture Register D"]
pub type GtccrdR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRD` writer - Compare Capture Register D"]
pub type GtccrdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    pub fn gtccrd(&self) -> GtccrdR {
        GtccrdR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRD")
            .field("gtccrd", &self.gtccrd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    pub fn gtccrd(&mut self) -> GtccrdW<GtccrdSpec> {
        GtccrdW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register D\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccrdSpec;
impl crate::RegisterSpec for GtccrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrd::R`](R) reader structure"]
impl crate::Readable for GtccrdSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccrd::W`](W) writer structure"]
impl crate::Writable for GtccrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRD to value 0xffff"]
impl crate::Resettable for GtccrdSpec {
    const RESET_VALUE: u32 = 0xffff;
}

#[doc = "Register `GTCCRB` reader"]
pub type R = crate::R<GtccrbSpec>;
#[doc = "Register `GTCCRB` writer"]
pub type W = crate::W<GtccrbSpec>;
#[doc = "Field `GTCCRB` reader - Compare Capture Register B"]
pub type GtccrbR = crate::FieldReader<u32>;
#[doc = "Field `GTCCRB` writer - Compare Capture Register B"]
pub type GtccrbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    pub fn gtccrb(&self) -> GtccrbR {
        GtccrbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCCRB")
            .field("gtccrb", &self.gtccrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    pub fn gtccrb(&mut self) -> GtccrbW<GtccrbSpec> {
        GtccrbW::new(self, 0)
    }
}
#[doc = "General PWM Timer Compare Capture Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gtccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtccrbSpec;
impl crate::RegisterSpec for GtccrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrb::R`](R) reader structure"]
impl crate::Readable for GtccrbSpec {}
#[doc = "`write(|w| ..)` method takes [`gtccrb::W`](W) writer structure"]
impl crate::Writable for GtccrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCCRB to value 0xffff_ffff"]
impl crate::Resettable for GtccrbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

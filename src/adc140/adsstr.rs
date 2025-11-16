#[doc = "Register `ADSSTR%s` reader"]
pub type R = crate::R<AdsstrSpec>;
#[doc = "Register `ADSSTR%s` writer"]
pub type W = crate::W<AdsstrSpec>;
#[doc = "Field `SST` reader - Sampling time setting"]
pub type SstR = crate::FieldReader;
#[doc = "Field `SST` writer - Sampling time setting"]
pub type SstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sampling time setting"]
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSSTR").field("sst", &self.sst()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling time setting"]
    #[inline(always)]
    pub fn sst(&mut self) -> SstW<AdsstrSpec> {
        SstW::new(self, 0)
    }
}
#[doc = "A/D Sampling State Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdsstrSpec;
impl crate::RegisterSpec for AdsstrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adsstr::R`](R) reader structure"]
impl crate::Readable for AdsstrSpec {}
#[doc = "`write(|w| ..)` method takes [`adsstr::W`](W) writer structure"]
impl crate::Writable for AdsstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADSSTR%s to value 0x0d"]
impl crate::Resettable for AdsstrSpec {
    const RESET_VALUE: u8 = 0x0d;
}

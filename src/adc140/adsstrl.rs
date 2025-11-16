#[doc = "Register `ADSSTRL` reader"]
pub type R = crate::R<AdsstrlSpec>;
#[doc = "Register `ADSSTRL` writer"]
pub type W = crate::W<AdsstrlSpec>;
#[doc = "Field `SST` reader - Sampling Time Setting (AN016-AN027)"]
pub type SstR = crate::FieldReader;
#[doc = "Field `SST` writer - Sampling Time Setting (AN016-AN027)"]
pub type SstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sampling Time Setting (AN016-AN027)"]
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSSTRL").field("sst", &self.sst()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling Time Setting (AN016-AN027)"]
    #[inline(always)]
    pub fn sst(&mut self) -> SstW<AdsstrlSpec> {
        SstW::new(self, 0)
    }
}
#[doc = "A/D Sampling State Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdsstrlSpec;
impl crate::RegisterSpec for AdsstrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adsstrl::R`](R) reader structure"]
impl crate::Readable for AdsstrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adsstrl::W`](W) writer structure"]
impl crate::Writable for AdsstrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADSSTRL to value 0x0d"]
impl crate::Resettable for AdsstrlSpec {
    const RESET_VALUE: u8 = 0x0d;
}

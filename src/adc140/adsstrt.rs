#[doc = "Register `ADSSTRT` reader"]
pub type R = crate::R<AdsstrtSpec>;
#[doc = "Register `ADSSTRT` writer"]
pub type W = crate::W<AdsstrtSpec>;
#[doc = "Field `SST` reader - Sampling Time Setting (temperature sensor output)"]
pub type SstR = crate::FieldReader;
#[doc = "Field `SST` writer - Sampling Time Setting (temperature sensor output)"]
pub type SstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sampling Time Setting (temperature sensor output)"]
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSSTRT").field("sst", &self.sst()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling Time Setting (temperature sensor output)"]
    #[inline(always)]
    pub fn sst(&mut self) -> SstW<AdsstrtSpec> {
        SstW::new(self, 0)
    }
}
#[doc = "A/D Sampling State Register T\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdsstrtSpec;
impl crate::RegisterSpec for AdsstrtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adsstrt::R`](R) reader structure"]
impl crate::Readable for AdsstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`adsstrt::W`](W) writer structure"]
impl crate::Writable for AdsstrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADSSTRT to value 0x0d"]
impl crate::Resettable for AdsstrtSpec {
    const RESET_VALUE: u8 = 0x0d;
}

#[doc = "Register `DMDAR` reader"]
pub type R = crate::R<DmdarSpec>;
#[doc = "Register `DMDAR` writer"]
pub type W = crate::W<DmdarSpec>;
#[doc = "Field `DMDAR` reader - Specifies the transfer destination start address."]
pub type DmdarR = crate::FieldReader<u32>;
#[doc = "Field `DMDAR` writer - Specifies the transfer destination start address."]
pub type DmdarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the transfer destination start address."]
    #[inline(always)]
    pub fn dmdar(&self) -> DmdarR {
        DmdarR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMDAR")
            .field("dmdar", &self.dmdar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the transfer destination start address."]
    #[inline(always)]
    pub fn dmdar(&mut self) -> DmdarW<DmdarSpec> {
        DmdarW::new(self, 0)
    }
}
#[doc = "DMA Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmdar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmdarSpec;
impl crate::RegisterSpec for DmdarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmdar::R`](R) reader structure"]
impl crate::Readable for DmdarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmdar::W`](W) writer structure"]
impl crate::Writable for DmdarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMDAR to value 0"]
impl crate::Resettable for DmdarSpec {}

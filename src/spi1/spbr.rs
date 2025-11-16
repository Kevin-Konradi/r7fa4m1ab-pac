#[doc = "Register `SPBR` reader"]
pub type R = crate::R<SpbrSpec>;
#[doc = "Register `SPBR` writer"]
pub type W = crate::W<SpbrSpec>;
#[doc = "Field `SPR` reader - SPBR sets the bit rate in master mode."]
pub type SprR = crate::FieldReader;
#[doc = "Field `SPR` writer - SPBR sets the bit rate in master mode."]
pub type SprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    pub fn spr(&self) -> SprR {
        SprR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPBR").field("spr", &self.spr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    pub fn spr(&mut self) -> SprW<SpbrSpec> {
        SprW::new(self, 0)
    }
}
#[doc = "SPI Bit Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpbrSpec;
impl crate::RegisterSpec for SpbrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spbr::R`](R) reader structure"]
impl crate::Readable for SpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`spbr::W`](W) writer structure"]
impl crate::Writable for SpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPBR to value 0xff"]
impl crate::Resettable for SpbrSpec {
    const RESET_VALUE: u8 = 0xff;
}

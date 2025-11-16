#[doc = "Register `SPDR` reader"]
pub type R = crate::R<SpdrSpec>;
#[doc = "Register `SPDR` writer"]
pub type W = crate::W<SpdrSpec>;
#[doc = "Field `SPDR` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SpdrR = crate::FieldReader<u32>;
#[doc = "Field `SPDR` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SpdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    pub fn spdr(&self) -> SpdrR {
        SpdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDR").field("spdr", &self.spdr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    pub fn spdr(&mut self) -> SpdrW<SpdrSpec> {
        SpdrW::new(self, 0)
    }
}
#[doc = "SPI Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdrSpec;
impl crate::RegisterSpec for SpdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdr::R`](R) reader structure"]
impl crate::Readable for SpdrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdr::W`](W) writer structure"]
impl crate::Writable for SpdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDR to value 0"]
impl crate::Resettable for SpdrSpec {}

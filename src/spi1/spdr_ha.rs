#[doc = "Register `SPDR_HA` reader"]
pub type R = crate::R<SpdrHaSpec>;
#[doc = "Register `SPDR_HA` writer"]
pub type W = crate::W<SpdrHaSpec>;
#[doc = "Field `SPDR_HA` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
pub type SpdrHaR = crate::FieldReader<u16>;
#[doc = "Field `SPDR_HA` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
pub type SpdrHaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[inline(always)]
    pub fn spdr_ha(&self) -> SpdrHaR {
        SpdrHaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDR_HA")
            .field("spdr_ha", &self.spdr_ha())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[inline(always)]
    pub fn spdr_ha(&mut self) -> SpdrHaW<SpdrHaSpec> {
        SpdrHaW::new(self, 0)
    }
}
#[doc = "SPI Data Register ( halfword access )\n\nYou can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdrHaSpec;
impl crate::RegisterSpec for SpdrHaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdr_ha::R`](R) reader structure"]
impl crate::Readable for SpdrHaSpec {}
#[doc = "`write(|w| ..)` method takes [`spdr_ha::W`](W) writer structure"]
impl crate::Writable for SpdrHaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDR_HA to value 0"]
impl crate::Resettable for SpdrHaSpec {}

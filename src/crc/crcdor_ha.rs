#[doc = "Register `CRCDOR_HA` reader"]
pub type R = crate::R<CrcdorHaSpec>;
#[doc = "Register `CRCDOR_HA` writer"]
pub type W = crate::W<CrcdorHaSpec>;
#[doc = "Field `CRCDOR_HA` reader - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CrcdorHaR = crate::FieldReader<u16>;
#[doc = "Field `CRCDOR_HA` writer - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CrcdorHaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdor_ha(&self) -> CrcdorHaR {
        CrcdorHaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDOR_HA")
            .field("crcdor_ha", &self.crcdor_ha())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdor_ha(&mut self) -> CrcdorHaW<CrcdorHaSpec> {
        CrcdorHaW::new(self, 0)
    }
}
#[doc = "CRC Data Output Register (halfword access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdorHaSpec;
impl crate::RegisterSpec for CrcdorHaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdor_ha::R`](R) reader structure"]
impl crate::Readable for CrcdorHaSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdor_ha::W`](W) writer structure"]
impl crate::Writable for CrcdorHaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDOR_HA to value 0"]
impl crate::Resettable for CrcdorHaSpec {}

#[doc = "Register `DACS%s` reader"]
pub type R = crate::R<DacsSpec>;
#[doc = "Register `DACS%s` writer"]
pub type W = crate::W<DacsSpec>;
#[doc = "Field `DACS` reader - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
pub type DacsR = crate::FieldReader;
#[doc = "Field `DACS` writer - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
pub type DacsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
    #[inline(always)]
    pub fn dacs(&self) -> DacsR {
        DacsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DACS").field("dacs", &self.dacs()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
    #[inline(always)]
    pub fn dacs(&mut self) -> DacsW<DacsSpec> {
        DacsW::new(self, 0)
    }
}
#[doc = "D/A Conversion Value Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dacs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacsSpec;
impl crate::RegisterSpec for DacsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dacs::R`](R) reader structure"]
impl crate::Readable for DacsSpec {}
#[doc = "`write(|w| ..)` method takes [`dacs::W`](W) writer structure"]
impl crate::Writable for DacsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACS%s to value 0"]
impl crate::Resettable for DacsSpec {}

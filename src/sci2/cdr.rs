#[doc = "Register `CDR` reader"]
pub type R = crate::R<CdrSpec>;
#[doc = "Register `CDR` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `CMPD` reader - Compare Match Data Compare data pattern for address match wake-up function"]
pub type CmpdR = crate::FieldReader<u16>;
#[doc = "Field `CMPD` writer - Compare Match Data Compare data pattern for address match wake-up function"]
pub type CmpdW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Compare Match Data Compare data pattern for address match wake-up function"]
    #[inline(always)]
    pub fn cmpd(&self) -> CmpdR {
        CmpdR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR").field("cmpd", &self.cmpd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Compare Match Data Compare data pattern for address match wake-up function"]
    #[inline(always)]
    pub fn cmpd(&mut self) -> CmpdW<CdrSpec> {
        CmpdW::new(self, 0)
    }
}
#[doc = "Compare Match Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {}

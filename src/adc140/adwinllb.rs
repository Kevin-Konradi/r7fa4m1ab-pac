#[doc = "Register `ADWINLLB` reader"]
pub type R = crate::R<AdwinllbSpec>;
#[doc = "Register `ADWINLLB` writer"]
pub type W = crate::W<AdwinllbSpec>;
#[doc = "Field `ADWINLLB` reader - This register is used to compare A window function is used to set the lower level of the window B."]
pub type AdwinllbR = crate::FieldReader<u16>;
#[doc = "Field `ADWINLLB` writer - This register is used to compare A window function is used to set the lower level of the window B."]
pub type AdwinllbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    pub fn adwinllb(&self) -> AdwinllbR {
        AdwinllbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADWINLLB")
            .field("adwinllb", &self.adwinllb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    pub fn adwinllb(&mut self) -> AdwinllbW<AdwinllbSpec> {
        AdwinllbW::new(self, 0)
    }
}
#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinllb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdwinllbSpec;
impl crate::RegisterSpec for AdwinllbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adwinllb::R`](R) reader structure"]
impl crate::Readable for AdwinllbSpec {}
#[doc = "`write(|w| ..)` method takes [`adwinllb::W`](W) writer structure"]
impl crate::Writable for AdwinllbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADWINLLB to value 0"]
impl crate::Resettable for AdwinllbSpec {}

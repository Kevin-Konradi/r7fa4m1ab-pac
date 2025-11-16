#[doc = "Register `ADWINULB` reader"]
pub type R = crate::R<AdwinulbSpec>;
#[doc = "Register `ADWINULB` writer"]
pub type W = crate::W<AdwinulbSpec>;
#[doc = "Field `ADWINULB` reader - This register is used to compare A window function is used to set the higher level of the window B."]
pub type AdwinulbR = crate::FieldReader<u16>;
#[doc = "Field `ADWINULB` writer - This register is used to compare A window function is used to set the higher level of the window B."]
pub type AdwinulbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B."]
    #[inline(always)]
    pub fn adwinulb(&self) -> AdwinulbR {
        AdwinulbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADWINULB")
            .field("adwinulb", &self.adwinulb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B."]
    #[inline(always)]
    pub fn adwinulb(&mut self) -> AdwinulbW<AdwinulbSpec> {
        AdwinulbW::new(self, 0)
    }
}
#[doc = "A/D Compare Function Window B Upper-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinulb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdwinulbSpec;
impl crate::RegisterSpec for AdwinulbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adwinulb::R`](R) reader structure"]
impl crate::Readable for AdwinulbSpec {}
#[doc = "`write(|w| ..)` method takes [`adwinulb::W`](W) writer structure"]
impl crate::Writable for AdwinulbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADWINULB to value 0"]
impl crate::Resettable for AdwinulbSpec {}

#[doc = "Register `ADCMPDR1` reader"]
pub type R = crate::R<Adcmpdr1Spec>;
#[doc = "Register `ADCMPDR1` writer"]
pub type W = crate::W<Adcmpdr1Spec>;
#[doc = "Field `ADCMPDR1` reader - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type Adcmpdr1R = crate::FieldReader<u16>;
#[doc = "Field `ADCMPDR1` writer - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type Adcmpdr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    pub fn adcmpdr1(&self) -> Adcmpdr1R {
        Adcmpdr1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPDR1")
            .field("adcmpdr1", &self.adcmpdr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    pub fn adcmpdr1(&mut self) -> Adcmpdr1W<Adcmpdr1Spec> {
        Adcmpdr1W::new(self, 0)
    }
}
#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmpdr1Spec;
impl crate::RegisterSpec for Adcmpdr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpdr1::R`](R) reader structure"]
impl crate::Readable for Adcmpdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmpdr1::W`](W) writer structure"]
impl crate::Writable for Adcmpdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPDR1 to value 0"]
impl crate::Resettable for Adcmpdr1Spec {}

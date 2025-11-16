#[doc = "Register `ADCMPDR0` reader"]
pub type R = crate::R<Adcmpdr0Spec>;
#[doc = "Register `ADCMPDR0` writer"]
pub type W = crate::W<Adcmpdr0Spec>;
#[doc = "Field `ADCMPDR0` reader - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type Adcmpdr0R = crate::FieldReader<u16>;
#[doc = "Field `ADCMPDR0` writer - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type Adcmpdr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr0(&self) -> Adcmpdr0R {
        Adcmpdr0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPDR0")
            .field("adcmpdr0", &self.adcmpdr0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr0(&mut self) -> Adcmpdr0W<Adcmpdr0Spec> {
        Adcmpdr0W::new(self, 0)
    }
}
#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmpdr0Spec;
impl crate::RegisterSpec for Adcmpdr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpdr0::R`](R) reader structure"]
impl crate::Readable for Adcmpdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmpdr0::W`](W) writer structure"]
impl crate::Writable for Adcmpdr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPDR0 to value 0"]
impl crate::Resettable for Adcmpdr0Spec {}

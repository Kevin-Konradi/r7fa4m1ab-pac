#[doc = "Register `BCNT0` reader"]
pub type R = crate::R<Bcnt0Spec>;
#[doc = "Register `BCNT0` writer"]
pub type W = crate::W<Bcnt0Spec>;
#[doc = "Field `BCNT0` reader - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
pub type Bcnt0R = crate::FieldReader;
#[doc = "Field `BCNT0` writer - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
pub type Bcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0(&self) -> Bcnt0R {
        Bcnt0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT0")
            .field("bcnt0", &self.bcnt0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0(&mut self) -> Bcnt0W<Bcnt0Spec> {
        Bcnt0W::new(self, 0)
    }
}
#[doc = "Binary Counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt0Spec;
impl crate::RegisterSpec for Bcnt0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0::R`](R) reader structure"]
impl crate::Readable for Bcnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`bcnt0::W`](W) writer structure"]
impl crate::Writable for Bcnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT0 to value 0"]
impl crate::Resettable for Bcnt0Spec {}

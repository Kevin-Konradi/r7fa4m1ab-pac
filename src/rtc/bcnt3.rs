#[doc = "Register `BCNT3` reader"]
pub type R = crate::R<Bcnt3Spec>;
#[doc = "Register `BCNT3` writer"]
pub type W = crate::W<Bcnt3Spec>;
#[doc = "Field `BCNT3` reader - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type Bcnt3R = crate::FieldReader;
#[doc = "Field `BCNT3` writer - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type Bcnt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3(&self) -> Bcnt3R {
        Bcnt3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT3")
            .field("bcnt3", &self.bcnt3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3(&mut self) -> Bcnt3W<Bcnt3Spec> {
        Bcnt3W::new(self, 0)
    }
}
#[doc = "Binary Counter 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt3Spec;
impl crate::RegisterSpec for Bcnt3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3::R`](R) reader structure"]
impl crate::Readable for Bcnt3Spec {}
#[doc = "`write(|w| ..)` method takes [`bcnt3::W`](W) writer structure"]
impl crate::Writable for Bcnt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT3 to value 0"]
impl crate::Resettable for Bcnt3Spec {}

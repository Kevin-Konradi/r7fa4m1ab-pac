#[doc = "Register `BCNT1` reader"]
pub type R = crate::R<Bcnt1Spec>;
#[doc = "Register `BCNT1` writer"]
pub type W = crate::W<Bcnt1Spec>;
#[doc = "Field `BCNT1` reader - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
pub type Bcnt1R = crate::FieldReader;
#[doc = "Field `BCNT1` writer - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
pub type Bcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1(&self) -> Bcnt1R {
        Bcnt1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT1")
            .field("bcnt1", &self.bcnt1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1(&mut self) -> Bcnt1W<Bcnt1Spec> {
        Bcnt1W::new(self, 0)
    }
}
#[doc = "Binary Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt1Spec;
impl crate::RegisterSpec for Bcnt1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt1::R`](R) reader structure"]
impl crate::Readable for Bcnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcnt1::W`](W) writer structure"]
impl crate::Writable for Bcnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT1 to value 0"]
impl crate::Resettable for Bcnt1Spec {}

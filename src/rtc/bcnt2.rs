#[doc = "Register `BCNT2` reader"]
pub type R = crate::R<Bcnt2Spec>;
#[doc = "Register `BCNT2` writer"]
pub type W = crate::W<Bcnt2Spec>;
#[doc = "Field `BCNT2` reader - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type Bcnt2R = crate::FieldReader;
#[doc = "Field `BCNT2` writer - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type Bcnt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2(&self) -> Bcnt2R {
        Bcnt2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT2")
            .field("bcnt2", &self.bcnt2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2(&mut self) -> Bcnt2W<Bcnt2Spec> {
        Bcnt2W::new(self, 0)
    }
}
#[doc = "Binary Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt2Spec;
impl crate::RegisterSpec for Bcnt2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt2::R`](R) reader structure"]
impl crate::Readable for Bcnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`bcnt2::W`](W) writer structure"]
impl crate::Writable for Bcnt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT2 to value 0"]
impl crate::Resettable for Bcnt2Spec {}

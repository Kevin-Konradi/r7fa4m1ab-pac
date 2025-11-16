#[doc = "Register `BCNT2AR` reader"]
pub type R = crate::R<Bcnt2arSpec>;
#[doc = "Register `BCNT2AR` writer"]
pub type W = crate::W<Bcnt2arSpec>;
#[doc = "Field `BCNT2AR` reader - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type Bcnt2arR = crate::FieldReader;
#[doc = "Field `BCNT2AR` writer - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type Bcnt2arW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2ar(&self) -> Bcnt2arR {
        Bcnt2arR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT2AR")
            .field("bcnt2ar", &self.bcnt2ar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2ar(&mut self) -> Bcnt2arW<Bcnt2arSpec> {
        Bcnt2arW::new(self, 0)
    }
}
#[doc = "Binary Counter 2 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt2arSpec;
impl crate::RegisterSpec for Bcnt2arSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt2ar::R`](R) reader structure"]
impl crate::Readable for Bcnt2arSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt2ar::W`](W) writer structure"]
impl crate::Writable for Bcnt2arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT2AR to value 0"]
impl crate::Resettable for Bcnt2arSpec {}

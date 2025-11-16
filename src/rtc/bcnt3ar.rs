#[doc = "Register `BCNT3AR` reader"]
pub type R = crate::R<Bcnt3arSpec>;
#[doc = "Register `BCNT3AR` writer"]
pub type W = crate::W<Bcnt3arSpec>;
#[doc = "Field `BCNT3AR` reader - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type Bcnt3arR = crate::FieldReader;
#[doc = "Field `BCNT3AR` writer - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type Bcnt3arW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3ar(&self) -> Bcnt3arR {
        Bcnt3arR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT3AR")
            .field("bcnt3ar", &self.bcnt3ar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3ar(&mut self) -> Bcnt3arW<Bcnt3arSpec> {
        Bcnt3arW::new(self, 0)
    }
}
#[doc = "Binary Counter 3 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt3arSpec;
impl crate::RegisterSpec for Bcnt3arSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3ar::R`](R) reader structure"]
impl crate::Readable for Bcnt3arSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt3ar::W`](W) writer structure"]
impl crate::Writable for Bcnt3arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT3AR to value 0"]
impl crate::Resettable for Bcnt3arSpec {}

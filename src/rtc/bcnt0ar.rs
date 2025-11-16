#[doc = "Register `BCNT0AR` reader"]
pub type R = crate::R<Bcnt0arSpec>;
#[doc = "Register `BCNT0AR` writer"]
pub type W = crate::W<Bcnt0arSpec>;
#[doc = "Field `BCNT0AR` reader - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
pub type Bcnt0arR = crate::FieldReader;
#[doc = "Field `BCNT0AR` writer - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
pub type Bcnt0arW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0ar(&self) -> Bcnt0arR {
        Bcnt0arR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT0AR")
            .field("bcnt0ar", &self.bcnt0ar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0ar(&mut self) -> Bcnt0arW<Bcnt0arSpec> {
        Bcnt0arW::new(self, 0)
    }
}
#[doc = "Binary Counter 0 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt0arSpec;
impl crate::RegisterSpec for Bcnt0arSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0ar::R`](R) reader structure"]
impl crate::Readable for Bcnt0arSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt0ar::W`](W) writer structure"]
impl crate::Writable for Bcnt0arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT0AR to value 0"]
impl crate::Resettable for Bcnt0arSpec {}

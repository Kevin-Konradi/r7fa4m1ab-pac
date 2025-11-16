#[doc = "Register `BCNT1AR` reader"]
pub type R = crate::R<Bcnt1arSpec>;
#[doc = "Register `BCNT1AR` writer"]
pub type W = crate::W<Bcnt1arSpec>;
#[doc = "Field `BCNT1AR` reader - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type Bcnt1arR = crate::FieldReader;
#[doc = "Field `BCNT1AR` writer - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type Bcnt1arW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1ar(&self) -> Bcnt1arR {
        Bcnt1arR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT1AR")
            .field("bcnt1ar", &self.bcnt1ar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1ar(&mut self) -> Bcnt1arW<Bcnt1arSpec> {
        Bcnt1arW::new(self, 0)
    }
}
#[doc = "Binary Counter 1 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt1arSpec;
impl crate::RegisterSpec for Bcnt1arSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt1ar::R`](R) reader structure"]
impl crate::Readable for Bcnt1arSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt1ar::W`](W) writer structure"]
impl crate::Writable for Bcnt1arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT1AR to value 0"]
impl crate::Resettable for Bcnt1arSpec {}

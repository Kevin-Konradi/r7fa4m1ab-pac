#[doc = "Register `AGTCMA` reader"]
pub type R = crate::R<AgtcmaSpec>;
#[doc = "Register `AGTCMA` writer"]
pub type W = crate::W<AgtcmaSpec>;
#[doc = "Field `AGTCMA` reader - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AgtcmaR = crate::FieldReader<u16>;
#[doc = "Field `AGTCMA` writer - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AgtcmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcma(&self) -> AgtcmaR {
        AgtcmaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCMA")
            .field("agtcma", &self.agtcma())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcma(&mut self) -> AgtcmaW<AgtcmaSpec> {
        AgtcmaW::new(self, 0)
    }
}
#[doc = "AGT Compare Match A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtcma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtcmaSpec;
impl crate::RegisterSpec for AgtcmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agtcma::R`](R) reader structure"]
impl crate::Readable for AgtcmaSpec {}
#[doc = "`write(|w| ..)` method takes [`agtcma::W`](W) writer structure"]
impl crate::Writable for AgtcmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTCMA to value 0xffff"]
impl crate::Resettable for AgtcmaSpec {
    const RESET_VALUE: u16 = 0xffff;
}

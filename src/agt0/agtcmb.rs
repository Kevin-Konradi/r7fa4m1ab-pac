#[doc = "Register `AGTCMB` reader"]
pub type R = crate::R<AgtcmbSpec>;
#[doc = "Register `AGTCMB` writer"]
pub type W = crate::W<AgtcmbSpec>;
#[doc = "Field `AGTCMB` reader - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AgtcmbR = crate::FieldReader<u16>;
#[doc = "Field `AGTCMB` writer - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AgtcmbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcmb(&self) -> AgtcmbR {
        AgtcmbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCMB")
            .field("agtcmb", &self.agtcmb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcmb(&mut self) -> AgtcmbW<AgtcmbSpec> {
        AgtcmbW::new(self, 0)
    }
}
#[doc = "AGT Compare Match B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtcmbSpec;
impl crate::RegisterSpec for AgtcmbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agtcmb::R`](R) reader structure"]
impl crate::Readable for AgtcmbSpec {}
#[doc = "`write(|w| ..)` method takes [`agtcmb::W`](W) writer structure"]
impl crate::Writable for AgtcmbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTCMB to value 0xffff"]
impl crate::Resettable for AgtcmbSpec {
    const RESET_VALUE: u16 = 0xffff;
}

#[doc = "Register `AGT` reader"]
pub type R = crate::R<AgtSpec>;
#[doc = "Register `AGT` writer"]
pub type W = crate::W<AgtSpec>;
#[doc = "Field `AGT` reader - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
pub type AgtR = crate::FieldReader<u16>;
#[doc = "Field `AGT` writer - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
pub type AgtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[inline(always)]
    pub fn agt(&self) -> AgtR {
        AgtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGT").field("agt", &self.agt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[inline(always)]
    pub fn agt(&mut self) -> AgtW<AgtSpec> {
        AgtW::new(self, 0)
    }
}
#[doc = "AGT Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtSpec;
impl crate::RegisterSpec for AgtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agt::R`](R) reader structure"]
impl crate::Readable for AgtSpec {}
#[doc = "`write(|w| ..)` method takes [`agt::W`](W) writer structure"]
impl crate::Writable for AgtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGT to value 0xffff"]
impl crate::Resettable for AgtSpec {
    const RESET_VALUE: u16 = 0xffff;
}

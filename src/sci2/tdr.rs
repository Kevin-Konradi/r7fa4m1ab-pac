#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` reader - TDR is an 8-bit register that stores transmit data."]
pub type TdrR = crate::FieldReader;
#[doc = "Field `TDR` writer - TDR is an 8-bit register that stores transmit data."]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDR").field("tdr", &self.tdr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<TdrSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0xff"]
impl crate::Resettable for TdrSpec {
    const RESET_VALUE: u8 = 0xff;
}

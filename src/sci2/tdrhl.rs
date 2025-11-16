#[doc = "Register `TDRHL` reader"]
pub type R = crate::R<TdrhlSpec>;
#[doc = "Register `TDRHL` writer"]
pub type W = crate::W<TdrhlSpec>;
#[doc = "Field `TDRHL` writer - TDRHL is a 16-bit register that stores transmit data."]
pub type TdrhlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDRHL").finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - TDRHL is a 16-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdrhl(&mut self) -> TdrhlW<TdrhlSpec> {
        TdrhlW::new(self, 0)
    }
}
#[doc = "Transmit 9-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdrhl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrhlSpec;
impl crate::RegisterSpec for TdrhlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdrhl::R`](R) reader structure"]
impl crate::Readable for TdrhlSpec {}
#[doc = "`write(|w| ..)` method takes [`tdrhl::W`](W) writer structure"]
impl crate::Writable for TdrhlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDRHL to value 0xffff"]
impl crate::Resettable for TdrhlSpec {
    const RESET_VALUE: u16 = 0xffff;
}

#[doc = "Register `BCNT0AER` reader"]
pub type R = crate::R<Bcnt0aerSpec>;
#[doc = "Register `BCNT0AER` writer"]
pub type W = crate::W<Bcnt0aerSpec>;
#[doc = "Field `ENB` reader - The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
pub type EnbR = crate::FieldReader;
#[doc = "Field `ENB` writer - The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
pub type EnbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT0AER")
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<Bcnt0aerSpec> {
        EnbW::new(self, 0)
    }
}
#[doc = "Binary Counter 0 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt0aerSpec;
impl crate::RegisterSpec for Bcnt0aerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0aer::R`](R) reader structure"]
impl crate::Readable for Bcnt0aerSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt0aer::W`](W) writer structure"]
impl crate::Writable for Bcnt0aerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT0AER to value 0"]
impl crate::Resettable for Bcnt0aerSpec {}

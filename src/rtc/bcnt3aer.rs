#[doc = "Register `BCNT3AER` reader"]
pub type R = crate::R<Bcnt3aerSpec>;
#[doc = "Register `BCNT3AER` writer"]
pub type W = crate::W<Bcnt3aerSpec>;
#[doc = "Field `ENB` reader - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
pub type EnbR = crate::FieldReader;
#[doc = "Field `ENB` writer - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
pub type EnbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT3AER")
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<Bcnt3aerSpec> {
        EnbW::new(self, 0)
    }
}
#[doc = "Binary Counter 3 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt3aerSpec;
impl crate::RegisterSpec for Bcnt3aerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3aer::R`](R) reader structure"]
impl crate::Readable for Bcnt3aerSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt3aer::W`](W) writer structure"]
impl crate::Writable for Bcnt3aerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT3AER to value 0"]
impl crate::Resettable for Bcnt3aerSpec {}

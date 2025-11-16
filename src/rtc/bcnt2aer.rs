#[doc = "Register `BCNT2AER` reader"]
pub type R = crate::R<Bcnt2aerSpec>;
#[doc = "Register `BCNT2AER` writer"]
pub type W = crate::W<Bcnt2aerSpec>;
#[doc = "Field `ENB` reader - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
pub type EnbR = crate::FieldReader;
#[doc = "Field `ENB` writer - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
pub type EnbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCNT2AER")
            .field("enb", &self.enb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<Bcnt2aerSpec> {
        EnbW::new(self, 0)
    }
}
#[doc = "Binary Counter 2 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcnt2aerSpec;
impl crate::RegisterSpec for Bcnt2aerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bcnt2aer::R`](R) reader structure"]
impl crate::Readable for Bcnt2aerSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt2aer::W`](W) writer structure"]
impl crate::Writable for Bcnt2aerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT2AER to value 0"]
impl crate::Resettable for Bcnt2aerSpec {}

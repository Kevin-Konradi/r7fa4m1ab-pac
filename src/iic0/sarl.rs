#[doc = "Register `SARL%s` reader"]
pub type R = crate::R<SarlSpec>;
#[doc = "Register `SARL%s` writer"]
pub type W = crate::W<SarlSpec>;
#[doc = "Field `SVA` reader - A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
pub type SvaR = crate::FieldReader;
#[doc = "Field `SVA` writer - A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
pub type SvaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
    #[inline(always)]
    pub fn sva(&self) -> SvaR {
        SvaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARL").field("sva", &self.sva()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - A slave address is set. 7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
    #[inline(always)]
    pub fn sva(&mut self) -> SvaW<SarlSpec> {
        SvaW::new(self, 0)
    }
}
#[doc = "Slave Address Register L%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sarl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sarl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarlSpec;
impl crate::RegisterSpec for SarlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sarl::R`](R) reader structure"]
impl crate::Readable for SarlSpec {}
#[doc = "`write(|w| ..)` method takes [`sarl::W`](W) writer structure"]
impl crate::Writable for SarlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SARL%s to value 0"]
impl crate::Resettable for SarlSpec {}

#[doc = "Register `AFSR` reader"]
pub type R = crate::R<AfsrSpec>;
#[doc = "Register `AFSR` writer"]
pub type W = crate::W<AfsrSpec>;
#[doc = "Field `AFSR` reader - After the standard ID of a received message is written, the value converted for data table search can be read."]
pub type AfsrR = crate::FieldReader<u16>;
#[doc = "Field `AFSR` writer - After the standard ID of a received message is written, the value converted for data table search can be read."]
pub type AfsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    pub fn afsr(&self) -> AfsrR {
        AfsrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFSR").field("afsr", &self.afsr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    pub fn afsr(&mut self) -> AfsrW<AfsrSpec> {
        AfsrW::new(self, 0)
    }
}
#[doc = "Acceptance Filter Support Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfsrSpec;
impl crate::RegisterSpec for AfsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`afsr::R`](R) reader structure"]
impl crate::Readable for AfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`afsr::W`](W) writer structure"]
impl crate::Writable for AfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AfsrSpec {}

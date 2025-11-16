#[doc = "Register `CTSUSO0` reader"]
pub type R = crate::R<Ctsuso0Spec>;
#[doc = "Register `CTSUSO0` writer"]
pub type W = crate::W<Ctsuso0Spec>;
#[doc = "Field `CTSUSO` reader - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
pub type CtsusoR = crate::FieldReader<u16>;
#[doc = "Field `CTSUSO` writer - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
pub type CtsusoW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CTSUSNUM` reader - CTSU Measurement Count Setting"]
pub type CtsusnumR = crate::FieldReader;
#[doc = "Field `CTSUSNUM` writer - CTSU Measurement Count Setting"]
pub type CtsusnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[inline(always)]
    pub fn ctsuso(&self) -> CtsusoR {
        CtsusoR::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 10:15 - CTSU Measurement Count Setting"]
    #[inline(always)]
    pub fn ctsusnum(&self) -> CtsusnumR {
        CtsusnumR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSO0")
            .field("ctsusnum", &self.ctsusnum())
            .field("ctsuso", &self.ctsuso())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[inline(always)]
    pub fn ctsuso(&mut self) -> CtsusoW<Ctsuso0Spec> {
        CtsusoW::new(self, 0)
    }
    #[doc = "Bits 10:15 - CTSU Measurement Count Setting"]
    #[inline(always)]
    pub fn ctsusnum(&mut self) -> CtsusnumW<Ctsuso0Spec> {
        CtsusnumW::new(self, 10)
    }
}
#[doc = "CTSU Sensor Offset Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuso0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuso0Spec;
impl crate::RegisterSpec for Ctsuso0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsuso0::R`](R) reader structure"]
impl crate::Readable for Ctsuso0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuso0::W`](W) writer structure"]
impl crate::Writable for Ctsuso0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUSO0 to value 0"]
impl crate::Resettable for Ctsuso0Spec {}

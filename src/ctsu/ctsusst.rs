#[doc = "Register `CTSUSST` reader"]
pub type R = crate::R<CtsusstSpec>;
#[doc = "Register `CTSUSST` writer"]
pub type W = crate::W<CtsusstSpec>;
#[doc = "Field `CTSUSST` reader - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
pub type CtsusstR = crate::FieldReader;
#[doc = "Field `CTSUSST` writer - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
pub type CtsusstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    pub fn ctsusst(&self) -> CtsusstR {
        CtsusstR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSST")
            .field("ctsusst", &self.ctsusst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    pub fn ctsusst(&mut self) -> CtsusstW<CtsusstSpec> {
        CtsusstW::new(self, 0)
    }
}
#[doc = "CTSU Sensor Stabilization Wait Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsusst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsusstSpec;
impl crate::RegisterSpec for CtsusstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsusst::R`](R) reader structure"]
impl crate::Readable for CtsusstSpec {}
#[doc = "`write(|w| ..)` method takes [`ctsusst::W`](W) writer structure"]
impl crate::Writable for CtsusstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUSST to value 0"]
impl crate::Resettable for CtsusstSpec {}

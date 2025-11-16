#[doc = "Register `ADSTRGR` reader"]
pub type R = crate::R<AdstrgrSpec>;
#[doc = "Register `ADSTRGR` writer"]
pub type W = crate::W<AdstrgrSpec>;
#[doc = "Field `TRSB` reader - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TrsbR = crate::FieldReader;
#[doc = "Field `TRSB` writer - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TrsbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRSA` reader - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TrsaR = crate::FieldReader;
#[doc = "Field `TRSA` writer - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TrsaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub fn trsb(&self) -> TrsbR {
        TrsbR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub fn trsa(&self) -> TrsaR {
        TrsaR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSTRGR")
            .field("trsa", &self.trsa())
            .field("trsb", &self.trsb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub fn trsb(&mut self) -> TrsbW<AdstrgrSpec> {
        TrsbW::new(self, 0)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub fn trsa(&mut self) -> TrsaW<AdstrgrSpec> {
        TrsaW::new(self, 8)
    }
}
#[doc = "A/D Conversion Start Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adstrgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdstrgrSpec;
impl crate::RegisterSpec for AdstrgrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adstrgr::R`](R) reader structure"]
impl crate::Readable for AdstrgrSpec {}
#[doc = "`write(|w| ..)` method takes [`adstrgr::W`](W) writer structure"]
impl crate::Writable for AdstrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADSTRGR to value 0"]
impl crate::Resettable for AdstrgrSpec {}

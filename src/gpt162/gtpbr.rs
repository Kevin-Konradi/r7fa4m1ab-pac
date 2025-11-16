#[doc = "Register `GTPBR` reader"]
pub type R = crate::R<GtpbrSpec>;
#[doc = "Register `GTPBR` writer"]
pub type W = crate::W<GtpbrSpec>;
#[doc = "Field `GTPBR` reader - Cycle Setting Buffer Register"]
pub type GtpbrR = crate::FieldReader<u32>;
#[doc = "Field `GTPBR` writer - Cycle Setting Buffer Register"]
pub type GtpbrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle Setting Buffer Register"]
    #[inline(always)]
    pub fn gtpbr(&self) -> GtpbrR {
        GtpbrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPBR")
            .field("gtpbr", &self.gtpbr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cycle Setting Buffer Register"]
    #[inline(always)]
    pub fn gtpbr(&mut self) -> GtpbrW<GtpbrSpec> {
        GtpbrW::new(self, 0)
    }
}
#[doc = "General PWM Timer Cycle Setting Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtpbrSpec;
impl crate::RegisterSpec for GtpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpbr::R`](R) reader structure"]
impl crate::Readable for GtpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtpbr::W`](W) writer structure"]
impl crate::Writable for GtpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTPBR to value 0xffff"]
impl crate::Resettable for GtpbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}

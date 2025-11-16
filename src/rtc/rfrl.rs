#[doc = "Register `RFRL` reader"]
pub type R = crate::R<RfrlSpec>;
#[doc = "Register `RFRL` writer"]
pub type W = crate::W<RfrlSpec>;
#[doc = "Field `RFC` reader - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RfcR = crate::FieldReader<u16>;
#[doc = "Field `RFC` writer - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RfcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFRL").field("rfc", &self.rfc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc(&mut self) -> RfcW<RfrlSpec> {
        RfcW::new(self, 0)
    }
}
#[doc = "Frequency Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfrlSpec;
impl crate::RegisterSpec for RfrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rfrl::R`](R) reader structure"]
impl crate::Readable for RfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rfrl::W`](W) writer structure"]
impl crate::Writable for RfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFRL to value 0"]
impl crate::Resettable for RfrlSpec {}

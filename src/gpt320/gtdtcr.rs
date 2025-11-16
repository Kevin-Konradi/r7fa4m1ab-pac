#[doc = "Register `GTDTCR` reader"]
pub type R = crate::R<GtdtcrSpec>;
#[doc = "Register `GTDTCR` writer"]
pub type W = crate::W<GtdtcrSpec>;
#[doc = "Negative-Phase Waveform Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    #[doc = "0: GTCCRB is set without using GTDVU and GTDVD."]
    _0 = 0,
    #[doc = "1: GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    _1 = 1,
}
impl From<Tde> for bool {
    #[inline(always)]
    fn from(variant: Tde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Negative-Phase Waveform Setting"]
pub type TdeR = crate::BitReader<Tde>;
impl TdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tde {
        match self.bits {
            false => Tde::_0,
            true => Tde::_1,
        }
    }
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tde::_0
    }
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tde::_1
    }
}
#[doc = "Field `TDE` writer - Negative-Phase Waveform Setting"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG, Tde>;
impl<'a, REG> TdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_0)
    }
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTDTCR").field("tde", &self.tde()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<GtdtcrSpec> {
        TdeW::new(self, 0)
    }
}
#[doc = "General PWM Timer Dead Time Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtdtcrSpec;
impl crate::RegisterSpec for GtdtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtdtcr::R`](R) reader structure"]
impl crate::Readable for GtdtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtdtcr::W`](W) writer structure"]
impl crate::Writable for GtdtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTDTCR to value 0"]
impl crate::Resettable for GtdtcrSpec {}

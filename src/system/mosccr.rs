#[doc = "Register `MOSCCR` reader"]
pub type R = crate::R<MosccrSpec>;
#[doc = "Register `MOSCCR` writer"]
pub type W = crate::W<MosccrSpec>;
#[doc = "Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mostp {
    #[doc = "0: Main clock oscillator is operating."]
    _0 = 0,
    #[doc = "1: Main clock oscillator is stopped."]
    _1 = 1,
}
impl From<Mostp> for bool {
    #[inline(always)]
    fn from(variant: Mostp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOSTP` reader - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
pub type MostpR = crate::BitReader<Mostp>;
impl MostpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mostp {
        match self.bits {
            false => Mostp::_0,
            true => Mostp::_1,
        }
    }
    #[doc = "Main clock oscillator is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mostp::_0
    }
    #[doc = "Main clock oscillator is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mostp::_1
    }
}
#[doc = "Field `MOSTP` writer - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
pub type MostpW<'a, REG> = crate::BitWriter<'a, REG, Mostp>;
impl<'a, REG> MostpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main clock oscillator is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mostp::_0)
    }
    #[doc = "Main clock oscillator is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mostp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    pub fn mostp(&self) -> MostpR {
        MostpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOSCCR")
            .field("mostp", &self.mostp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    pub fn mostp(&mut self) -> MostpW<MosccrSpec> {
        MostpW::new(self, 0)
    }
}
#[doc = "Main Clock Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MosccrSpec;
impl crate::RegisterSpec for MosccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mosccr::R`](R) reader structure"]
impl crate::Readable for MosccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mosccr::W`](W) writer structure"]
impl crate::Writable for MosccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOSCCR to value 0x01"]
impl crate::Resettable for MosccrSpec {
    const RESET_VALUE: u8 = 0x01;
}

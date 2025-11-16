#[doc = "Register `OSTDCR` reader"]
pub type R = crate::R<OstdcrSpec>;
#[doc = "Register `OSTDCR` writer"]
pub type W = crate::W<OstdcrSpec>;
#[doc = "Oscillation Stop Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostdie {
    #[doc = "0: The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    _0 = 0,
    #[doc = "1: The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    _1 = 1,
}
impl From<Ostdie> for bool {
    #[inline(always)]
    fn from(variant: Ostdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTDIE` reader - Oscillation Stop Detection Interrupt Enable"]
pub type OstdieR = crate::BitReader<Ostdie>;
impl OstdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostdie {
        match self.bits {
            false => Ostdie::_0,
            true => Ostdie::_1,
        }
    }
    #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostdie::_0
    }
    #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostdie::_1
    }
}
#[doc = "Field `OSTDIE` writer - Oscillation Stop Detection Interrupt Enable"]
pub type OstdieW<'a, REG> = crate::BitWriter<'a, REG, Ostdie>;
impl<'a, REG> OstdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostdie::_0)
    }
    #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostdie::_1)
    }
}
#[doc = "Oscillation Stop Detection Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostde {
    #[doc = "0: Oscillation stop detection function is disabled."]
    _0 = 0,
    #[doc = "1: Oscillation stop detection function is enabled."]
    _1 = 1,
}
impl From<Ostde> for bool {
    #[inline(always)]
    fn from(variant: Ostde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTDE` reader - Oscillation Stop Detection Function Enable"]
pub type OstdeR = crate::BitReader<Ostde>;
impl OstdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostde {
        match self.bits {
            false => Ostde::_0,
            true => Ostde::_1,
        }
    }
    #[doc = "Oscillation stop detection function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostde::_0
    }
    #[doc = "Oscillation stop detection function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostde::_1
    }
}
#[doc = "Field `OSTDE` writer - Oscillation Stop Detection Function Enable"]
pub type OstdeW<'a, REG> = crate::BitWriter<'a, REG, Ostde>;
impl<'a, REG> OstdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillation stop detection function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostde::_0)
    }
    #[doc = "Oscillation stop detection function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostde::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(&self) -> OstdieR {
        OstdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(&self) -> OstdeR {
        OstdeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSTDCR")
            .field("ostde", &self.ostde())
            .field("ostdie", &self.ostdie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(&mut self) -> OstdieW<OstdcrSpec> {
        OstdieW::new(self, 0)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(&mut self) -> OstdeW<OstdcrSpec> {
        OstdeW::new(self, 7)
    }
}
#[doc = "Oscillation Stop Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ostdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OstdcrSpec;
impl crate::RegisterSpec for OstdcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ostdcr::R`](R) reader structure"]
impl crate::Readable for OstdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ostdcr::W`](W) writer structure"]
impl crate::Writable for OstdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSTDCR to value 0"]
impl crate::Resettable for OstdcrSpec {}

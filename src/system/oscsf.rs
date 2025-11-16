#[doc = "Register `OSCSF` reader"]
pub type R = crate::R<OscsfSpec>;
#[doc = "HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hocosf {
    #[doc = "0: The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<Hocosf> for bool {
    #[inline(always)]
    fn from(variant: Hocosf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
pub type HocosfR = crate::BitReader<Hocosf>;
impl HocosfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hocosf {
        match self.bits {
            false => Hocosf::_0,
            true => Hocosf::_1,
        }
    }
    #[doc = "The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hocosf::_0
    }
    #[doc = "Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hocosf::_1
    }
}
#[doc = "Main Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moscsf {
    #[doc = "0: MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the main clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<Moscsf> for bool {
    #[inline(always)]
    fn from(variant: Moscsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOSCSF` reader - Main Clock Oscillation Stabilization Flag"]
pub type MoscsfR = crate::BitReader<Moscsf>;
impl MoscsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moscsf {
        match self.bits {
            false => Moscsf::_0,
            true => Moscsf::_1,
        }
    }
    #[doc = "MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moscsf::_0
    }
    #[doc = "Oscillation of the main clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moscsf::_1
    }
}
#[doc = "PLL Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsf {
    #[doc = "0: The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<Pllsf> for bool {
    #[inline(always)]
    fn from(variant: Pllsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSF` reader - PLL Clock Oscillation Stabilization Flag"]
pub type PllsfR = crate::BitReader<Pllsf>;
impl PllsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsf {
        match self.bits {
            false => Pllsf::_0,
            true => Pllsf::_1,
        }
    }
    #[doc = "The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllsf::_0
    }
    #[doc = "Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllsf::_1
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[inline(always)]
    pub fn hocosf(&self) -> HocosfR {
        HocosfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(&self) -> MoscsfR {
        MoscsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(&self) -> PllsfR {
        PllsfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSCSF")
            .field("pllsf", &self.pllsf())
            .field("moscsf", &self.moscsf())
            .field("hocosf", &self.hocosf())
            .finish()
    }
}
#[doc = "Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oscsf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscsfSpec;
impl crate::RegisterSpec for OscsfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oscsf::R`](R) reader structure"]
impl crate::Readable for OscsfSpec {}
#[doc = "`reset()` method sets OSCSF to value 0"]
impl crate::Resettable for OscsfSpec {}

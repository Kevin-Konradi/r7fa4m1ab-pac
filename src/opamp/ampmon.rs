#[doc = "Register `AMPMON` reader"]
pub type R = crate::R<AmpmonSpec>;
#[doc = "Operational amplifier status(UNIT0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampmon0 {
    #[doc = "0: Operational amplifier 0 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 0 is operating."]
    _1 = 1,
}
impl From<Ampmon0> for bool {
    #[inline(always)]
    fn from(variant: Ampmon0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPMON0` reader - Operational amplifier status(UNIT0)"]
pub type Ampmon0R = crate::BitReader<Ampmon0>;
impl Ampmon0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampmon0 {
        match self.bits {
            false => Ampmon0::_0,
            true => Ampmon0::_1,
        }
    }
    #[doc = "Operational amplifier 0 is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampmon0::_0
    }
    #[doc = "Operational amplifier 0 is operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampmon0::_1
    }
}
#[doc = "Operational amplifier status(UNIT1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampmon1 {
    #[doc = "0: Operational amplifier 1 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 1 is operating."]
    _1 = 1,
}
impl From<Ampmon1> for bool {
    #[inline(always)]
    fn from(variant: Ampmon1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPMON1` reader - Operational amplifier status(UNIT1)"]
pub type Ampmon1R = crate::BitReader<Ampmon1>;
impl Ampmon1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampmon1 {
        match self.bits {
            false => Ampmon1::_0,
            true => Ampmon1::_1,
        }
    }
    #[doc = "Operational amplifier 1 is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampmon1::_0
    }
    #[doc = "Operational amplifier 1 is operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampmon1::_1
    }
}
#[doc = "Operational amplifier status(UNIT2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampmon2 {
    #[doc = "0: Operational amplifier 2 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 2 is operating."]
    _1 = 1,
}
impl From<Ampmon2> for bool {
    #[inline(always)]
    fn from(variant: Ampmon2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPMON2` reader - Operational amplifier status(UNIT2)"]
pub type Ampmon2R = crate::BitReader<Ampmon2>;
impl Ampmon2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampmon2 {
        match self.bits {
            false => Ampmon2::_0,
            true => Ampmon2::_1,
        }
    }
    #[doc = "Operational amplifier 2 is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampmon2::_0
    }
    #[doc = "Operational amplifier 2 is operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampmon2::_1
    }
}
#[doc = "Operational amplifier status(UNIT3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampmon3 {
    #[doc = "0: Operational amplifier 3 is stopped."]
    _0 = 0,
    #[doc = "1: Operational amplifier 3 is operating."]
    _1 = 1,
}
impl From<Ampmon3> for bool {
    #[inline(always)]
    fn from(variant: Ampmon3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPMON3` reader - Operational amplifier status(UNIT3)"]
pub type Ampmon3R = crate::BitReader<Ampmon3>;
impl Ampmon3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampmon3 {
        match self.bits {
            false => Ampmon3::_0,
            true => Ampmon3::_1,
        }
    }
    #[doc = "Operational amplifier 3 is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampmon3::_0
    }
    #[doc = "Operational amplifier 3 is operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampmon3::_1
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier status(UNIT0)"]
    #[inline(always)]
    pub fn ampmon0(&self) -> Ampmon0R {
        Ampmon0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier status(UNIT1)"]
    #[inline(always)]
    pub fn ampmon1(&self) -> Ampmon1R {
        Ampmon1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier status(UNIT2)"]
    #[inline(always)]
    pub fn ampmon2(&self) -> Ampmon2R {
        Ampmon2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier status(UNIT3)"]
    #[inline(always)]
    pub fn ampmon3(&self) -> Ampmon3R {
        Ampmon3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPMON")
            .field("ampmon3", &self.ampmon3())
            .field("ampmon2", &self.ampmon2())
            .field("ampmon1", &self.ampmon1())
            .field("ampmon0", &self.ampmon0())
            .finish()
    }
}
#[doc = "Operational amplifier monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpmonSpec;
impl crate::RegisterSpec for AmpmonSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ampmon::R`](R) reader structure"]
impl crate::Readable for AmpmonSpec {}
#[doc = "`reset()` method sets AMPMON to value 0"]
impl crate::Resettable for AmpmonSpec {}

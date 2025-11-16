#[doc = "Register `ADWINMON` reader"]
pub type R = crate::R<AdwinmonSpec>;
#[doc = "Combination result monitor This bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moncomb {
    #[doc = "0: Window A / window B composite conditions are not met."]
    _0 = 0,
    #[doc = "1: Window A / window B composite conditions are met."]
    _1 = 1,
}
impl From<Moncomb> for bool {
    #[inline(always)]
    fn from(variant: Moncomb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONCOMB` reader - Combination result monitor This bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled."]
pub type MoncombR = crate::BitReader<Moncomb>;
impl MoncombR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moncomb {
        match self.bits {
            false => Moncomb::_0,
            true => Moncomb::_1,
        }
    }
    #[doc = "Window A / window B composite conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moncomb::_0
    }
    #[doc = "Window A / window B composite conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moncomb::_1
    }
}
#[doc = "Comparison Result Monitor A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moncmpa {
    #[doc = "0: Window A comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Window A comparison conditions are met."]
    _1 = 1,
}
impl From<Moncmpa> for bool {
    #[inline(always)]
    fn from(variant: Moncmpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONCMPA` reader - Comparison Result Monitor A"]
pub type MoncmpaR = crate::BitReader<Moncmpa>;
impl MoncmpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moncmpa {
        match self.bits {
            false => Moncmpa::_0,
            true => Moncmpa::_1,
        }
    }
    #[doc = "Window A comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moncmpa::_0
    }
    #[doc = "Window A comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moncmpa::_1
    }
}
#[doc = "Comparison Result Monitor B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moncmpb {
    #[doc = "0: Window B comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Window B comparison conditions are met."]
    _1 = 1,
}
impl From<Moncmpb> for bool {
    #[inline(always)]
    fn from(variant: Moncmpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONCMPB` reader - Comparison Result Monitor B"]
pub type MoncmpbR = crate::BitReader<Moncmpb>;
impl MoncmpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moncmpb {
        match self.bits {
            false => Moncmpb::_0,
            true => Moncmpb::_1,
        }
    }
    #[doc = "Window B comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moncmpb::_0
    }
    #[doc = "Window B comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moncmpb::_1
    }
}
impl R {
    #[doc = "Bit 0 - Combination result monitor This bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled."]
    #[inline(always)]
    pub fn moncomb(&self) -> MoncombR {
        MoncombR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison Result Monitor A"]
    #[inline(always)]
    pub fn moncmpa(&self) -> MoncmpaR {
        MoncmpaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison Result Monitor B"]
    #[inline(always)]
    pub fn moncmpb(&self) -> MoncmpbR {
        MoncmpbR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADWINMON")
            .field("moncmpb", &self.moncmpb())
            .field("moncmpa", &self.moncmpa())
            .field("moncomb", &self.moncomb())
            .finish()
    }
}
#[doc = "A/D Compare Function Window A/B Status Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdwinmonSpec;
impl crate::RegisterSpec for AdwinmonSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adwinmon::R`](R) reader structure"]
impl crate::Readable for AdwinmonSpec {}
#[doc = "`reset()` method sets ADWINMON to value 0"]
impl crate::Resettable for AdwinmonSpec {}

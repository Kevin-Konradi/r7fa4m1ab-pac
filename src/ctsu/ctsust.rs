#[doc = "Register `CTSUST` reader"]
pub type R = crate::R<CtsustSpec>;
#[doc = "Register `CTSUST` writer"]
pub type W = crate::W<CtsustSpec>;
#[doc = "CTSU Measurement Status Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsustc {
    #[doc = "0: Status 0"]
    _000 = 0,
    #[doc = "1: Status 1"]
    _001 = 1,
    #[doc = "2: Status 2"]
    _010 = 2,
    #[doc = "3: Status 3"]
    _011 = 3,
    #[doc = "4: Status 4"]
    _100 = 4,
    #[doc = "5: Status 5"]
    _101 = 5,
}
impl From<Ctsustc> for u8 {
    #[inline(always)]
    fn from(variant: Ctsustc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsustc {
    type Ux = u8;
}
impl crate::IsEnum for Ctsustc {}
#[doc = "Field `CTSUSTC` reader - CTSU Measurement Status Counter"]
pub type CtsustcR = crate::FieldReader<Ctsustc>;
impl CtsustcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsustc> {
        match self.bits {
            0 => Some(Ctsustc::_000),
            1 => Some(Ctsustc::_001),
            2 => Some(Ctsustc::_010),
            3 => Some(Ctsustc::_011),
            4 => Some(Ctsustc::_100),
            5 => Some(Ctsustc::_101),
            _ => None,
        }
    }
    #[doc = "Status 0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ctsustc::_000
    }
    #[doc = "Status 1"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ctsustc::_001
    }
    #[doc = "Status 2"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ctsustc::_010
    }
    #[doc = "Status 3"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ctsustc::_011
    }
    #[doc = "Status 4"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ctsustc::_100
    }
    #[doc = "Status 5"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ctsustc::_101
    }
}
#[doc = "CTSU Data Transfer Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsudtsr {
    #[doc = "0: Measurement result has been read"]
    _0 = 0,
    #[doc = "1: Measurement result has not been read"]
    _1 = 1,
}
impl From<Ctsudtsr> for bool {
    #[inline(always)]
    fn from(variant: Ctsudtsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUDTSR` reader - CTSU Data Transfer Status Flag"]
pub type CtsudtsrR = crate::BitReader<Ctsudtsr>;
impl CtsudtsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsudtsr {
        match self.bits {
            false => Ctsudtsr::_0,
            true => Ctsudtsr::_1,
        }
    }
    #[doc = "Measurement result has been read"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsudtsr::_0
    }
    #[doc = "Measurement result has not been read"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsudtsr::_1
    }
}
#[doc = "CTSU Sensor Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsusovf {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: An overflow"]
    _1 = 1,
}
impl From<Ctsusovf> for bool {
    #[inline(always)]
    fn from(variant: Ctsusovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUSOVF` reader - CTSU Sensor Counter Overflow Flag"]
pub type CtsusovfR = crate::BitReader<Ctsusovf>;
impl CtsusovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsusovf {
        match self.bits {
            false => Ctsusovf::_0,
            true => Ctsusovf::_1,
        }
    }
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsusovf::_0
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsusovf::_1
    }
}
#[doc = "Field `CTSUSOVF` writer - CTSU Sensor Counter Overflow Flag"]
pub type CtsusovfW<'a, REG> = crate::BitWriter<'a, REG, Ctsusovf>;
impl<'a, REG> CtsusovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusovf::_0)
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusovf::_1)
    }
}
#[doc = "CTSU Reference Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsurovf {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: An overflow"]
    _1 = 1,
}
impl From<Ctsurovf> for bool {
    #[inline(always)]
    fn from(variant: Ctsurovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUROVF` reader - CTSU Reference Counter Overflow Flag"]
pub type CtsurovfR = crate::BitReader<Ctsurovf>;
impl CtsurovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsurovf {
        match self.bits {
            false => Ctsurovf::_0,
            true => Ctsurovf::_1,
        }
    }
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsurovf::_0
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsurovf::_1
    }
}
#[doc = "Field `CTSUROVF` writer - CTSU Reference Counter Overflow Flag"]
pub type CtsurovfW<'a, REG> = crate::BitWriter<'a, REG, Ctsurovf>;
impl<'a, REG> CtsurovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsurovf::_0)
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsurovf::_1)
    }
}
#[doc = "CTSU Mutual Capacitance Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsups {
    #[doc = "0: First measurement"]
    _0 = 0,
    #[doc = "1: Second measurement"]
    _1 = 1,
}
impl From<Ctsups> for bool {
    #[inline(always)]
    fn from(variant: Ctsups) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUPS` reader - CTSU Mutual Capacitance Status Flag"]
pub type CtsupsR = crate::BitReader<Ctsups>;
impl CtsupsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsups {
        match self.bits {
            false => Ctsups::_0,
            true => Ctsups::_1,
        }
    }
    #[doc = "First measurement"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsups::_0
    }
    #[doc = "Second measurement"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsups::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - CTSU Measurement Status Counter"]
    #[inline(always)]
    pub fn ctsustc(&self) -> CtsustcR {
        CtsustcR::new(self.bits & 7)
    }
    #[doc = "Bit 4 - CTSU Data Transfer Status Flag"]
    #[inline(always)]
    pub fn ctsudtsr(&self) -> CtsudtsrR {
        CtsudtsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsusovf(&self) -> CtsusovfR {
        CtsusovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTSU Reference Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsurovf(&self) -> CtsurovfR {
        CtsurovfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTSU Mutual Capacitance Status Flag"]
    #[inline(always)]
    pub fn ctsups(&self) -> CtsupsR {
        CtsupsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUST")
            .field("ctsups", &self.ctsups())
            .field("ctsurovf", &self.ctsurovf())
            .field("ctsusovf", &self.ctsusovf())
            .field("ctsudtsr", &self.ctsudtsr())
            .field("ctsustc", &self.ctsustc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsusovf(&mut self) -> CtsusovfW<CtsustSpec> {
        CtsusovfW::new(self, 5)
    }
    #[doc = "Bit 6 - CTSU Reference Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsurovf(&mut self) -> CtsurovfW<CtsustSpec> {
        CtsurovfW::new(self, 6)
    }
}
#[doc = "CTSU Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsust::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsustSpec;
impl crate::RegisterSpec for CtsustSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsust::R`](R) reader structure"]
impl crate::Readable for CtsustSpec {}
#[doc = "`write(|w| ..)` method takes [`ctsust::W`](W) writer structure"]
impl crate::Writable for CtsustSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUST to value 0"]
impl crate::Resettable for CtsustSpec {}

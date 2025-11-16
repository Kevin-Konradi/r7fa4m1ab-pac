#[doc = "Register `GTSTP` reader"]
pub type R = crate::R<GtstpSpec>;
#[doc = "Register `GTSTP` writer"]
pub type W = crate::W<GtstpSpec>;
#[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop0 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop0> for bool {
    #[inline(always)]
    fn from(variant: Cstop0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP0` reader - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop0R = crate::BitReader<Cstop0>;
impl Cstop0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop0 {
        match self.bits {
            false => Cstop0::_0,
            true => Cstop0::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop0::_0
    }
    #[doc = "GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop0::_1
    }
}
#[doc = "Field `CSTOP0` writer - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop0W<'a, REG> = crate::BitWriter<'a, REG, Cstop0>;
impl<'a, REG> Cstop0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop0::_0)
    }
    #[doc = "GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop0::_1)
    }
}
#[doc = "Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop1 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop1> for bool {
    #[inline(always)]
    fn from(variant: Cstop1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP1` reader - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop1R = crate::BitReader<Cstop1>;
impl Cstop1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop1 {
        match self.bits {
            false => Cstop1::_0,
            true => Cstop1::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop1::_0
    }
    #[doc = "GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop1::_1
    }
}
#[doc = "Field `CSTOP1` writer - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop1W<'a, REG> = crate::BitWriter<'a, REG, Cstop1>;
impl<'a, REG> Cstop1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop1::_0)
    }
    #[doc = "GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop1::_1)
    }
}
#[doc = "Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop2 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop2> for bool {
    #[inline(always)]
    fn from(variant: Cstop2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP2` reader - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop2R = crate::BitReader<Cstop2>;
impl Cstop2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop2 {
        match self.bits {
            false => Cstop2::_0,
            true => Cstop2::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop2::_0
    }
    #[doc = "GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop2::_1
    }
}
#[doc = "Field `CSTOP2` writer - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop2W<'a, REG> = crate::BitWriter<'a, REG, Cstop2>;
impl<'a, REG> Cstop2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop2::_0)
    }
    #[doc = "GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop2::_1)
    }
}
#[doc = "Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop3 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop3> for bool {
    #[inline(always)]
    fn from(variant: Cstop3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP3` reader - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop3R = crate::BitReader<Cstop3>;
impl Cstop3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop3 {
        match self.bits {
            false => Cstop3::_0,
            true => Cstop3::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop3::_0
    }
    #[doc = "GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop3::_1
    }
}
#[doc = "Field `CSTOP3` writer - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop3W<'a, REG> = crate::BitWriter<'a, REG, Cstop3>;
impl<'a, REG> Cstop3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop3::_0)
    }
    #[doc = "GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop3::_1)
    }
}
#[doc = "Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop4 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop4> for bool {
    #[inline(always)]
    fn from(variant: Cstop4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP4` reader - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop4R = crate::BitReader<Cstop4>;
impl Cstop4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop4 {
        match self.bits {
            false => Cstop4::_0,
            true => Cstop4::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop4::_0
    }
    #[doc = "GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop4::_1
    }
}
#[doc = "Field `CSTOP4` writer - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop4W<'a, REG> = crate::BitWriter<'a, REG, Cstop4>;
impl<'a, REG> Cstop4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop4::_0)
    }
    #[doc = "GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop4::_1)
    }
}
#[doc = "Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop5 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop5> for bool {
    #[inline(always)]
    fn from(variant: Cstop5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP5` reader - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop5R = crate::BitReader<Cstop5>;
impl Cstop5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop5 {
        match self.bits {
            false => Cstop5::_0,
            true => Cstop5::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop5::_0
    }
    #[doc = "GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop5::_1
    }
}
#[doc = "Field `CSTOP5` writer - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop5W<'a, REG> = crate::BitWriter<'a, REG, Cstop5>;
impl<'a, REG> Cstop5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop5::_0)
    }
    #[doc = "GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop5::_1)
    }
}
#[doc = "Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop6 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop6> for bool {
    #[inline(always)]
    fn from(variant: Cstop6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP6` reader - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop6R = crate::BitReader<Cstop6>;
impl Cstop6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop6 {
        match self.bits {
            false => Cstop6::_0,
            true => Cstop6::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop6::_0
    }
    #[doc = "GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop6::_1
    }
}
#[doc = "Field `CSTOP6` writer - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop6W<'a, REG> = crate::BitWriter<'a, REG, Cstop6>;
impl<'a, REG> Cstop6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop6::_0)
    }
    #[doc = "GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop6::_1)
    }
}
#[doc = "Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop7 {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<Cstop7> for bool {
    #[inline(always)]
    fn from(variant: Cstop7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTOP7` reader - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop7R = crate::BitReader<Cstop7>;
impl Cstop7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstop7 {
        match self.bits {
            false => Cstop7::_0,
            true => Cstop7::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop7::_0
    }
    #[doc = "GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop7::_1
    }
}
#[doc = "Field `CSTOP7` writer - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type Cstop7W<'a, REG> = crate::BitWriter<'a, REG, Cstop7>;
impl<'a, REG> Cstop7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop7::_0)
    }
    #[doc = "GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop0(&self) -> Cstop0R {
        Cstop0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop1(&self) -> Cstop1R {
        Cstop1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop2(&self) -> Cstop2R {
        Cstop2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop3(&self) -> Cstop3R {
        Cstop3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop4(&self) -> Cstop4R {
        Cstop4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop5(&self) -> Cstop5R {
        Cstop5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop6(&self) -> Cstop6R {
        Cstop6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop7(&self) -> Cstop7R {
        Cstop7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSTP")
            .field("cstop7", &self.cstop7())
            .field("cstop6", &self.cstop6())
            .field("cstop5", &self.cstop5())
            .field("cstop4", &self.cstop4())
            .field("cstop3", &self.cstop3())
            .field("cstop2", &self.cstop2())
            .field("cstop1", &self.cstop1())
            .field("cstop0", &self.cstop0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop0(&mut self) -> Cstop0W<GtstpSpec> {
        Cstop0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop1(&mut self) -> Cstop1W<GtstpSpec> {
        Cstop1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop2(&mut self) -> Cstop2W<GtstpSpec> {
        Cstop2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop3(&mut self) -> Cstop3W<GtstpSpec> {
        Cstop3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop4(&mut self) -> Cstop4W<GtstpSpec> {
        Cstop4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop5(&mut self) -> Cstop5W<GtstpSpec> {
        Cstop5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop6(&mut self) -> Cstop6W<GtstpSpec> {
        Cstop6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop7(&mut self) -> Cstop7W<GtstpSpec> {
        Cstop7W::new(self, 7)
    }
}
#[doc = "General PWM Timer Software Stop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtstp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtstpSpec;
impl crate::RegisterSpec for GtstpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtstp::R`](R) reader structure"]
impl crate::Readable for GtstpSpec {}
#[doc = "`write(|w| ..)` method takes [`gtstp::W`](W) writer structure"]
impl crate::Writable for GtstpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTSTP to value 0xffff_ffff"]
impl crate::Resettable for GtstpSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

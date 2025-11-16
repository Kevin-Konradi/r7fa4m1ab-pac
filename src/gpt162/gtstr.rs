#[doc = "Register `GTSTR` reader"]
pub type R = crate::R<GtstrSpec>;
#[doc = "Register `GTSTR` writer"]
pub type W = crate::W<GtstrSpec>;
#[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt0 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt0> for bool {
    #[inline(always)]
    fn from(variant: Cstrt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT0` reader - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt0R = crate::BitReader<Cstrt0>;
impl Cstrt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt0 {
        match self.bits {
            false => Cstrt0::_0,
            true => Cstrt0::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt0::_0
    }
    #[doc = "GPT320.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt0::_1
    }
}
#[doc = "Field `CSTRT0` writer - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt0W<'a, REG> = crate::BitWriter<'a, REG, Cstrt0>;
impl<'a, REG> Cstrt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt0::_0)
    }
    #[doc = "GPT320.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt0::_1)
    }
}
#[doc = "Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt1 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt1> for bool {
    #[inline(always)]
    fn from(variant: Cstrt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT1` reader - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt1R = crate::BitReader<Cstrt1>;
impl Cstrt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt1 {
        match self.bits {
            false => Cstrt1::_0,
            true => Cstrt1::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt1::_0
    }
    #[doc = "GPT321.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt1::_1
    }
}
#[doc = "Field `CSTRT1` writer - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt1W<'a, REG> = crate::BitWriter<'a, REG, Cstrt1>;
impl<'a, REG> Cstrt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt1::_0)
    }
    #[doc = "GPT321.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt1::_1)
    }
}
#[doc = "Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt2 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt2> for bool {
    #[inline(always)]
    fn from(variant: Cstrt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT2` reader - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt2R = crate::BitReader<Cstrt2>;
impl Cstrt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt2 {
        match self.bits {
            false => Cstrt2::_0,
            true => Cstrt2::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt2::_0
    }
    #[doc = "GPT322.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt2::_1
    }
}
#[doc = "Field `CSTRT2` writer - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt2W<'a, REG> = crate::BitWriter<'a, REG, Cstrt2>;
impl<'a, REG> Cstrt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt2::_0)
    }
    #[doc = "GPT322.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt2::_1)
    }
}
#[doc = "Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt3 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt3> for bool {
    #[inline(always)]
    fn from(variant: Cstrt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT3` reader - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt3R = crate::BitReader<Cstrt3>;
impl Cstrt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt3 {
        match self.bits {
            false => Cstrt3::_0,
            true => Cstrt3::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt3::_0
    }
    #[doc = "GPT323.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt3::_1
    }
}
#[doc = "Field `CSTRT3` writer - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt3W<'a, REG> = crate::BitWriter<'a, REG, Cstrt3>;
impl<'a, REG> Cstrt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt3::_0)
    }
    #[doc = "GPT323.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt3::_1)
    }
}
#[doc = "Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt4 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt4> for bool {
    #[inline(always)]
    fn from(variant: Cstrt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT4` reader - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt4R = crate::BitReader<Cstrt4>;
impl Cstrt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt4 {
        match self.bits {
            false => Cstrt4::_0,
            true => Cstrt4::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt4::_0
    }
    #[doc = "GPT164.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt4::_1
    }
}
#[doc = "Field `CSTRT4` writer - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt4W<'a, REG> = crate::BitWriter<'a, REG, Cstrt4>;
impl<'a, REG> Cstrt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt4::_0)
    }
    #[doc = "GPT164.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt4::_1)
    }
}
#[doc = "Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt5 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt5> for bool {
    #[inline(always)]
    fn from(variant: Cstrt5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT5` reader - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt5R = crate::BitReader<Cstrt5>;
impl Cstrt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt5 {
        match self.bits {
            false => Cstrt5::_0,
            true => Cstrt5::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt5::_0
    }
    #[doc = "GPT165.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt5::_1
    }
}
#[doc = "Field `CSTRT5` writer - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt5W<'a, REG> = crate::BitWriter<'a, REG, Cstrt5>;
impl<'a, REG> Cstrt5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt5::_0)
    }
    #[doc = "GPT165.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt5::_1)
    }
}
#[doc = "Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt6 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt6> for bool {
    #[inline(always)]
    fn from(variant: Cstrt6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT6` reader - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt6R = crate::BitReader<Cstrt6>;
impl Cstrt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt6 {
        match self.bits {
            false => Cstrt6::_0,
            true => Cstrt6::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt6::_0
    }
    #[doc = "GPT166.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt6::_1
    }
}
#[doc = "Field `CSTRT6` writer - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt6W<'a, REG> = crate::BitWriter<'a, REG, Cstrt6>;
impl<'a, REG> Cstrt6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt6::_0)
    }
    #[doc = "GPT166.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt6::_1)
    }
}
#[doc = "Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt7 {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<Cstrt7> for bool {
    #[inline(always)]
    fn from(variant: Cstrt7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTRT7` reader - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt7R = crate::BitReader<Cstrt7>;
impl Cstrt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt7 {
        match self.bits {
            false => Cstrt7::_0,
            true => Cstrt7::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt7::_0
    }
    #[doc = "GPT167.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt7::_1
    }
}
#[doc = "Field `CSTRT7` writer - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type Cstrt7W<'a, REG> = crate::BitWriter<'a, REG, Cstrt7>;
impl<'a, REG> Cstrt7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt7::_0)
    }
    #[doc = "GPT167.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt0(&self) -> Cstrt0R {
        Cstrt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt1(&self) -> Cstrt1R {
        Cstrt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt2(&self) -> Cstrt2R {
        Cstrt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt3(&self) -> Cstrt3R {
        Cstrt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt4(&self) -> Cstrt4R {
        Cstrt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt5(&self) -> Cstrt5R {
        Cstrt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt6(&self) -> Cstrt6R {
        Cstrt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt7(&self) -> Cstrt7R {
        Cstrt7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSTR")
            .field("cstrt7", &self.cstrt7())
            .field("cstrt6", &self.cstrt6())
            .field("cstrt5", &self.cstrt5())
            .field("cstrt4", &self.cstrt4())
            .field("cstrt3", &self.cstrt3())
            .field("cstrt2", &self.cstrt2())
            .field("cstrt1", &self.cstrt1())
            .field("cstrt0", &self.cstrt0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt0(&mut self) -> Cstrt0W<GtstrSpec> {
        Cstrt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt1(&mut self) -> Cstrt1W<GtstrSpec> {
        Cstrt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt2(&mut self) -> Cstrt2W<GtstrSpec> {
        Cstrt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt3(&mut self) -> Cstrt3W<GtstrSpec> {
        Cstrt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt4(&mut self) -> Cstrt4W<GtstrSpec> {
        Cstrt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt5(&mut self) -> Cstrt5W<GtstrSpec> {
        Cstrt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt6(&mut self) -> Cstrt6W<GtstrSpec> {
        Cstrt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt7(&mut self) -> Cstrt7W<GtstrSpec> {
        Cstrt7W::new(self, 7)
    }
}
#[doc = "General PWM Timer Software Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtstrSpec;
impl crate::RegisterSpec for GtstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtstr::R`](R) reader structure"]
impl crate::Readable for GtstrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtstr::W`](W) writer structure"]
impl crate::Writable for GtstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTSTR to value 0"]
impl crate::Resettable for GtstrSpec {}

#[doc = "Register `NRDYENB` reader"]
pub type R = crate::R<NrdyenbSpec>;
#[doc = "Register `NRDYENB` writer"]
pub type W = crate::W<NrdyenbSpec>;
#[doc = "NRDY Interrupt Enable for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe0nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe0nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE0NRDYE` reader - NRDY Interrupt Enable for PIPE0"]
pub type Pipe0nrdyeR = crate::BitReader<Pipe0nrdye>;
impl Pipe0nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0nrdye {
        match self.bits {
            false => Pipe0nrdye::_0,
            true => Pipe0nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0nrdye::_1
    }
}
#[doc = "Field `PIPE0NRDYE` writer - NRDY Interrupt Enable for PIPE0"]
pub type Pipe0nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe0nrdye>;
impl<'a, REG> Pipe0nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe1nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe1nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE1NRDYE` reader - NRDY Interrupt Enable for PIPE1"]
pub type Pipe1nrdyeR = crate::BitReader<Pipe1nrdye>;
impl Pipe1nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1nrdye {
        match self.bits {
            false => Pipe1nrdye::_0,
            true => Pipe1nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1nrdye::_1
    }
}
#[doc = "Field `PIPE1NRDYE` writer - NRDY Interrupt Enable for PIPE1"]
pub type Pipe1nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe1nrdye>;
impl<'a, REG> Pipe1nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe2nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe2nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE2NRDYE` reader - NRDY Interrupt Enable for PIPE2"]
pub type Pipe2nrdyeR = crate::BitReader<Pipe2nrdye>;
impl Pipe2nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2nrdye {
        match self.bits {
            false => Pipe2nrdye::_0,
            true => Pipe2nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2nrdye::_1
    }
}
#[doc = "Field `PIPE2NRDYE` writer - NRDY Interrupt Enable for PIPE2"]
pub type Pipe2nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe2nrdye>;
impl<'a, REG> Pipe2nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe3nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe3nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE3NRDYE` reader - NRDY Interrupt Enable for PIPE3"]
pub type Pipe3nrdyeR = crate::BitReader<Pipe3nrdye>;
impl Pipe3nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3nrdye {
        match self.bits {
            false => Pipe3nrdye::_0,
            true => Pipe3nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3nrdye::_1
    }
}
#[doc = "Field `PIPE3NRDYE` writer - NRDY Interrupt Enable for PIPE3"]
pub type Pipe3nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe3nrdye>;
impl<'a, REG> Pipe3nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe4nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe4nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE4NRDYE` reader - NRDY Interrupt Enable for PIPE4"]
pub type Pipe4nrdyeR = crate::BitReader<Pipe4nrdye>;
impl Pipe4nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4nrdye {
        match self.bits {
            false => Pipe4nrdye::_0,
            true => Pipe4nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4nrdye::_1
    }
}
#[doc = "Field `PIPE4NRDYE` writer - NRDY Interrupt Enable for PIPE4"]
pub type Pipe4nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe4nrdye>;
impl<'a, REG> Pipe4nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe5nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe5nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE5NRDYE` reader - NRDY Interrupt Enable for PIPE5"]
pub type Pipe5nrdyeR = crate::BitReader<Pipe5nrdye>;
impl Pipe5nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5nrdye {
        match self.bits {
            false => Pipe5nrdye::_0,
            true => Pipe5nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5nrdye::_1
    }
}
#[doc = "Field `PIPE5NRDYE` writer - NRDY Interrupt Enable for PIPE5"]
pub type Pipe5nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe5nrdye>;
impl<'a, REG> Pipe5nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe6nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe6nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE6NRDYE` reader - NRDY Interrupt Enable for PIPE6"]
pub type Pipe6nrdyeR = crate::BitReader<Pipe6nrdye>;
impl Pipe6nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6nrdye {
        match self.bits {
            false => Pipe6nrdye::_0,
            true => Pipe6nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6nrdye::_1
    }
}
#[doc = "Field `PIPE6NRDYE` writer - NRDY Interrupt Enable for PIPE6"]
pub type Pipe6nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe6nrdye>;
impl<'a, REG> Pipe6nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe7nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe7nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE7NRDYE` reader - NRDY Interrupt Enable for PIPE7"]
pub type Pipe7nrdyeR = crate::BitReader<Pipe7nrdye>;
impl Pipe7nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7nrdye {
        match self.bits {
            false => Pipe7nrdye::_0,
            true => Pipe7nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7nrdye::_1
    }
}
#[doc = "Field `PIPE7NRDYE` writer - NRDY Interrupt Enable for PIPE7"]
pub type Pipe7nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe7nrdye>;
impl<'a, REG> Pipe7nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe8nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe8nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE8NRDYE` reader - NRDY Interrupt Enable for PIPE8"]
pub type Pipe8nrdyeR = crate::BitReader<Pipe8nrdye>;
impl Pipe8nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8nrdye {
        match self.bits {
            false => Pipe8nrdye::_0,
            true => Pipe8nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8nrdye::_1
    }
}
#[doc = "Field `PIPE8NRDYE` writer - NRDY Interrupt Enable for PIPE8"]
pub type Pipe8nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe8nrdye>;
impl<'a, REG> Pipe8nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdye::_1)
    }
}
#[doc = "NRDY Interrupt Enable for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pipe9nrdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe9nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE9NRDYE` reader - NRDY Interrupt Enable for PIPE9"]
pub type Pipe9nrdyeR = crate::BitReader<Pipe9nrdye>;
impl Pipe9nrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9nrdye {
        match self.bits {
            false => Pipe9nrdye::_0,
            true => Pipe9nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9nrdye::_1
    }
}
#[doc = "Field `PIPE9NRDYE` writer - NRDY Interrupt Enable for PIPE9"]
pub type Pipe9nrdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe9nrdye>;
impl<'a, REG> Pipe9nrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdye::_1)
    }
}
impl R {
    #[doc = "Bit 0 - NRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdye(&self) -> Pipe0nrdyeR {
        Pipe0nrdyeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdye(&self) -> Pipe1nrdyeR {
        Pipe1nrdyeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdye(&self) -> Pipe2nrdyeR {
        Pipe2nrdyeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdye(&self) -> Pipe3nrdyeR {
        Pipe3nrdyeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdye(&self) -> Pipe4nrdyeR {
        Pipe4nrdyeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdye(&self) -> Pipe5nrdyeR {
        Pipe5nrdyeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdye(&self) -> Pipe6nrdyeR {
        Pipe6nrdyeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdye(&self) -> Pipe7nrdyeR {
        Pipe7nrdyeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdye(&self) -> Pipe8nrdyeR {
        Pipe8nrdyeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdye(&self) -> Pipe9nrdyeR {
        Pipe9nrdyeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRDYENB")
            .field("pipe9nrdye", &self.pipe9nrdye())
            .field("pipe8nrdye", &self.pipe8nrdye())
            .field("pipe7nrdye", &self.pipe7nrdye())
            .field("pipe6nrdye", &self.pipe6nrdye())
            .field("pipe5nrdye", &self.pipe5nrdye())
            .field("pipe4nrdye", &self.pipe4nrdye())
            .field("pipe3nrdye", &self.pipe3nrdye())
            .field("pipe2nrdye", &self.pipe2nrdye())
            .field("pipe1nrdye", &self.pipe1nrdye())
            .field("pipe0nrdye", &self.pipe0nrdye())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdye(&mut self) -> Pipe0nrdyeW<NrdyenbSpec> {
        Pipe0nrdyeW::new(self, 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdye(&mut self) -> Pipe1nrdyeW<NrdyenbSpec> {
        Pipe1nrdyeW::new(self, 1)
    }
    #[doc = "Bit 2 - NRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdye(&mut self) -> Pipe2nrdyeW<NrdyenbSpec> {
        Pipe2nrdyeW::new(self, 2)
    }
    #[doc = "Bit 3 - NRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdye(&mut self) -> Pipe3nrdyeW<NrdyenbSpec> {
        Pipe3nrdyeW::new(self, 3)
    }
    #[doc = "Bit 4 - NRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdye(&mut self) -> Pipe4nrdyeW<NrdyenbSpec> {
        Pipe4nrdyeW::new(self, 4)
    }
    #[doc = "Bit 5 - NRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdye(&mut self) -> Pipe5nrdyeW<NrdyenbSpec> {
        Pipe5nrdyeW::new(self, 5)
    }
    #[doc = "Bit 6 - NRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdye(&mut self) -> Pipe6nrdyeW<NrdyenbSpec> {
        Pipe6nrdyeW::new(self, 6)
    }
    #[doc = "Bit 7 - NRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdye(&mut self) -> Pipe7nrdyeW<NrdyenbSpec> {
        Pipe7nrdyeW::new(self, 7)
    }
    #[doc = "Bit 8 - NRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdye(&mut self) -> Pipe8nrdyeW<NrdyenbSpec> {
        Pipe8nrdyeW::new(self, 8)
    }
    #[doc = "Bit 9 - NRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdye(&mut self) -> Pipe9nrdyeW<NrdyenbSpec> {
        Pipe9nrdyeW::new(self, 9)
    }
}
#[doc = "NRDY Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrdyenbSpec;
impl crate::RegisterSpec for NrdyenbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nrdyenb::R`](R) reader structure"]
impl crate::Readable for NrdyenbSpec {}
#[doc = "`write(|w| ..)` method takes [`nrdyenb::W`](W) writer structure"]
impl crate::Writable for NrdyenbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NRDYENB to value 0"]
impl crate::Resettable for NrdyenbSpec {}

#[doc = "Register `NRDYSTS` reader"]
pub type R = crate::R<NrdystsSpec>;
#[doc = "Register `NRDYSTS` writer"]
pub type W = crate::W<NrdystsSpec>;
#[doc = "NRDY Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe0nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe0nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE0NRDY` reader - NRDY Interrupt Status for PIPE0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe0nrdyR = crate::BitReader<Pipe0nrdy>;
impl Pipe0nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0nrdy {
        match self.bits {
            false => Pipe0nrdy::_0,
            true => Pipe0nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0nrdy::_1
    }
}
#[doc = "Field `PIPE0NRDY` writer - NRDY Interrupt Status for PIPE0"]
pub type Pipe0nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe0nrdy>;
impl<'a, REG> Pipe0nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe1nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe1nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE1NRDY` reader - NRDY Interrupt Status for PIPE1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe1nrdyR = crate::BitReader<Pipe1nrdy>;
impl Pipe1nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1nrdy {
        match self.bits {
            false => Pipe1nrdy::_0,
            true => Pipe1nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1nrdy::_1
    }
}
#[doc = "Field `PIPE1NRDY` writer - NRDY Interrupt Status for PIPE1"]
pub type Pipe1nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe1nrdy>;
impl<'a, REG> Pipe1nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe2nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe2nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE2NRDY` reader - NRDY Interrupt Status for PIPE2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe2nrdyR = crate::BitReader<Pipe2nrdy>;
impl Pipe2nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2nrdy {
        match self.bits {
            false => Pipe2nrdy::_0,
            true => Pipe2nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2nrdy::_1
    }
}
#[doc = "Field `PIPE2NRDY` writer - NRDY Interrupt Status for PIPE2"]
pub type Pipe2nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe2nrdy>;
impl<'a, REG> Pipe2nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe3nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe3nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE3NRDY` reader - NRDY Interrupt Status for PIPE3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe3nrdyR = crate::BitReader<Pipe3nrdy>;
impl Pipe3nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3nrdy {
        match self.bits {
            false => Pipe3nrdy::_0,
            true => Pipe3nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3nrdy::_1
    }
}
#[doc = "Field `PIPE3NRDY` writer - NRDY Interrupt Status for PIPE3"]
pub type Pipe3nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe3nrdy>;
impl<'a, REG> Pipe3nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe4nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe4nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE4NRDY` reader - NRDY Interrupt Status for PIPE4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe4nrdyR = crate::BitReader<Pipe4nrdy>;
impl Pipe4nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4nrdy {
        match self.bits {
            false => Pipe4nrdy::_0,
            true => Pipe4nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4nrdy::_1
    }
}
#[doc = "Field `PIPE4NRDY` writer - NRDY Interrupt Status for PIPE4"]
pub type Pipe4nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe4nrdy>;
impl<'a, REG> Pipe4nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe5nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe5nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE5NRDY` reader - NRDY Interrupt Status for PIPE5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe5nrdyR = crate::BitReader<Pipe5nrdy>;
impl Pipe5nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5nrdy {
        match self.bits {
            false => Pipe5nrdy::_0,
            true => Pipe5nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5nrdy::_1
    }
}
#[doc = "Field `PIPE5NRDY` writer - NRDY Interrupt Status for PIPE5"]
pub type Pipe5nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe5nrdy>;
impl<'a, REG> Pipe5nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe6nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe6nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE6NRDY` reader - NRDY Interrupt Status for PIPE6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe6nrdyR = crate::BitReader<Pipe6nrdy>;
impl Pipe6nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6nrdy {
        match self.bits {
            false => Pipe6nrdy::_0,
            true => Pipe6nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6nrdy::_1
    }
}
#[doc = "Field `PIPE6NRDY` writer - NRDY Interrupt Status for PIPE6"]
pub type Pipe6nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe6nrdy>;
impl<'a, REG> Pipe6nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe7nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe7nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE7NRDY` reader - NRDY Interrupt Status for PIPE7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe7nrdyR = crate::BitReader<Pipe7nrdy>;
impl Pipe7nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7nrdy {
        match self.bits {
            false => Pipe7nrdy::_0,
            true => Pipe7nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7nrdy::_1
    }
}
#[doc = "Field `PIPE7NRDY` writer - NRDY Interrupt Status for PIPE7"]
pub type Pipe7nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe7nrdy>;
impl<'a, REG> Pipe7nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe8nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe8nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE8NRDY` reader - NRDY Interrupt Status for PIPE8\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe8nrdyR = crate::BitReader<Pipe8nrdy>;
impl Pipe8nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8nrdy {
        match self.bits {
            false => Pipe8nrdy::_0,
            true => Pipe8nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8nrdy::_1
    }
}
#[doc = "Field `PIPE8NRDY` writer - NRDY Interrupt Status for PIPE8"]
pub type Pipe8nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe8nrdy>;
impl<'a, REG> Pipe8nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdy::_1)
    }
}
#[doc = "NRDY Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9nrdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe9nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe9nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE9NRDY` reader - NRDY Interrupt Status for PIPE9\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe9nrdyR = crate::BitReader<Pipe9nrdy>;
impl Pipe9nrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9nrdy {
        match self.bits {
            false => Pipe9nrdy::_0,
            true => Pipe9nrdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9nrdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9nrdy::_1
    }
}
#[doc = "Field `PIPE9NRDY` writer - NRDY Interrupt Status for PIPE9"]
pub type Pipe9nrdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe9nrdy>;
impl<'a, REG> Pipe9nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdy::_1)
    }
}
impl R {
    #[doc = "Bit 0 - NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdy(&self) -> Pipe0nrdyR {
        Pipe0nrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdy(&self) -> Pipe1nrdyR {
        Pipe1nrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdy(&self) -> Pipe2nrdyR {
        Pipe2nrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdy(&self) -> Pipe3nrdyR {
        Pipe3nrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdy(&self) -> Pipe4nrdyR {
        Pipe4nrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdy(&self) -> Pipe5nrdyR {
        Pipe5nrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdy(&self) -> Pipe6nrdyR {
        Pipe6nrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdy(&self) -> Pipe7nrdyR {
        Pipe7nrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdy(&self) -> Pipe8nrdyR {
        Pipe8nrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdy(&self) -> Pipe9nrdyR {
        Pipe9nrdyR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRDYSTS").finish()
    }
}
impl W {
    #[doc = "Bit 0 - NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdy(&mut self) -> Pipe0nrdyW<NrdystsSpec> {
        Pipe0nrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdy(&mut self) -> Pipe1nrdyW<NrdystsSpec> {
        Pipe1nrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdy(&mut self) -> Pipe2nrdyW<NrdystsSpec> {
        Pipe2nrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdy(&mut self) -> Pipe3nrdyW<NrdystsSpec> {
        Pipe3nrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdy(&mut self) -> Pipe4nrdyW<NrdystsSpec> {
        Pipe4nrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdy(&mut self) -> Pipe5nrdyW<NrdystsSpec> {
        Pipe5nrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdy(&mut self) -> Pipe6nrdyW<NrdystsSpec> {
        Pipe6nrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdy(&mut self) -> Pipe7nrdyW<NrdystsSpec> {
        Pipe7nrdyW::new(self, 7)
    }
    #[doc = "Bit 8 - NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdy(&mut self) -> Pipe8nrdyW<NrdystsSpec> {
        Pipe8nrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdy(&mut self) -> Pipe9nrdyW<NrdystsSpec> {
        Pipe9nrdyW::new(self, 9)
    }
}
#[doc = "NRDY Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrdystsSpec;
impl crate::RegisterSpec for NrdystsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nrdysts::R`](R) reader structure"]
impl crate::Readable for NrdystsSpec {}
#[doc = "`write(|w| ..)` method takes [`nrdysts::W`](W) writer structure"]
impl crate::Writable for NrdystsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
#[doc = "`reset()` method sets NRDYSTS to value 0"]
impl crate::Resettable for NrdystsSpec {}

#[doc = "Register `BRDYSTS` reader"]
pub type R = crate::R<BrdystsSpec>;
#[doc = "Register `BRDYSTS` writer"]
pub type W = crate::W<BrdystsSpec>;
#[doc = "BRDY Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe0brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe0brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE0BRDY` reader - BRDY Interrupt Status for PIPE0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe0brdyR = crate::BitReader<Pipe0brdy>;
impl Pipe0brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0brdy {
        match self.bits {
            false => Pipe0brdy::_0,
            true => Pipe0brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0brdy::_1
    }
}
#[doc = "Field `PIPE0BRDY` writer - BRDY Interrupt Status for PIPE0"]
pub type Pipe0brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe0brdy>;
impl<'a, REG> Pipe0brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe1brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe1brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE1BRDY` reader - BRDY Interrupt Status for PIPE1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe1brdyR = crate::BitReader<Pipe1brdy>;
impl Pipe1brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1brdy {
        match self.bits {
            false => Pipe1brdy::_0,
            true => Pipe1brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1brdy::_1
    }
}
#[doc = "Field `PIPE1BRDY` writer - BRDY Interrupt Status for PIPE1"]
pub type Pipe1brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe1brdy>;
impl<'a, REG> Pipe1brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe2brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe2brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE2BRDY` reader - BRDY Interrupt Status for PIPE2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe2brdyR = crate::BitReader<Pipe2brdy>;
impl Pipe2brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2brdy {
        match self.bits {
            false => Pipe2brdy::_0,
            true => Pipe2brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2brdy::_1
    }
}
#[doc = "Field `PIPE2BRDY` writer - BRDY Interrupt Status for PIPE2"]
pub type Pipe2brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe2brdy>;
impl<'a, REG> Pipe2brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe3brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe3brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE3BRDY` reader - BRDY Interrupt Status for PIPE3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe3brdyR = crate::BitReader<Pipe3brdy>;
impl Pipe3brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3brdy {
        match self.bits {
            false => Pipe3brdy::_0,
            true => Pipe3brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3brdy::_1
    }
}
#[doc = "Field `PIPE3BRDY` writer - BRDY Interrupt Status for PIPE3"]
pub type Pipe3brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe3brdy>;
impl<'a, REG> Pipe3brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe4brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe4brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE4BRDY` reader - BRDY Interrupt Status for PIPE4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe4brdyR = crate::BitReader<Pipe4brdy>;
impl Pipe4brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4brdy {
        match self.bits {
            false => Pipe4brdy::_0,
            true => Pipe4brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4brdy::_1
    }
}
#[doc = "Field `PIPE4BRDY` writer - BRDY Interrupt Status for PIPE4"]
pub type Pipe4brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe4brdy>;
impl<'a, REG> Pipe4brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe5brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe5brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE5BRDY` reader - BRDY Interrupt Status for PIPE5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe5brdyR = crate::BitReader<Pipe5brdy>;
impl Pipe5brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5brdy {
        match self.bits {
            false => Pipe5brdy::_0,
            true => Pipe5brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5brdy::_1
    }
}
#[doc = "Field `PIPE5BRDY` writer - BRDY Interrupt Status for PIPE5"]
pub type Pipe5brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe5brdy>;
impl<'a, REG> Pipe5brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe6brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe6brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE6BRDY` reader - BRDY Interrupt Status for PIPE6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe6brdyR = crate::BitReader<Pipe6brdy>;
impl Pipe6brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6brdy {
        match self.bits {
            false => Pipe6brdy::_0,
            true => Pipe6brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6brdy::_1
    }
}
#[doc = "Field `PIPE6BRDY` writer - BRDY Interrupt Status for PIPE6"]
pub type Pipe6brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe6brdy>;
impl<'a, REG> Pipe6brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe7brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe7brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE7BRDY` reader - BRDY Interrupt Status for PIPE7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe7brdyR = crate::BitReader<Pipe7brdy>;
impl Pipe7brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7brdy {
        match self.bits {
            false => Pipe7brdy::_0,
            true => Pipe7brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7brdy::_1
    }
}
#[doc = "Field `PIPE7BRDY` writer - BRDY Interrupt Status for PIPE7"]
pub type Pipe7brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe7brdy>;
impl<'a, REG> Pipe7brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe8brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe8brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE8BRDY` reader - BRDY Interrupt Status for PIPE8\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe8brdyR = crate::BitReader<Pipe8brdy>;
impl Pipe8brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8brdy {
        match self.bits {
            false => Pipe8brdy::_0,
            true => Pipe8brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8brdy::_1
    }
}
#[doc = "Field `PIPE8BRDY` writer - BRDY Interrupt Status for PIPE8"]
pub type Pipe8brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe8brdy>;
impl<'a, REG> Pipe8brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8brdy::_1)
    }
}
#[doc = "BRDY Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9brdy {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe9brdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe9brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE9BRDY` reader - BRDY Interrupt Status for PIPE9\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe9brdyR = crate::BitReader<Pipe9brdy>;
impl Pipe9brdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9brdy {
        match self.bits {
            false => Pipe9brdy::_0,
            true => Pipe9brdy::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9brdy::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9brdy::_1
    }
}
#[doc = "Field `PIPE9BRDY` writer - BRDY Interrupt Status for PIPE9"]
pub type Pipe9brdyW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe9brdy>;
impl<'a, REG> Pipe9brdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9brdy::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9brdy::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0brdy(&self) -> Pipe0brdyR {
        Pipe0brdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1brdy(&self) -> Pipe1brdyR {
        Pipe1brdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2brdy(&self) -> Pipe2brdyR {
        Pipe2brdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3brdy(&self) -> Pipe3brdyR {
        Pipe3brdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4brdy(&self) -> Pipe4brdyR {
        Pipe4brdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5brdy(&self) -> Pipe5brdyR {
        Pipe5brdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6brdy(&self) -> Pipe6brdyR {
        Pipe6brdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7brdy(&self) -> Pipe7brdyR {
        Pipe7brdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8brdy(&self) -> Pipe8brdyR {
        Pipe8brdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9brdy(&self) -> Pipe9brdyR {
        Pipe9brdyR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRDYSTS").finish()
    }
}
impl W {
    #[doc = "Bit 0 - BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0brdy(&mut self) -> Pipe0brdyW<BrdystsSpec> {
        Pipe0brdyW::new(self, 0)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1brdy(&mut self) -> Pipe1brdyW<BrdystsSpec> {
        Pipe1brdyW::new(self, 1)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2brdy(&mut self) -> Pipe2brdyW<BrdystsSpec> {
        Pipe2brdyW::new(self, 2)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3brdy(&mut self) -> Pipe3brdyW<BrdystsSpec> {
        Pipe3brdyW::new(self, 3)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4brdy(&mut self) -> Pipe4brdyW<BrdystsSpec> {
        Pipe4brdyW::new(self, 4)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5brdy(&mut self) -> Pipe5brdyW<BrdystsSpec> {
        Pipe5brdyW::new(self, 5)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6brdy(&mut self) -> Pipe6brdyW<BrdystsSpec> {
        Pipe6brdyW::new(self, 6)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7brdy(&mut self) -> Pipe7brdyW<BrdystsSpec> {
        Pipe7brdyW::new(self, 7)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8brdy(&mut self) -> Pipe8brdyW<BrdystsSpec> {
        Pipe8brdyW::new(self, 8)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9brdy(&mut self) -> Pipe9brdyW<BrdystsSpec> {
        Pipe9brdyW::new(self, 9)
    }
}
#[doc = "BRDY Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrdystsSpec;
impl crate::RegisterSpec for BrdystsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`brdysts::R`](R) reader structure"]
impl crate::Readable for BrdystsSpec {}
#[doc = "`write(|w| ..)` method takes [`brdysts::W`](W) writer structure"]
impl crate::Writable for BrdystsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
#[doc = "`reset()` method sets BRDYSTS to value 0"]
impl crate::Resettable for BrdystsSpec {}

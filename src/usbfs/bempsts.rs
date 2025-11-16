#[doc = "Register `BEMPSTS` reader"]
pub type R = crate::R<BempstsSpec>;
#[doc = "Register `BEMPSTS` writer"]
pub type W = crate::W<BempstsSpec>;
#[doc = "BEMP Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe0bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe0bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE0BEMP` reader - BEMP Interrupt Status for PIPE0\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe0bempR = crate::BitReader<Pipe0bemp>;
impl Pipe0bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0bemp {
        match self.bits {
            false => Pipe0bemp::_0,
            true => Pipe0bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0bemp::_1
    }
}
#[doc = "Field `PIPE0BEMP` writer - BEMP Interrupt Status for PIPE0"]
pub type Pipe0bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe0bemp>;
impl<'a, REG> Pipe0bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe1bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe1bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE1BEMP` reader - BEMP Interrupt Status for PIPE1\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe1bempR = crate::BitReader<Pipe1bemp>;
impl Pipe1bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1bemp {
        match self.bits {
            false => Pipe1bemp::_0,
            true => Pipe1bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1bemp::_1
    }
}
#[doc = "Field `PIPE1BEMP` writer - BEMP Interrupt Status for PIPE1"]
pub type Pipe1bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe1bemp>;
impl<'a, REG> Pipe1bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe2bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe2bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE2BEMP` reader - BEMP Interrupt Status for PIPE2\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe2bempR = crate::BitReader<Pipe2bemp>;
impl Pipe2bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2bemp {
        match self.bits {
            false => Pipe2bemp::_0,
            true => Pipe2bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2bemp::_1
    }
}
#[doc = "Field `PIPE2BEMP` writer - BEMP Interrupt Status for PIPE2"]
pub type Pipe2bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe2bemp>;
impl<'a, REG> Pipe2bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe3bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe3bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE3BEMP` reader - BEMP Interrupt Status for PIPE3\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe3bempR = crate::BitReader<Pipe3bemp>;
impl Pipe3bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3bemp {
        match self.bits {
            false => Pipe3bemp::_0,
            true => Pipe3bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3bemp::_1
    }
}
#[doc = "Field `PIPE3BEMP` writer - BEMP Interrupt Status for PIPE3"]
pub type Pipe3bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe3bemp>;
impl<'a, REG> Pipe3bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe4bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe4bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE4BEMP` reader - BEMP Interrupt Status for PIPE4\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe4bempR = crate::BitReader<Pipe4bemp>;
impl Pipe4bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4bemp {
        match self.bits {
            false => Pipe4bemp::_0,
            true => Pipe4bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4bemp::_1
    }
}
#[doc = "Field `PIPE4BEMP` writer - BEMP Interrupt Status for PIPE4"]
pub type Pipe4bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe4bemp>;
impl<'a, REG> Pipe4bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe5bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe5bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE5BEMP` reader - BEMP Interrupt Status for PIPE5\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe5bempR = crate::BitReader<Pipe5bemp>;
impl Pipe5bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5bemp {
        match self.bits {
            false => Pipe5bemp::_0,
            true => Pipe5bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5bemp::_1
    }
}
#[doc = "Field `PIPE5BEMP` writer - BEMP Interrupt Status for PIPE5"]
pub type Pipe5bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe5bemp>;
impl<'a, REG> Pipe5bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe6bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe6bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE6BEMP` reader - BEMP Interrupt Status for PIPE6\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe6bempR = crate::BitReader<Pipe6bemp>;
impl Pipe6bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6bemp {
        match self.bits {
            false => Pipe6bemp::_0,
            true => Pipe6bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6bemp::_1
    }
}
#[doc = "Field `PIPE6BEMP` writer - BEMP Interrupt Status for PIPE6"]
pub type Pipe6bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe6bemp>;
impl<'a, REG> Pipe6bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe7bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe7bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE7BEMP` reader - BEMP Interrupt Status for PIPE7\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe7bempR = crate::BitReader<Pipe7bemp>;
impl Pipe7bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7bemp {
        match self.bits {
            false => Pipe7bemp::_0,
            true => Pipe7bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7bemp::_1
    }
}
#[doc = "Field `PIPE7BEMP` writer - BEMP Interrupt Status for PIPE7"]
pub type Pipe7bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe7bemp>;
impl<'a, REG> Pipe7bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe8bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe8bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE8BEMP` reader - BEMP Interrupt Status for PIPE8\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe8bempR = crate::BitReader<Pipe8bemp>;
impl Pipe8bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8bemp {
        match self.bits {
            false => Pipe8bemp::_0,
            true => Pipe8bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8bemp::_1
    }
}
#[doc = "Field `PIPE8BEMP` writer - BEMP Interrupt Status for PIPE8"]
pub type Pipe8bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe8bemp>;
impl<'a, REG> Pipe8bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8bemp::_1)
    }
}
#[doc = "BEMP Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9bemp {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<Pipe9bemp> for bool {
    #[inline(always)]
    fn from(variant: Pipe9bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPE9BEMP` reader - BEMP Interrupt Status for PIPE9\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pipe9bempR = crate::BitReader<Pipe9bemp>;
impl Pipe9bempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9bemp {
        match self.bits {
            false => Pipe9bemp::_0,
            true => Pipe9bemp::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9bemp::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9bemp::_1
    }
}
#[doc = "Field `PIPE9BEMP` writer - BEMP Interrupt Status for PIPE9"]
pub type Pipe9bempW<'a, REG> = crate::BitWriter0C<'a, REG, Pipe9bemp>;
impl<'a, REG> Pipe9bempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9bemp::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9bemp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0bemp(&self) -> Pipe0bempR {
        Pipe0bempR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1bemp(&self) -> Pipe1bempR {
        Pipe1bempR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2bemp(&self) -> Pipe2bempR {
        Pipe2bempR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3bemp(&self) -> Pipe3bempR {
        Pipe3bempR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4bemp(&self) -> Pipe4bempR {
        Pipe4bempR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5bemp(&self) -> Pipe5bempR {
        Pipe5bempR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6bemp(&self) -> Pipe6bempR {
        Pipe6bempR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7bemp(&self) -> Pipe7bempR {
        Pipe7bempR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8bemp(&self) -> Pipe8bempR {
        Pipe8bempR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9bemp(&self) -> Pipe9bempR {
        Pipe9bempR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BEMPSTS").finish()
    }
}
impl W {
    #[doc = "Bit 0 - BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0bemp(&mut self) -> Pipe0bempW<BempstsSpec> {
        Pipe0bempW::new(self, 0)
    }
    #[doc = "Bit 1 - BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1bemp(&mut self) -> Pipe1bempW<BempstsSpec> {
        Pipe1bempW::new(self, 1)
    }
    #[doc = "Bit 2 - BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2bemp(&mut self) -> Pipe2bempW<BempstsSpec> {
        Pipe2bempW::new(self, 2)
    }
    #[doc = "Bit 3 - BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3bemp(&mut self) -> Pipe3bempW<BempstsSpec> {
        Pipe3bempW::new(self, 3)
    }
    #[doc = "Bit 4 - BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4bemp(&mut self) -> Pipe4bempW<BempstsSpec> {
        Pipe4bempW::new(self, 4)
    }
    #[doc = "Bit 5 - BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5bemp(&mut self) -> Pipe5bempW<BempstsSpec> {
        Pipe5bempW::new(self, 5)
    }
    #[doc = "Bit 6 - BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6bemp(&mut self) -> Pipe6bempW<BempstsSpec> {
        Pipe6bempW::new(self, 6)
    }
    #[doc = "Bit 7 - BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7bemp(&mut self) -> Pipe7bempW<BempstsSpec> {
        Pipe7bempW::new(self, 7)
    }
    #[doc = "Bit 8 - BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8bemp(&mut self) -> Pipe8bempW<BempstsSpec> {
        Pipe8bempW::new(self, 8)
    }
    #[doc = "Bit 9 - BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9bemp(&mut self) -> Pipe9bempW<BempstsSpec> {
        Pipe9bempW::new(self, 9)
    }
}
#[doc = "BEMP Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bempsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BempstsSpec;
impl crate::RegisterSpec for BempstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bempsts::R`](R) reader structure"]
impl crate::Readable for BempstsSpec {}
#[doc = "`write(|w| ..)` method takes [`bempsts::W`](W) writer structure"]
impl crate::Writable for BempstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
#[doc = "`reset()` method sets BEMPSTS to value 0"]
impl crate::Resettable for BempstsSpec {}

#[doc = "Register `SNZREQCR` reader"]
pub type R = crate::R<SnzreqcrSpec>;
#[doc = "Register `SNZREQCR` writer"]
pub type W = crate::W<SnzreqcrSpec>;
#[doc = "Snooze Request Enable 0 Enable IRQ0 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen0 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen0> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN0` reader - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
pub type Snzreqen0R = crate::BitReader<Snzreqen0>;
impl Snzreqen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen0 {
        match self.bits {
            false => Snzreqen0::_0,
            true => Snzreqen0::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen0::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen0::_1
    }
}
#[doc = "Field `SNZREQEN0` writer - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
pub type Snzreqen0W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen0>;
impl<'a, REG> Snzreqen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_1)
    }
}
#[doc = "Snooze Request Enable 1 Enable IRQ1 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen1 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen1> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN1` reader - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
pub type Snzreqen1R = crate::BitReader<Snzreqen1>;
impl Snzreqen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen1 {
        match self.bits {
            false => Snzreqen1::_0,
            true => Snzreqen1::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen1::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen1::_1
    }
}
#[doc = "Field `SNZREQEN1` writer - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
pub type Snzreqen1W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen1>;
impl<'a, REG> Snzreqen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_1)
    }
}
#[doc = "Snooze Request Enable 2 Enable IRQ2 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen2 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen2> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN2` reader - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
pub type Snzreqen2R = crate::BitReader<Snzreqen2>;
impl Snzreqen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen2 {
        match self.bits {
            false => Snzreqen2::_0,
            true => Snzreqen2::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen2::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen2::_1
    }
}
#[doc = "Field `SNZREQEN2` writer - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
pub type Snzreqen2W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen2>;
impl<'a, REG> Snzreqen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_1)
    }
}
#[doc = "Snooze Request Enable 3 Enable IRQ3 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen3 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen3> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN3` reader - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
pub type Snzreqen3R = crate::BitReader<Snzreqen3>;
impl Snzreqen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen3 {
        match self.bits {
            false => Snzreqen3::_0,
            true => Snzreqen3::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen3::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen3::_1
    }
}
#[doc = "Field `SNZREQEN3` writer - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
pub type Snzreqen3W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen3>;
impl<'a, REG> Snzreqen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_1)
    }
}
#[doc = "Snooze Request Enable 4 Enable IRQ4 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen4 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen4> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN4` reader - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
pub type Snzreqen4R = crate::BitReader<Snzreqen4>;
impl Snzreqen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen4 {
        match self.bits {
            false => Snzreqen4::_0,
            true => Snzreqen4::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen4::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen4::_1
    }
}
#[doc = "Field `SNZREQEN4` writer - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
pub type Snzreqen4W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen4>;
impl<'a, REG> Snzreqen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_1)
    }
}
#[doc = "Snooze Request Enable 5 Enable IRQ5 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen5 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen5> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN5` reader - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
pub type Snzreqen5R = crate::BitReader<Snzreqen5>;
impl Snzreqen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen5 {
        match self.bits {
            false => Snzreqen5::_0,
            true => Snzreqen5::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen5::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen5::_1
    }
}
#[doc = "Field `SNZREQEN5` writer - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
pub type Snzreqen5W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen5>;
impl<'a, REG> Snzreqen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_1)
    }
}
#[doc = "Snooze Request Enable 6 Enable IRQ6 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen6 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen6> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN6` reader - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
pub type Snzreqen6R = crate::BitReader<Snzreqen6>;
impl Snzreqen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen6 {
        match self.bits {
            false => Snzreqen6::_0,
            true => Snzreqen6::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen6::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen6::_1
    }
}
#[doc = "Field `SNZREQEN6` writer - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
pub type Snzreqen6W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen6>;
impl<'a, REG> Snzreqen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_1)
    }
}
#[doc = "Snooze Request Enable 7 Enable IRQ7 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen7 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen7> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN7` reader - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
pub type Snzreqen7R = crate::BitReader<Snzreqen7>;
impl Snzreqen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen7 {
        match self.bits {
            false => Snzreqen7::_0,
            true => Snzreqen7::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen7::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen7::_1
    }
}
#[doc = "Field `SNZREQEN7` writer - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
pub type Snzreqen7W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen7>;
impl<'a, REG> Snzreqen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_1)
    }
}
#[doc = "Snooze Request Enable 8 Enable IRQ8 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen8 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen8> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN8` reader - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
pub type Snzreqen8R = crate::BitReader<Snzreqen8>;
impl Snzreqen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen8 {
        match self.bits {
            false => Snzreqen8::_0,
            true => Snzreqen8::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen8::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen8::_1
    }
}
#[doc = "Field `SNZREQEN8` writer - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
pub type Snzreqen8W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen8>;
impl<'a, REG> Snzreqen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen8::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen8::_1)
    }
}
#[doc = "Snooze Request Enable 9 Enable IRQ9 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen9 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen9> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN9` reader - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
pub type Snzreqen9R = crate::BitReader<Snzreqen9>;
impl Snzreqen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen9 {
        match self.bits {
            false => Snzreqen9::_0,
            true => Snzreqen9::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen9::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen9::_1
    }
}
#[doc = "Field `SNZREQEN9` writer - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
pub type Snzreqen9W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen9>;
impl<'a, REG> Snzreqen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen9::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen9::_1)
    }
}
#[doc = "Snooze Request Enable 10 Enable IRQ10 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen10 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen10> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN10` reader - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
pub type Snzreqen10R = crate::BitReader<Snzreqen10>;
impl Snzreqen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen10 {
        match self.bits {
            false => Snzreqen10::_0,
            true => Snzreqen10::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen10::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen10::_1
    }
}
#[doc = "Field `SNZREQEN10` writer - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
pub type Snzreqen10W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen10>;
impl<'a, REG> Snzreqen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen10::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen10::_1)
    }
}
#[doc = "Snooze Request Enable 11 Enable IRQ11 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen11 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen11> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN11` reader - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
pub type Snzreqen11R = crate::BitReader<Snzreqen11>;
impl Snzreqen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen11 {
        match self.bits {
            false => Snzreqen11::_0,
            true => Snzreqen11::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen11::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen11::_1
    }
}
#[doc = "Field `SNZREQEN11` writer - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
pub type Snzreqen11W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen11>;
impl<'a, REG> Snzreqen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen11::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen11::_1)
    }
}
#[doc = "Snooze Request Enable 12 Enable IRQ12 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen12 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen12> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN12` reader - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
pub type Snzreqen12R = crate::BitReader<Snzreqen12>;
impl Snzreqen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen12 {
        match self.bits {
            false => Snzreqen12::_0,
            true => Snzreqen12::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen12::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen12::_1
    }
}
#[doc = "Field `SNZREQEN12` writer - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
pub type Snzreqen12W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen12>;
impl<'a, REG> Snzreqen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen12::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen12::_1)
    }
}
#[doc = "Snooze Request Enable 14 Enable IRQ14 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen14 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen14> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN14` reader - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
pub type Snzreqen14R = crate::BitReader<Snzreqen14>;
impl Snzreqen14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen14 {
        match self.bits {
            false => Snzreqen14::_0,
            true => Snzreqen14::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen14::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen14::_1
    }
}
#[doc = "Field `SNZREQEN14` writer - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
pub type Snzreqen14W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen14>;
impl<'a, REG> Snzreqen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen14::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen14::_1)
    }
}
#[doc = "Snooze Request Enable 15 Enable IRQ15 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen15 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen15> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN15` reader - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
pub type Snzreqen15R = crate::BitReader<Snzreqen15>;
impl Snzreqen15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen15 {
        match self.bits {
            false => Snzreqen15::_0,
            true => Snzreqen15::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen15::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen15::_1
    }
}
#[doc = "Field `SNZREQEN15` writer - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
pub type Snzreqen15W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen15>;
impl<'a, REG> Snzreqen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen15::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen15::_1)
    }
}
#[doc = "Snooze Request Enable 17 Enable KINT snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen17 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen17> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN17` reader - Snooze Request Enable 17 Enable KINT snooze request"]
pub type Snzreqen17R = crate::BitReader<Snzreqen17>;
impl Snzreqen17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen17 {
        match self.bits {
            false => Snzreqen17::_0,
            true => Snzreqen17::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen17::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen17::_1
    }
}
#[doc = "Field `SNZREQEN17` writer - Snooze Request Enable 17 Enable KINT snooze request"]
pub type Snzreqen17W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen17>;
impl<'a, REG> Snzreqen17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen17::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen17::_1)
    }
}
#[doc = "Snooze Request Enable 23 Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen23 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen23> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN23` reader - Snooze Request Enable 23 Enable RTC alarm snooze request"]
pub type Snzreqen23R = crate::BitReader<Snzreqen23>;
impl Snzreqen23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen23 {
        match self.bits {
            false => Snzreqen23::_0,
            true => Snzreqen23::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen23::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen23::_1
    }
}
#[doc = "Field `SNZREQEN23` writer - Snooze Request Enable 23 Enable RTC alarm snooze request"]
pub type Snzreqen23W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen23>;
impl<'a, REG> Snzreqen23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen23::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen23::_1)
    }
}
#[doc = "Snooze Request Enable 24 Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen24 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen24> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN24` reader - Snooze Request Enable 24 Enable RTC alarm snooze request"]
pub type Snzreqen24R = crate::BitReader<Snzreqen24>;
impl Snzreqen24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen24 {
        match self.bits {
            false => Snzreqen24::_0,
            true => Snzreqen24::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen24::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen24::_1
    }
}
#[doc = "Field `SNZREQEN24` writer - Snooze Request Enable 24 Enable RTC alarm snooze request"]
pub type Snzreqen24W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen24>;
impl<'a, REG> Snzreqen24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_1)
    }
}
#[doc = "Snooze Request Enable 25 Enable RTC period snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen25 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen25> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN25` reader - Snooze Request Enable 25 Enable RTC period snooze request"]
pub type Snzreqen25R = crate::BitReader<Snzreqen25>;
impl Snzreqen25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen25 {
        match self.bits {
            false => Snzreqen25::_0,
            true => Snzreqen25::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen25::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen25::_1
    }
}
#[doc = "Field `SNZREQEN25` writer - Snooze Request Enable 25 Enable RTC period snooze request"]
pub type Snzreqen25W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen25>;
impl<'a, REG> Snzreqen25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen25::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen25::_1)
    }
}
#[doc = "Snooze Request Enable 28 Enable AGT1 underflow snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen28 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen28> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN28` reader - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
pub type Snzreqen28R = crate::BitReader<Snzreqen28>;
impl Snzreqen28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen28 {
        match self.bits {
            false => Snzreqen28::_0,
            true => Snzreqen28::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen28::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen28::_1
    }
}
#[doc = "Field `SNZREQEN28` writer - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
pub type Snzreqen28W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen28>;
impl<'a, REG> Snzreqen28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen28::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen28::_1)
    }
}
#[doc = "Snooze Request Enable 29 Enable AGT1 compare match A snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen29 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen29> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN29` reader - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
pub type Snzreqen29R = crate::BitReader<Snzreqen29>;
impl Snzreqen29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen29 {
        match self.bits {
            false => Snzreqen29::_0,
            true => Snzreqen29::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen29::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen29::_1
    }
}
#[doc = "Field `SNZREQEN29` writer - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
pub type Snzreqen29W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen29>;
impl<'a, REG> Snzreqen29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen29::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen29::_1)
    }
}
#[doc = "Snooze Request Enable 30 Enable AGT1 compare match B snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen30 {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<Snzreqen30> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN30` reader - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
pub type Snzreqen30R = crate::BitReader<Snzreqen30>;
impl Snzreqen30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen30 {
        match self.bits {
            false => Snzreqen30::_0,
            true => Snzreqen30::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen30::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen30::_1
    }
}
#[doc = "Field `SNZREQEN30` writer - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
pub type Snzreqen30W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen30>;
impl<'a, REG> Snzreqen30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen30::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen30::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&self) -> Snzreqen0R {
        Snzreqen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&self) -> Snzreqen1R {
        Snzreqen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&self) -> Snzreqen2R {
        Snzreqen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&self) -> Snzreqen3R {
        Snzreqen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&self) -> Snzreqen4R {
        Snzreqen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&self) -> Snzreqen5R {
        Snzreqen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&self) -> Snzreqen6R {
        Snzreqen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&self) -> Snzreqen7R {
        Snzreqen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(&self) -> Snzreqen8R {
        Snzreqen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(&self) -> Snzreqen9R {
        Snzreqen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(&self) -> Snzreqen10R {
        Snzreqen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(&self) -> Snzreqen11R {
        Snzreqen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(&self) -> Snzreqen12R {
        Snzreqen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen14(&self) -> Snzreqen14R {
        Snzreqen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen15(&self) -> Snzreqen15R {
        Snzreqen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Snooze Request Enable 17 Enable KINT snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&self) -> Snzreqen17R {
        Snzreqen17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen23(&self) -> Snzreqen23R {
        Snzreqen23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&self) -> Snzreqen24R {
        Snzreqen24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Snooze Request Enable 25 Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(&self) -> Snzreqen25R {
        Snzreqen25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(&self) -> Snzreqen28R {
        Snzreqen28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(&self) -> Snzreqen29R {
        Snzreqen29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(&self) -> Snzreqen30R {
        Snzreqen30R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZREQCR")
            .field("snzreqen30", &self.snzreqen30())
            .field("snzreqen29", &self.snzreqen29())
            .field("snzreqen28", &self.snzreqen28())
            .field("snzreqen25", &self.snzreqen25())
            .field("snzreqen24", &self.snzreqen24())
            .field("snzreqen23", &self.snzreqen23())
            .field("snzreqen17", &self.snzreqen17())
            .field("snzreqen15", &self.snzreqen15())
            .field("snzreqen14", &self.snzreqen14())
            .field("snzreqen12", &self.snzreqen12())
            .field("snzreqen11", &self.snzreqen11())
            .field("snzreqen10", &self.snzreqen10())
            .field("snzreqen9", &self.snzreqen9())
            .field("snzreqen8", &self.snzreqen8())
            .field("snzreqen7", &self.snzreqen7())
            .field("snzreqen6", &self.snzreqen6())
            .field("snzreqen5", &self.snzreqen5())
            .field("snzreqen4", &self.snzreqen4())
            .field("snzreqen3", &self.snzreqen3())
            .field("snzreqen2", &self.snzreqen2())
            .field("snzreqen1", &self.snzreqen1())
            .field("snzreqen0", &self.snzreqen0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&mut self) -> Snzreqen0W<SnzreqcrSpec> {
        Snzreqen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&mut self) -> Snzreqen1W<SnzreqcrSpec> {
        Snzreqen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&mut self) -> Snzreqen2W<SnzreqcrSpec> {
        Snzreqen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&mut self) -> Snzreqen3W<SnzreqcrSpec> {
        Snzreqen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&mut self) -> Snzreqen4W<SnzreqcrSpec> {
        Snzreqen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&mut self) -> Snzreqen5W<SnzreqcrSpec> {
        Snzreqen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&mut self) -> Snzreqen6W<SnzreqcrSpec> {
        Snzreqen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&mut self) -> Snzreqen7W<SnzreqcrSpec> {
        Snzreqen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(&mut self) -> Snzreqen8W<SnzreqcrSpec> {
        Snzreqen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(&mut self) -> Snzreqen9W<SnzreqcrSpec> {
        Snzreqen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(&mut self) -> Snzreqen10W<SnzreqcrSpec> {
        Snzreqen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(&mut self) -> Snzreqen11W<SnzreqcrSpec> {
        Snzreqen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(&mut self) -> Snzreqen12W<SnzreqcrSpec> {
        Snzreqen12W::new(self, 12)
    }
    #[doc = "Bit 14 - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen14(&mut self) -> Snzreqen14W<SnzreqcrSpec> {
        Snzreqen14W::new(self, 14)
    }
    #[doc = "Bit 15 - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen15(&mut self) -> Snzreqen15W<SnzreqcrSpec> {
        Snzreqen15W::new(self, 15)
    }
    #[doc = "Bit 17 - Snooze Request Enable 17 Enable KINT snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&mut self) -> Snzreqen17W<SnzreqcrSpec> {
        Snzreqen17W::new(self, 17)
    }
    #[doc = "Bit 23 - Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen23(&mut self) -> Snzreqen23W<SnzreqcrSpec> {
        Snzreqen23W::new(self, 23)
    }
    #[doc = "Bit 24 - Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&mut self) -> Snzreqen24W<SnzreqcrSpec> {
        Snzreqen24W::new(self, 24)
    }
    #[doc = "Bit 25 - Snooze Request Enable 25 Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(&mut self) -> Snzreqen25W<SnzreqcrSpec> {
        Snzreqen25W::new(self, 25)
    }
    #[doc = "Bit 28 - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(&mut self) -> Snzreqen28W<SnzreqcrSpec> {
        Snzreqen28W::new(self, 28)
    }
    #[doc = "Bit 29 - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(&mut self) -> Snzreqen29W<SnzreqcrSpec> {
        Snzreqen29W::new(self, 29)
    }
    #[doc = "Bit 30 - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(&mut self) -> Snzreqen30W<SnzreqcrSpec> {
        Snzreqen30W::new(self, 30)
    }
}
#[doc = "Snooze Request Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzreqcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnzreqcrSpec;
impl crate::RegisterSpec for SnzreqcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snzreqcr::R`](R) reader structure"]
impl crate::Readable for SnzreqcrSpec {}
#[doc = "`write(|w| ..)` method takes [`snzreqcr::W`](W) writer structure"]
impl crate::Writable for SnzreqcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNZREQCR to value 0"]
impl crate::Resettable for SnzreqcrSpec {}

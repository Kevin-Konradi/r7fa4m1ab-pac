#[doc = "Register `WUPEN` reader"]
pub type R = crate::R<WupenSpec>;
#[doc = "Register `WUPEN` writer"]
pub type W = crate::W<WupenSpec>;
#[doc = "IRQ0 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen0 {
    #[doc = "0: S/W standby returns by IRQ0 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ0 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen0> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN0` reader - IRQ0 interrupt S/W standby returns enable"]
pub type Irqwupen0R = crate::BitReader<Irqwupen0>;
impl Irqwupen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen0 {
        match self.bits {
            false => Irqwupen0::_0,
            true => Irqwupen0::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ0 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen0::_0
    }
    #[doc = "S/W standby returns by IRQ0 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen0::_1
    }
}
#[doc = "Field `IRQWUPEN0` writer - IRQ0 interrupt S/W standby returns enable"]
pub type Irqwupen0W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen0>;
impl<'a, REG> Irqwupen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ0 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen0::_0)
    }
    #[doc = "S/W standby returns by IRQ0 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen0::_1)
    }
}
#[doc = "IRQ1 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen1 {
    #[doc = "0: S/W standby returns by IRQ1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ1 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen1> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN1` reader - IRQ1 interrupt S/W standby returns enable"]
pub type Irqwupen1R = crate::BitReader<Irqwupen1>;
impl Irqwupen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen1 {
        match self.bits {
            false => Irqwupen1::_0,
            true => Irqwupen1::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ1 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen1::_0
    }
    #[doc = "S/W standby returns by IRQ1 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen1::_1
    }
}
#[doc = "Field `IRQWUPEN1` writer - IRQ1 interrupt S/W standby returns enable"]
pub type Irqwupen1W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen1>;
impl<'a, REG> Irqwupen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen1::_0)
    }
    #[doc = "S/W standby returns by IRQ1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen1::_1)
    }
}
#[doc = "IRQ2 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen2 {
    #[doc = "0: S/W standby returns by IRQ2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ2 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen2> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN2` reader - IRQ2 interrupt S/W standby returns enable"]
pub type Irqwupen2R = crate::BitReader<Irqwupen2>;
impl Irqwupen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen2 {
        match self.bits {
            false => Irqwupen2::_0,
            true => Irqwupen2::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ2 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen2::_0
    }
    #[doc = "S/W standby returns by IRQ2 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen2::_1
    }
}
#[doc = "Field `IRQWUPEN2` writer - IRQ2 interrupt S/W standby returns enable"]
pub type Irqwupen2W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen2>;
impl<'a, REG> Irqwupen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen2::_0)
    }
    #[doc = "S/W standby returns by IRQ2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen2::_1)
    }
}
#[doc = "IRQ3 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen3 {
    #[doc = "0: S/W standby returns by IRQ3 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ3 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen3> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN3` reader - IRQ3 interrupt S/W standby returns enable"]
pub type Irqwupen3R = crate::BitReader<Irqwupen3>;
impl Irqwupen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen3 {
        match self.bits {
            false => Irqwupen3::_0,
            true => Irqwupen3::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ3 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen3::_0
    }
    #[doc = "S/W standby returns by IRQ3 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen3::_1
    }
}
#[doc = "Field `IRQWUPEN3` writer - IRQ3 interrupt S/W standby returns enable"]
pub type Irqwupen3W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen3>;
impl<'a, REG> Irqwupen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ3 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen3::_0)
    }
    #[doc = "S/W standby returns by IRQ3 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen3::_1)
    }
}
#[doc = "IRQ4 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen4 {
    #[doc = "0: S/W standby returns by IRQ4 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ4 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen4> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN4` reader - IRQ4 interrupt S/W standby returns enable"]
pub type Irqwupen4R = crate::BitReader<Irqwupen4>;
impl Irqwupen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen4 {
        match self.bits {
            false => Irqwupen4::_0,
            true => Irqwupen4::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ4 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen4::_0
    }
    #[doc = "S/W standby returns by IRQ4 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen4::_1
    }
}
#[doc = "Field `IRQWUPEN4` writer - IRQ4 interrupt S/W standby returns enable"]
pub type Irqwupen4W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen4>;
impl<'a, REG> Irqwupen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ4 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen4::_0)
    }
    #[doc = "S/W standby returns by IRQ4 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen4::_1)
    }
}
#[doc = "IRQ5 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen5 {
    #[doc = "0: S/W standby returns by IRQ5 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ5 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen5> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN5` reader - IRQ5 interrupt S/W standby returns enable"]
pub type Irqwupen5R = crate::BitReader<Irqwupen5>;
impl Irqwupen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen5 {
        match self.bits {
            false => Irqwupen5::_0,
            true => Irqwupen5::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ5 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen5::_0
    }
    #[doc = "S/W standby returns by IRQ5 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen5::_1
    }
}
#[doc = "Field `IRQWUPEN5` writer - IRQ5 interrupt S/W standby returns enable"]
pub type Irqwupen5W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen5>;
impl<'a, REG> Irqwupen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ5 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen5::_0)
    }
    #[doc = "S/W standby returns by IRQ5 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen5::_1)
    }
}
#[doc = "IRQ6 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen6 {
    #[doc = "0: S/W standby returns by IRQ6 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ6 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen6> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN6` reader - IRQ6 interrupt S/W standby returns enable"]
pub type Irqwupen6R = crate::BitReader<Irqwupen6>;
impl Irqwupen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen6 {
        match self.bits {
            false => Irqwupen6::_0,
            true => Irqwupen6::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ6 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen6::_0
    }
    #[doc = "S/W standby returns by IRQ6 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen6::_1
    }
}
#[doc = "Field `IRQWUPEN6` writer - IRQ6 interrupt S/W standby returns enable"]
pub type Irqwupen6W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen6>;
impl<'a, REG> Irqwupen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ6 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen6::_0)
    }
    #[doc = "S/W standby returns by IRQ6 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen6::_1)
    }
}
#[doc = "IRQ7 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen7 {
    #[doc = "0: S/W standby returns by IRQ7 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ7 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen7> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN7` reader - IRQ7 interrupt S/W standby returns enable"]
pub type Irqwupen7R = crate::BitReader<Irqwupen7>;
impl Irqwupen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen7 {
        match self.bits {
            false => Irqwupen7::_0,
            true => Irqwupen7::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ7 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen7::_0
    }
    #[doc = "S/W standby returns by IRQ7 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen7::_1
    }
}
#[doc = "Field `IRQWUPEN7` writer - IRQ7 interrupt S/W standby returns enable"]
pub type Irqwupen7W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen7>;
impl<'a, REG> Irqwupen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ7 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen7::_0)
    }
    #[doc = "S/W standby returns by IRQ7 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen7::_1)
    }
}
#[doc = "IRQ8 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen8 {
    #[doc = "0: S/W standby returns by IRQ8 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ8 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen8> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN8` reader - IRQ8 interrupt S/W standby returns enable"]
pub type Irqwupen8R = crate::BitReader<Irqwupen8>;
impl Irqwupen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen8 {
        match self.bits {
            false => Irqwupen8::_0,
            true => Irqwupen8::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ8 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen8::_0
    }
    #[doc = "S/W standby returns by IRQ8 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen8::_1
    }
}
#[doc = "Field `IRQWUPEN8` writer - IRQ8 interrupt S/W standby returns enable"]
pub type Irqwupen8W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen8>;
impl<'a, REG> Irqwupen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ8 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen8::_0)
    }
    #[doc = "S/W standby returns by IRQ8 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen8::_1)
    }
}
#[doc = "IRQ9 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen9 {
    #[doc = "0: S/W standby returns by IRQ9 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ9 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen9> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN9` reader - IRQ9 interrupt S/W standby returns enable"]
pub type Irqwupen9R = crate::BitReader<Irqwupen9>;
impl Irqwupen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen9 {
        match self.bits {
            false => Irqwupen9::_0,
            true => Irqwupen9::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ9 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen9::_0
    }
    #[doc = "S/W standby returns by IRQ9 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen9::_1
    }
}
#[doc = "Field `IRQWUPEN9` writer - IRQ9 interrupt S/W standby returns enable"]
pub type Irqwupen9W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen9>;
impl<'a, REG> Irqwupen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ9 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen9::_0)
    }
    #[doc = "S/W standby returns by IRQ9 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen9::_1)
    }
}
#[doc = "IRQ10 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen10 {
    #[doc = "0: S/W standby returns by IRQ10 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ10 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen10> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN10` reader - IRQ10 interrupt S/W standby returns enable"]
pub type Irqwupen10R = crate::BitReader<Irqwupen10>;
impl Irqwupen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen10 {
        match self.bits {
            false => Irqwupen10::_0,
            true => Irqwupen10::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ10 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen10::_0
    }
    #[doc = "S/W standby returns by IRQ10 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen10::_1
    }
}
#[doc = "Field `IRQWUPEN10` writer - IRQ10 interrupt S/W standby returns enable"]
pub type Irqwupen10W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen10>;
impl<'a, REG> Irqwupen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ10 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen10::_0)
    }
    #[doc = "S/W standby returns by IRQ10 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen10::_1)
    }
}
#[doc = "IRQ11 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen11 {
    #[doc = "0: S/W standby returns by IRQ11 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ11 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen11> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN11` reader - IRQ11 interrupt S/W standby returns enable"]
pub type Irqwupen11R = crate::BitReader<Irqwupen11>;
impl Irqwupen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen11 {
        match self.bits {
            false => Irqwupen11::_0,
            true => Irqwupen11::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ11 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen11::_0
    }
    #[doc = "S/W standby returns by IRQ11 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen11::_1
    }
}
#[doc = "Field `IRQWUPEN11` writer - IRQ11 interrupt S/W standby returns enable"]
pub type Irqwupen11W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen11>;
impl<'a, REG> Irqwupen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ11 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen11::_0)
    }
    #[doc = "S/W standby returns by IRQ11 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen11::_1)
    }
}
#[doc = "IRQ12 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen12 {
    #[doc = "0: S/W standby returns by IRQ12 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ12 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen12> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN12` reader - IRQ12 interrupt S/W standby returns enable"]
pub type Irqwupen12R = crate::BitReader<Irqwupen12>;
impl Irqwupen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen12 {
        match self.bits {
            false => Irqwupen12::_0,
            true => Irqwupen12::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ12 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen12::_0
    }
    #[doc = "S/W standby returns by IRQ12 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen12::_1
    }
}
#[doc = "Field `IRQWUPEN12` writer - IRQ12 interrupt S/W standby returns enable"]
pub type Irqwupen12W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen12>;
impl<'a, REG> Irqwupen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ12 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen12::_0)
    }
    #[doc = "S/W standby returns by IRQ12 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen12::_1)
    }
}
#[doc = "IRQ14 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen14 {
    #[doc = "0: S/W standby returns by IRQ14 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ14 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen14> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN14` reader - IRQ14 interrupt S/W standby returns enable"]
pub type Irqwupen14R = crate::BitReader<Irqwupen14>;
impl Irqwupen14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen14 {
        match self.bits {
            false => Irqwupen14::_0,
            true => Irqwupen14::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ14 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen14::_0
    }
    #[doc = "S/W standby returns by IRQ14 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen14::_1
    }
}
#[doc = "Field `IRQWUPEN14` writer - IRQ14 interrupt S/W standby returns enable"]
pub type Irqwupen14W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen14>;
impl<'a, REG> Irqwupen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ14 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen14::_0)
    }
    #[doc = "S/W standby returns by IRQ14 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen14::_1)
    }
}
#[doc = "IRQ15 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqwupen15 {
    #[doc = "0: S/W standby returns by IRQ15 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IRQ15 interrupt is enabled"]
    _1 = 1,
}
impl From<Irqwupen15> for bool {
    #[inline(always)]
    fn from(variant: Irqwupen15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQWUPEN15` reader - IRQ15 interrupt S/W standby returns enable"]
pub type Irqwupen15R = crate::BitReader<Irqwupen15>;
impl Irqwupen15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqwupen15 {
        match self.bits {
            false => Irqwupen15::_0,
            true => Irqwupen15::_1,
        }
    }
    #[doc = "S/W standby returns by IRQ15 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen15::_0
    }
    #[doc = "S/W standby returns by IRQ15 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen15::_1
    }
}
#[doc = "Field `IRQWUPEN15` writer - IRQ15 interrupt S/W standby returns enable"]
pub type Irqwupen15W<'a, REG> = crate::BitWriter<'a, REG, Irqwupen15>;
impl<'a, REG> Irqwupen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IRQ15 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen15::_0)
    }
    #[doc = "S/W standby returns by IRQ15 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen15::_1)
    }
}
#[doc = "IWDT interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtwupen {
    #[doc = "0: S/W standby returns by IWDT interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IWDT interrupt is enabled"]
    _1 = 1,
}
impl From<Iwdtwupen> for bool {
    #[inline(always)]
    fn from(variant: Iwdtwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTWUPEN` reader - IWDT interrupt S/W standby returns enable"]
pub type IwdtwupenR = crate::BitReader<Iwdtwupen>;
impl IwdtwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtwupen {
        match self.bits {
            false => Iwdtwupen::_0,
            true => Iwdtwupen::_1,
        }
    }
    #[doc = "S/W standby returns by IWDT interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtwupen::_0
    }
    #[doc = "S/W standby returns by IWDT interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtwupen::_1
    }
}
#[doc = "Field `IWDTWUPEN` writer - IWDT interrupt S/W standby returns enable"]
pub type IwdtwupenW<'a, REG> = crate::BitWriter<'a, REG, Iwdtwupen>;
impl<'a, REG> IwdtwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IWDT interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_0)
    }
    #[doc = "S/W standby returns by IWDT interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_1)
    }
}
#[doc = "Key interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keywupen {
    #[doc = "0: S/W standby returns by KEY interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by KEY interrupt is enabled"]
    _1 = 1,
}
impl From<Keywupen> for bool {
    #[inline(always)]
    fn from(variant: Keywupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYWUPEN` reader - Key interrupt S/W standby returns enable"]
pub type KeywupenR = crate::BitReader<Keywupen>;
impl KeywupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keywupen {
        match self.bits {
            false => Keywupen::_0,
            true => Keywupen::_1,
        }
    }
    #[doc = "S/W standby returns by KEY interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Keywupen::_0
    }
    #[doc = "S/W standby returns by KEY interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Keywupen::_1
    }
}
#[doc = "Field `KEYWUPEN` writer - Key interrupt S/W standby returns enable"]
pub type KeywupenW<'a, REG> = crate::BitWriter<'a, REG, Keywupen>;
impl<'a, REG> KeywupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by KEY interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Keywupen::_0)
    }
    #[doc = "S/W standby returns by KEY interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Keywupen::_1)
    }
}
#[doc = "LVD1 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1wupen {
    #[doc = "0: S/W standby returns by LVD1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by LVD1 interrupt is enabled"]
    _1 = 1,
}
impl From<Lvd1wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd1wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1WUPEN` reader - LVD1 interrupt S/W standby returns enable"]
pub type Lvd1wupenR = crate::BitReader<Lvd1wupen>;
impl Lvd1wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1wupen {
        match self.bits {
            false => Lvd1wupen::_0,
            true => Lvd1wupen::_1,
        }
    }
    #[doc = "S/W standby returns by LVD1 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1wupen::_0
    }
    #[doc = "S/W standby returns by LVD1 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1wupen::_1
    }
}
#[doc = "Field `LVD1WUPEN` writer - LVD1 interrupt S/W standby returns enable"]
pub type Lvd1wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd1wupen>;
impl<'a, REG> Lvd1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by LVD1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_0)
    }
    #[doc = "S/W standby returns by LVD1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_1)
    }
}
#[doc = "LVD2 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2wupen {
    #[doc = "0: S/W standby returns by LVD2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by LVD2 interrupt is enabled"]
    _1 = 1,
}
impl From<Lvd2wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd2wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2WUPEN` reader - LVD2 interrupt S/W standby returns enable"]
pub type Lvd2wupenR = crate::BitReader<Lvd2wupen>;
impl Lvd2wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2wupen {
        match self.bits {
            false => Lvd2wupen::_0,
            true => Lvd2wupen::_1,
        }
    }
    #[doc = "S/W standby returns by LVD2 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2wupen::_0
    }
    #[doc = "S/W standby returns by LVD2 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2wupen::_1
    }
}
#[doc = "Field `LVD2WUPEN` writer - LVD2 interrupt S/W standby returns enable"]
pub type Lvd2wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd2wupen>;
impl<'a, REG> Lvd2wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by LVD2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_0)
    }
    #[doc = "S/W standby returns by LVD2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_1)
    }
}
#[doc = "VBATT monitor interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbattwupen {
    #[doc = "0: S/W standby returns by VBATT monitor interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by VBATT monitor interrupt is enabled"]
    _1 = 1,
}
impl From<Vbattwupen> for bool {
    #[inline(always)]
    fn from(variant: Vbattwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATTWUPEN` reader - VBATT monitor interrupt S/W standby returns enable"]
pub type VbattwupenR = crate::BitReader<Vbattwupen>;
impl VbattwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbattwupen {
        match self.bits {
            false => Vbattwupen::_0,
            true => Vbattwupen::_1,
        }
    }
    #[doc = "S/W standby returns by VBATT monitor interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbattwupen::_0
    }
    #[doc = "S/W standby returns by VBATT monitor interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbattwupen::_1
    }
}
#[doc = "Field `VBATTWUPEN` writer - VBATT monitor interrupt S/W standby returns enable"]
pub type VbattwupenW<'a, REG> = crate::BitWriter<'a, REG, Vbattwupen>;
impl<'a, REG> VbattwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by VBATT monitor interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattwupen::_0)
    }
    #[doc = "S/W standby returns by VBATT monitor interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbattwupen::_1)
    }
}
#[doc = "ACMPLP0 interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmplp0wupen {
    #[doc = "0: S/W standby returns by ACMPLP0 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by ACMPLP0 interrupt is enabled"]
    _1 = 1,
}
impl From<Acmplp0wupen> for bool {
    #[inline(always)]
    fn from(variant: Acmplp0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMPLP0WUPEN` reader - ACMPLP0 interrupt S/W standby returns enable"]
pub type Acmplp0wupenR = crate::BitReader<Acmplp0wupen>;
impl Acmplp0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmplp0wupen {
        match self.bits {
            false => Acmplp0wupen::_0,
            true => Acmplp0wupen::_1,
        }
    }
    #[doc = "S/W standby returns by ACMPLP0 interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acmplp0wupen::_0
    }
    #[doc = "S/W standby returns by ACMPLP0 interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acmplp0wupen::_1
    }
}
#[doc = "Field `ACMPLP0WUPEN` writer - ACMPLP0 interrupt S/W standby returns enable"]
pub type Acmplp0wupenW<'a, REG> = crate::BitWriter<'a, REG, Acmplp0wupen>;
impl<'a, REG> Acmplp0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by ACMPLP0 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmplp0wupen::_0)
    }
    #[doc = "S/W standby returns by ACMPLP0 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acmplp0wupen::_1)
    }
}
#[doc = "RTC alarm interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcalmwupen {
    #[doc = "0: S/W standby returns by RTC alarm interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by RTC alarm interrupt is enabled"]
    _1 = 1,
}
impl From<Rtcalmwupen> for bool {
    #[inline(always)]
    fn from(variant: Rtcalmwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCALMWUPEN` reader - RTC alarm interrupt S/W standby returns enable"]
pub type RtcalmwupenR = crate::BitReader<Rtcalmwupen>;
impl RtcalmwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcalmwupen {
        match self.bits {
            false => Rtcalmwupen::_0,
            true => Rtcalmwupen::_1,
        }
    }
    #[doc = "S/W standby returns by RTC alarm interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcalmwupen::_0
    }
    #[doc = "S/W standby returns by RTC alarm interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcalmwupen::_1
    }
}
#[doc = "Field `RTCALMWUPEN` writer - RTC alarm interrupt S/W standby returns enable"]
pub type RtcalmwupenW<'a, REG> = crate::BitWriter<'a, REG, Rtcalmwupen>;
impl<'a, REG> RtcalmwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by RTC alarm interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcalmwupen::_0)
    }
    #[doc = "S/W standby returns by RTC alarm interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcalmwupen::_1)
    }
}
#[doc = "RCT period interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcprdwupen {
    #[doc = "0: S/W standby returns by RTC period interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by RTC period interrupt is enabled"]
    _1 = 1,
}
impl From<Rtcprdwupen> for bool {
    #[inline(always)]
    fn from(variant: Rtcprdwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCPRDWUPEN` reader - RCT period interrupt S/W standby returns enable"]
pub type RtcprdwupenR = crate::BitReader<Rtcprdwupen>;
impl RtcprdwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcprdwupen {
        match self.bits {
            false => Rtcprdwupen::_0,
            true => Rtcprdwupen::_1,
        }
    }
    #[doc = "S/W standby returns by RTC period interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcprdwupen::_0
    }
    #[doc = "S/W standby returns by RTC period interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcprdwupen::_1
    }
}
#[doc = "Field `RTCPRDWUPEN` writer - RCT period interrupt S/W standby returns enable"]
pub type RtcprdwupenW<'a, REG> = crate::BitWriter<'a, REG, Rtcprdwupen>;
impl<'a, REG> RtcprdwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by RTC period interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcprdwupen::_0)
    }
    #[doc = "S/W standby returns by RTC period interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcprdwupen::_1)
    }
}
#[doc = "USBFS interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbfswupen {
    #[doc = "0: S/W standby returns by USBFS interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by USBFS interrupt is enabled"]
    _1 = 1,
}
impl From<Usbfswupen> for bool {
    #[inline(always)]
    fn from(variant: Usbfswupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSWUPEN` reader - USBFS interrupt S/W standby returns enable"]
pub type UsbfswupenR = crate::BitReader<Usbfswupen>;
impl UsbfswupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfswupen {
        match self.bits {
            false => Usbfswupen::_0,
            true => Usbfswupen::_1,
        }
    }
    #[doc = "S/W standby returns by USBFS interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbfswupen::_0
    }
    #[doc = "S/W standby returns by USBFS interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbfswupen::_1
    }
}
#[doc = "Field `USBFSWUPEN` writer - USBFS interrupt S/W standby returns enable"]
pub type UsbfswupenW<'a, REG> = crate::BitWriter<'a, REG, Usbfswupen>;
impl<'a, REG> UsbfswupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by USBFS interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfswupen::_0)
    }
    #[doc = "S/W standby returns by USBFS interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfswupen::_1)
    }
}
#[doc = "AGT1 underflow interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1udwupen {
    #[doc = "0: S/W standby returns by AGT1 underflow interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 underflow interrupt is enabled"]
    _1 = 1,
}
impl From<Agt1udwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1udwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGT1UDWUPEN` reader - AGT1 underflow interrupt S/W standby returns enable"]
pub type Agt1udwupenR = crate::BitReader<Agt1udwupen>;
impl Agt1udwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agt1udwupen {
        match self.bits {
            false => Agt1udwupen::_0,
            true => Agt1udwupen::_1,
        }
    }
    #[doc = "S/W standby returns by AGT1 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1udwupen::_0
    }
    #[doc = "S/W standby returns by AGT1 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1udwupen::_1
    }
}
#[doc = "Field `AGT1UDWUPEN` writer - AGT1 underflow interrupt S/W standby returns enable"]
pub type Agt1udwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1udwupen>;
impl<'a, REG> Agt1udwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by AGT1 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1udwupen::_0)
    }
    #[doc = "S/W standby returns by AGT1 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1udwupen::_1)
    }
}
#[doc = "AGT1 compare match A interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1cawupen {
    #[doc = "0: S/W standby returns by AGT1 compare match A interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 compare match A interrupt is enabled"]
    _1 = 1,
}
impl From<Agt1cawupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1cawupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGT1CAWUPEN` reader - AGT1 compare match A interrupt S/W standby returns enable"]
pub type Agt1cawupenR = crate::BitReader<Agt1cawupen>;
impl Agt1cawupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agt1cawupen {
        match self.bits {
            false => Agt1cawupen::_0,
            true => Agt1cawupen::_1,
        }
    }
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1cawupen::_0
    }
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1cawupen::_1
    }
}
#[doc = "Field `AGT1CAWUPEN` writer - AGT1 compare match A interrupt S/W standby returns enable"]
pub type Agt1cawupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1cawupen>;
impl<'a, REG> Agt1cawupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cawupen::_0)
    }
    #[doc = "S/W standby returns by AGT1 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cawupen::_1)
    }
}
#[doc = "AGT1 compare match B interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1cbwupen {
    #[doc = "0: S/W standby returns by AGT1 compare match B interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by AGT1 compare match B interrupt is enabled"]
    _1 = 1,
}
impl From<Agt1cbwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1cbwupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGT1CBWUPEN` reader - AGT1 compare match B interrupt S/W standby returns enable"]
pub type Agt1cbwupenR = crate::BitReader<Agt1cbwupen>;
impl Agt1cbwupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agt1cbwupen {
        match self.bits {
            false => Agt1cbwupen::_0,
            true => Agt1cbwupen::_1,
        }
    }
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1cbwupen::_0
    }
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1cbwupen::_1
    }
}
#[doc = "Field `AGT1CBWUPEN` writer - AGT1 compare match B interrupt S/W standby returns enable"]
pub type Agt1cbwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1cbwupen>;
impl<'a, REG> Agt1cbwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cbwupen::_0)
    }
    #[doc = "S/W standby returns by AGT1 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cbwupen::_1)
    }
}
#[doc = "IIC0 address match interrupt S/W standby returns enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iic0wupen {
    #[doc = "0: S/W standby returns by IIC0 address match interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by IIC0 address match interrupt is enabled"]
    _1 = 1,
}
impl From<Iic0wupen> for bool {
    #[inline(always)]
    fn from(variant: Iic0wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIC0WUPEN` reader - IIC0 address match interrupt S/W standby returns enable"]
pub type Iic0wupenR = crate::BitReader<Iic0wupen>;
impl Iic0wupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iic0wupen {
        match self.bits {
            false => Iic0wupen::_0,
            true => Iic0wupen::_1,
        }
    }
    #[doc = "S/W standby returns by IIC0 address match interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iic0wupen::_0
    }
    #[doc = "S/W standby returns by IIC0 address match interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iic0wupen::_1
    }
}
#[doc = "Field `IIC0WUPEN` writer - IIC0 address match interrupt S/W standby returns enable"]
pub type Iic0wupenW<'a, REG> = crate::BitWriter<'a, REG, Iic0wupen>;
impl<'a, REG> Iic0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S/W standby returns by IIC0 address match interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iic0wupen::_0)
    }
    #[doc = "S/W standby returns by IIC0 address match interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iic0wupen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen0(&self) -> Irqwupen0R {
        Irqwupen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen1(&self) -> Irqwupen1R {
        Irqwupen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen2(&self) -> Irqwupen2R {
        Irqwupen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ3 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen3(&self) -> Irqwupen3R {
        Irqwupen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ4 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen4(&self) -> Irqwupen4R {
        Irqwupen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ5 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen5(&self) -> Irqwupen5R {
        Irqwupen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ6 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen6(&self) -> Irqwupen6R {
        Irqwupen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ7 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen7(&self) -> Irqwupen7R {
        Irqwupen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRQ8 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen8(&self) -> Irqwupen8R {
        Irqwupen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQ9 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen9(&self) -> Irqwupen9R {
        Irqwupen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRQ10 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen10(&self) -> Irqwupen10R {
        Irqwupen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IRQ11 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen11(&self) -> Irqwupen11R {
        Irqwupen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IRQ12 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen12(&self) -> Irqwupen12R {
        Irqwupen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ14 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen14(&self) -> Irqwupen14R {
        Irqwupen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ15 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen15(&self) -> Irqwupen15R {
        Irqwupen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IWDT interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IwdtwupenR {
        IwdtwupenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn keywupen(&self) -> KeywupenR {
        KeywupenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LVD1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd1wupen(&self) -> Lvd1wupenR {
        Lvd1wupenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LVD2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd2wupen(&self) -> Lvd2wupenR {
        Lvd2wupenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VBATT monitor interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn vbattwupen(&self) -> VbattwupenR {
        VbattwupenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - ACMPLP0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn acmplp0wupen(&self) -> Acmplp0wupenR {
        Acmplp0wupenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC alarm interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcalmwupen(&self) -> RtcalmwupenR {
        RtcalmwupenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCT period interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcprdwupen(&self) -> RtcprdwupenR {
        RtcprdwupenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USBFS interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn usbfswupen(&self) -> UsbfswupenR {
        UsbfswupenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AGT1 underflow interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1udwupen(&self) -> Agt1udwupenR {
        Agt1udwupenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AGT1 compare match A interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cawupen(&self) -> Agt1cawupenR {
        Agt1cawupenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AGT1 compare match B interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> Agt1cbwupenR {
        Agt1cbwupenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IIC0 address match interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iic0wupen(&self) -> Iic0wupenR {
        Iic0wupenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUPEN")
            .field("iic0wupen", &self.iic0wupen())
            .field("agt1cbwupen", &self.agt1cbwupen())
            .field("agt1cawupen", &self.agt1cawupen())
            .field("agt1udwupen", &self.agt1udwupen())
            .field("usbfswupen", &self.usbfswupen())
            .field("rtcprdwupen", &self.rtcprdwupen())
            .field("rtcalmwupen", &self.rtcalmwupen())
            .field("acmplp0wupen", &self.acmplp0wupen())
            .field("vbattwupen", &self.vbattwupen())
            .field("lvd2wupen", &self.lvd2wupen())
            .field("lvd1wupen", &self.lvd1wupen())
            .field("keywupen", &self.keywupen())
            .field("iwdtwupen", &self.iwdtwupen())
            .field("irqwupen15", &self.irqwupen15())
            .field("irqwupen14", &self.irqwupen14())
            .field("irqwupen12", &self.irqwupen12())
            .field("irqwupen11", &self.irqwupen11())
            .field("irqwupen10", &self.irqwupen10())
            .field("irqwupen9", &self.irqwupen9())
            .field("irqwupen8", &self.irqwupen8())
            .field("irqwupen7", &self.irqwupen7())
            .field("irqwupen6", &self.irqwupen6())
            .field("irqwupen5", &self.irqwupen5())
            .field("irqwupen4", &self.irqwupen4())
            .field("irqwupen3", &self.irqwupen3())
            .field("irqwupen2", &self.irqwupen2())
            .field("irqwupen1", &self.irqwupen1())
            .field("irqwupen0", &self.irqwupen0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IRQ0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen0(&mut self) -> Irqwupen0W<WupenSpec> {
        Irqwupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - IRQ1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen1(&mut self) -> Irqwupen1W<WupenSpec> {
        Irqwupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - IRQ2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen2(&mut self) -> Irqwupen2W<WupenSpec> {
        Irqwupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - IRQ3 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen3(&mut self) -> Irqwupen3W<WupenSpec> {
        Irqwupen3W::new(self, 3)
    }
    #[doc = "Bit 4 - IRQ4 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen4(&mut self) -> Irqwupen4W<WupenSpec> {
        Irqwupen4W::new(self, 4)
    }
    #[doc = "Bit 5 - IRQ5 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen5(&mut self) -> Irqwupen5W<WupenSpec> {
        Irqwupen5W::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ6 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen6(&mut self) -> Irqwupen6W<WupenSpec> {
        Irqwupen6W::new(self, 6)
    }
    #[doc = "Bit 7 - IRQ7 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen7(&mut self) -> Irqwupen7W<WupenSpec> {
        Irqwupen7W::new(self, 7)
    }
    #[doc = "Bit 8 - IRQ8 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen8(&mut self) -> Irqwupen8W<WupenSpec> {
        Irqwupen8W::new(self, 8)
    }
    #[doc = "Bit 9 - IRQ9 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen9(&mut self) -> Irqwupen9W<WupenSpec> {
        Irqwupen9W::new(self, 9)
    }
    #[doc = "Bit 10 - IRQ10 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen10(&mut self) -> Irqwupen10W<WupenSpec> {
        Irqwupen10W::new(self, 10)
    }
    #[doc = "Bit 11 - IRQ11 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen11(&mut self) -> Irqwupen11W<WupenSpec> {
        Irqwupen11W::new(self, 11)
    }
    #[doc = "Bit 12 - IRQ12 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen12(&mut self) -> Irqwupen12W<WupenSpec> {
        Irqwupen12W::new(self, 12)
    }
    #[doc = "Bit 14 - IRQ14 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen14(&mut self) -> Irqwupen14W<WupenSpec> {
        Irqwupen14W::new(self, 14)
    }
    #[doc = "Bit 15 - IRQ15 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn irqwupen15(&mut self) -> Irqwupen15W<WupenSpec> {
        Irqwupen15W::new(self, 15)
    }
    #[doc = "Bit 16 - IWDT interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iwdtwupen(&mut self) -> IwdtwupenW<WupenSpec> {
        IwdtwupenW::new(self, 16)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn keywupen(&mut self) -> KeywupenW<WupenSpec> {
        KeywupenW::new(self, 17)
    }
    #[doc = "Bit 18 - LVD1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd1wupen(&mut self) -> Lvd1wupenW<WupenSpec> {
        Lvd1wupenW::new(self, 18)
    }
    #[doc = "Bit 19 - LVD2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn lvd2wupen(&mut self) -> Lvd2wupenW<WupenSpec> {
        Lvd2wupenW::new(self, 19)
    }
    #[doc = "Bit 20 - VBATT monitor interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn vbattwupen(&mut self) -> VbattwupenW<WupenSpec> {
        VbattwupenW::new(self, 20)
    }
    #[doc = "Bit 23 - ACMPLP0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn acmplp0wupen(&mut self) -> Acmplp0wupenW<WupenSpec> {
        Acmplp0wupenW::new(self, 23)
    }
    #[doc = "Bit 24 - RTC alarm interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcalmwupen(&mut self) -> RtcalmwupenW<WupenSpec> {
        RtcalmwupenW::new(self, 24)
    }
    #[doc = "Bit 25 - RCT period interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn rtcprdwupen(&mut self) -> RtcprdwupenW<WupenSpec> {
        RtcprdwupenW::new(self, 25)
    }
    #[doc = "Bit 27 - USBFS interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn usbfswupen(&mut self) -> UsbfswupenW<WupenSpec> {
        UsbfswupenW::new(self, 27)
    }
    #[doc = "Bit 28 - AGT1 underflow interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1udwupen(&mut self) -> Agt1udwupenW<WupenSpec> {
        Agt1udwupenW::new(self, 28)
    }
    #[doc = "Bit 29 - AGT1 compare match A interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cawupen(&mut self) -> Agt1cawupenW<WupenSpec> {
        Agt1cawupenW::new(self, 29)
    }
    #[doc = "Bit 30 - AGT1 compare match B interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn agt1cbwupen(&mut self) -> Agt1cbwupenW<WupenSpec> {
        Agt1cbwupenW::new(self, 30)
    }
    #[doc = "Bit 31 - IIC0 address match interrupt S/W standby returns enable"]
    #[inline(always)]
    pub fn iic0wupen(&mut self) -> Iic0wupenW<WupenSpec> {
        Iic0wupenW::new(self, 31)
    }
}
#[doc = "Wake Up Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WupenSpec;
impl crate::RegisterSpec for WupenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupen::R`](R) reader structure"]
impl crate::Readable for WupenSpec {}
#[doc = "`write(|w| ..)` method takes [`wupen::W`](W) writer structure"]
impl crate::Writable for WupenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUPEN to value 0"]
impl crate::Resettable for WupenSpec {}

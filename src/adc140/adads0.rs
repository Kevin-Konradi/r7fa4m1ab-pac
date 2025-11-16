#[doc = "Register `ADADS0` reader"]
pub type R = crate::R<Adads0Spec>;
#[doc = "Register `ADADS0` writer"]
pub type W = crate::W<Adads0Spec>;
#[doc = "A/D-Converted Value Addition/Average Channel AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads00 {
    #[doc = "0: AN000 is not selected."]
    _0 = 0,
    #[doc = "1: AN000 is selected."]
    _1 = 1,
}
impl From<Ads00> for bool {
    #[inline(always)]
    fn from(variant: Ads00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS00` reader - A/D-Converted Value Addition/Average Channel AN000 Select"]
pub type Ads00R = crate::BitReader<Ads00>;
impl Ads00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads00 {
        match self.bits {
            false => Ads00::_0,
            true => Ads00::_1,
        }
    }
    #[doc = "AN000 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads00::_0
    }
    #[doc = "AN000 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads00::_1
    }
}
#[doc = "Field `ADS00` writer - A/D-Converted Value Addition/Average Channel AN000 Select"]
pub type Ads00W<'a, REG> = crate::BitWriter<'a, REG, Ads00>;
impl<'a, REG> Ads00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN000 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads00::_0)
    }
    #[doc = "AN000 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads00::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads01 {
    #[doc = "0: AN001 is not selected."]
    _0 = 0,
    #[doc = "1: AN001 is selected."]
    _1 = 1,
}
impl From<Ads01> for bool {
    #[inline(always)]
    fn from(variant: Ads01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS01` reader - A/D-Converted Value Addition/Average Channel AN001 Select"]
pub type Ads01R = crate::BitReader<Ads01>;
impl Ads01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads01 {
        match self.bits {
            false => Ads01::_0,
            true => Ads01::_1,
        }
    }
    #[doc = "AN001 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads01::_0
    }
    #[doc = "AN001 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads01::_1
    }
}
#[doc = "Field `ADS01` writer - A/D-Converted Value Addition/Average Channel AN001 Select"]
pub type Ads01W<'a, REG> = crate::BitWriter<'a, REG, Ads01>;
impl<'a, REG> Ads01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN001 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads01::_0)
    }
    #[doc = "AN001 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads01::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads02 {
    #[doc = "0: AN002 is not selected."]
    _0 = 0,
    #[doc = "1: AN002 is selected."]
    _1 = 1,
}
impl From<Ads02> for bool {
    #[inline(always)]
    fn from(variant: Ads02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS02` reader - A/D-Converted Value Addition/Average Channel AN002 Select"]
pub type Ads02R = crate::BitReader<Ads02>;
impl Ads02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads02 {
        match self.bits {
            false => Ads02::_0,
            true => Ads02::_1,
        }
    }
    #[doc = "AN002 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads02::_0
    }
    #[doc = "AN002 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads02::_1
    }
}
#[doc = "Field `ADS02` writer - A/D-Converted Value Addition/Average Channel AN002 Select"]
pub type Ads02W<'a, REG> = crate::BitWriter<'a, REG, Ads02>;
impl<'a, REG> Ads02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN002 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads02::_0)
    }
    #[doc = "AN002 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads02::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads03 {
    #[doc = "0: AN003 is not selected."]
    _0 = 0,
    #[doc = "1: AN003 is selected."]
    _1 = 1,
}
impl From<Ads03> for bool {
    #[inline(always)]
    fn from(variant: Ads03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS03` reader - A/D-Converted Value Addition/Average Channel AN003 Select"]
pub type Ads03R = crate::BitReader<Ads03>;
impl Ads03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads03 {
        match self.bits {
            false => Ads03::_0,
            true => Ads03::_1,
        }
    }
    #[doc = "AN003 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads03::_0
    }
    #[doc = "AN003 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads03::_1
    }
}
#[doc = "Field `ADS03` writer - A/D-Converted Value Addition/Average Channel AN003 Select"]
pub type Ads03W<'a, REG> = crate::BitWriter<'a, REG, Ads03>;
impl<'a, REG> Ads03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN003 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads03::_0)
    }
    #[doc = "AN003 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads03::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads04 {
    #[doc = "0: AN004 is not selected."]
    _0 = 0,
    #[doc = "1: AN004 is selected."]
    _1 = 1,
}
impl From<Ads04> for bool {
    #[inline(always)]
    fn from(variant: Ads04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS04` reader - A/D-Converted Value Addition/Average Channel AN004 Select"]
pub type Ads04R = crate::BitReader<Ads04>;
impl Ads04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads04 {
        match self.bits {
            false => Ads04::_0,
            true => Ads04::_1,
        }
    }
    #[doc = "AN004 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads04::_0
    }
    #[doc = "AN004 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads04::_1
    }
}
#[doc = "Field `ADS04` writer - A/D-Converted Value Addition/Average Channel AN004 Select"]
pub type Ads04W<'a, REG> = crate::BitWriter<'a, REG, Ads04>;
impl<'a, REG> Ads04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN004 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads04::_0)
    }
    #[doc = "AN004 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads04::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads05 {
    #[doc = "0: AN005 is not selected."]
    _0 = 0,
    #[doc = "1: AN005 is selected."]
    _1 = 1,
}
impl From<Ads05> for bool {
    #[inline(always)]
    fn from(variant: Ads05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS05` reader - A/D-Converted Value Addition/Average Channel AN005 Select"]
pub type Ads05R = crate::BitReader<Ads05>;
impl Ads05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads05 {
        match self.bits {
            false => Ads05::_0,
            true => Ads05::_1,
        }
    }
    #[doc = "AN005 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads05::_0
    }
    #[doc = "AN005 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads05::_1
    }
}
#[doc = "Field `ADS05` writer - A/D-Converted Value Addition/Average Channel AN005 Select"]
pub type Ads05W<'a, REG> = crate::BitWriter<'a, REG, Ads05>;
impl<'a, REG> Ads05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN005 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads05::_0)
    }
    #[doc = "AN005 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads05::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads06 {
    #[doc = "0: AN006 is not selected."]
    _0 = 0,
    #[doc = "1: AN006 is selected."]
    _1 = 1,
}
impl From<Ads06> for bool {
    #[inline(always)]
    fn from(variant: Ads06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS06` reader - A/D-Converted Value Addition/Average Channel AN006 Select"]
pub type Ads06R = crate::BitReader<Ads06>;
impl Ads06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads06 {
        match self.bits {
            false => Ads06::_0,
            true => Ads06::_1,
        }
    }
    #[doc = "AN006 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads06::_0
    }
    #[doc = "AN006 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads06::_1
    }
}
#[doc = "Field `ADS06` writer - A/D-Converted Value Addition/Average Channel AN006 Select"]
pub type Ads06W<'a, REG> = crate::BitWriter<'a, REG, Ads06>;
impl<'a, REG> Ads06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN006 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads06::_0)
    }
    #[doc = "AN006 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads06::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads07 {
    #[doc = "0: AN007 is not selected."]
    _0 = 0,
    #[doc = "1: AN007 is selected."]
    _1 = 1,
}
impl From<Ads07> for bool {
    #[inline(always)]
    fn from(variant: Ads07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS07` reader - A/D-Converted Value Addition/Average Channel AN007 Select"]
pub type Ads07R = crate::BitReader<Ads07>;
impl Ads07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads07 {
        match self.bits {
            false => Ads07::_0,
            true => Ads07::_1,
        }
    }
    #[doc = "AN007 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads07::_0
    }
    #[doc = "AN007 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads07::_1
    }
}
#[doc = "Field `ADS07` writer - A/D-Converted Value Addition/Average Channel AN007 Select"]
pub type Ads07W<'a, REG> = crate::BitWriter<'a, REG, Ads07>;
impl<'a, REG> Ads07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN007 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads07::_0)
    }
    #[doc = "AN007 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads07::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads08 {
    #[doc = "0: AN008 is not selected."]
    _0 = 0,
    #[doc = "1: AN008 is selected."]
    _1 = 1,
}
impl From<Ads08> for bool {
    #[inline(always)]
    fn from(variant: Ads08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS08` reader - A/D-Converted Value Addition/Average Channel AN008 Select"]
pub type Ads08R = crate::BitReader<Ads08>;
impl Ads08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads08 {
        match self.bits {
            false => Ads08::_0,
            true => Ads08::_1,
        }
    }
    #[doc = "AN008 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads08::_0
    }
    #[doc = "AN008 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads08::_1
    }
}
#[doc = "Field `ADS08` writer - A/D-Converted Value Addition/Average Channel AN008 Select"]
pub type Ads08W<'a, REG> = crate::BitWriter<'a, REG, Ads08>;
impl<'a, REG> Ads08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN008 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads08::_0)
    }
    #[doc = "AN008 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads08::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads09 {
    #[doc = "0: AN009 is not selected."]
    _0 = 0,
    #[doc = "1: AN009 is selected."]
    _1 = 1,
}
impl From<Ads09> for bool {
    #[inline(always)]
    fn from(variant: Ads09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS09` reader - A/D-Converted Value Addition/Average Channel AN009 Select"]
pub type Ads09R = crate::BitReader<Ads09>;
impl Ads09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads09 {
        match self.bits {
            false => Ads09::_0,
            true => Ads09::_1,
        }
    }
    #[doc = "AN009 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads09::_0
    }
    #[doc = "AN009 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads09::_1
    }
}
#[doc = "Field `ADS09` writer - A/D-Converted Value Addition/Average Channel AN009 Select"]
pub type Ads09W<'a, REG> = crate::BitWriter<'a, REG, Ads09>;
impl<'a, REG> Ads09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN009 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads09::_0)
    }
    #[doc = "AN009 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads09::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads10 {
    #[doc = "0: AN010 is not selected."]
    _0 = 0,
    #[doc = "1: AN010 is selected."]
    _1 = 1,
}
impl From<Ads10> for bool {
    #[inline(always)]
    fn from(variant: Ads10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS10` reader - A/D-Converted Value Addition/Average Channel AN010 Select"]
pub type Ads10R = crate::BitReader<Ads10>;
impl Ads10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads10 {
        match self.bits {
            false => Ads10::_0,
            true => Ads10::_1,
        }
    }
    #[doc = "AN010 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads10::_0
    }
    #[doc = "AN010 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads10::_1
    }
}
#[doc = "Field `ADS10` writer - A/D-Converted Value Addition/Average Channel AN010 Select"]
pub type Ads10W<'a, REG> = crate::BitWriter<'a, REG, Ads10>;
impl<'a, REG> Ads10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN010 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads10::_0)
    }
    #[doc = "AN010 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads10::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads11 {
    #[doc = "0: AN011 is not selected."]
    _0 = 0,
    #[doc = "1: AN011 is selected."]
    _1 = 1,
}
impl From<Ads11> for bool {
    #[inline(always)]
    fn from(variant: Ads11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS11` reader - A/D-Converted Value Addition/Average Channel AN011 Select"]
pub type Ads11R = crate::BitReader<Ads11>;
impl Ads11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads11 {
        match self.bits {
            false => Ads11::_0,
            true => Ads11::_1,
        }
    }
    #[doc = "AN011 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads11::_0
    }
    #[doc = "AN011 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads11::_1
    }
}
#[doc = "Field `ADS11` writer - A/D-Converted Value Addition/Average Channel AN011 Select"]
pub type Ads11W<'a, REG> = crate::BitWriter<'a, REG, Ads11>;
impl<'a, REG> Ads11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN011 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads11::_0)
    }
    #[doc = "AN011 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads11::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads12 {
    #[doc = "0: AN012 is not selected."]
    _0 = 0,
    #[doc = "1: AN012 is selected."]
    _1 = 1,
}
impl From<Ads12> for bool {
    #[inline(always)]
    fn from(variant: Ads12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS12` reader - A/D-Converted Value Addition/Average Channel AN012 Select"]
pub type Ads12R = crate::BitReader<Ads12>;
impl Ads12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads12 {
        match self.bits {
            false => Ads12::_0,
            true => Ads12::_1,
        }
    }
    #[doc = "AN012 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads12::_0
    }
    #[doc = "AN012 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads12::_1
    }
}
#[doc = "Field `ADS12` writer - A/D-Converted Value Addition/Average Channel AN012 Select"]
pub type Ads12W<'a, REG> = crate::BitWriter<'a, REG, Ads12>;
impl<'a, REG> Ads12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN012 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads12::_0)
    }
    #[doc = "AN012 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads12::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads13 {
    #[doc = "0: AN013 is not selected."]
    _0 = 0,
    #[doc = "1: AN013 is selected."]
    _1 = 1,
}
impl From<Ads13> for bool {
    #[inline(always)]
    fn from(variant: Ads13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS13` reader - A/D-Converted Value Addition/Average Channel AN013 Select"]
pub type Ads13R = crate::BitReader<Ads13>;
impl Ads13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads13 {
        match self.bits {
            false => Ads13::_0,
            true => Ads13::_1,
        }
    }
    #[doc = "AN013 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads13::_0
    }
    #[doc = "AN013 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads13::_1
    }
}
#[doc = "Field `ADS13` writer - A/D-Converted Value Addition/Average Channel AN013 Select"]
pub type Ads13W<'a, REG> = crate::BitWriter<'a, REG, Ads13>;
impl<'a, REG> Ads13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN013 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads13::_0)
    }
    #[doc = "AN013 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads13::_1)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads14 {
    #[doc = "0: AN014 is not selected."]
    _0 = 0,
    #[doc = "1: AN014 is selected."]
    _1 = 1,
}
impl From<Ads14> for bool {
    #[inline(always)]
    fn from(variant: Ads14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADS14` reader - A/D-Converted Value Addition/Average Channel AN014 Select"]
pub type Ads14R = crate::BitReader<Ads14>;
impl Ads14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ads14 {
        match self.bits {
            false => Ads14::_0,
            true => Ads14::_1,
        }
    }
    #[doc = "AN014 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads14::_0
    }
    #[doc = "AN014 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads14::_1
    }
}
#[doc = "Field `ADS14` writer - A/D-Converted Value Addition/Average Channel AN014 Select"]
pub type Ads14W<'a, REG> = crate::BitWriter<'a, REG, Ads14>;
impl<'a, REG> Ads14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN014 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads14::_0)
    }
    #[doc = "AN014 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads14::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[inline(always)]
    pub fn ads00(&self) -> Ads00R {
        Ads00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[inline(always)]
    pub fn ads01(&self) -> Ads01R {
        Ads01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[inline(always)]
    pub fn ads02(&self) -> Ads02R {
        Ads02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[inline(always)]
    pub fn ads03(&self) -> Ads03R {
        Ads03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[inline(always)]
    pub fn ads04(&self) -> Ads04R {
        Ads04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[inline(always)]
    pub fn ads05(&self) -> Ads05R {
        Ads05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[inline(always)]
    pub fn ads06(&self) -> Ads06R {
        Ads06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[inline(always)]
    pub fn ads07(&self) -> Ads07R {
        Ads07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel AN008 Select"]
    #[inline(always)]
    pub fn ads08(&self) -> Ads08R {
        Ads08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel AN009 Select"]
    #[inline(always)]
    pub fn ads09(&self) -> Ads09R {
        Ads09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D-Converted Value Addition/Average Channel AN010 Select"]
    #[inline(always)]
    pub fn ads10(&self) -> Ads10R {
        Ads10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel AN011 Select"]
    #[inline(always)]
    pub fn ads11(&self) -> Ads11R {
        Ads11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel AN012 Select"]
    #[inline(always)]
    pub fn ads12(&self) -> Ads12R {
        Ads12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel AN013 Select"]
    #[inline(always)]
    pub fn ads13(&self) -> Ads13R {
        Ads13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A/D-Converted Value Addition/Average Channel AN014 Select"]
    #[inline(always)]
    pub fn ads14(&self) -> Ads14R {
        Ads14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADADS0")
            .field("ads14", &self.ads14())
            .field("ads13", &self.ads13())
            .field("ads12", &self.ads12())
            .field("ads11", &self.ads11())
            .field("ads10", &self.ads10())
            .field("ads09", &self.ads09())
            .field("ads08", &self.ads08())
            .field("ads07", &self.ads07())
            .field("ads06", &self.ads06())
            .field("ads05", &self.ads05())
            .field("ads04", &self.ads04())
            .field("ads03", &self.ads03())
            .field("ads02", &self.ads02())
            .field("ads01", &self.ads01())
            .field("ads00", &self.ads00())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select"]
    #[inline(always)]
    pub fn ads00(&mut self) -> Ads00W<Adads0Spec> {
        Ads00W::new(self, 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select"]
    #[inline(always)]
    pub fn ads01(&mut self) -> Ads01W<Adads0Spec> {
        Ads01W::new(self, 1)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select"]
    #[inline(always)]
    pub fn ads02(&mut self) -> Ads02W<Adads0Spec> {
        Ads02W::new(self, 2)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select"]
    #[inline(always)]
    pub fn ads03(&mut self) -> Ads03W<Adads0Spec> {
        Ads03W::new(self, 3)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select"]
    #[inline(always)]
    pub fn ads04(&mut self) -> Ads04W<Adads0Spec> {
        Ads04W::new(self, 4)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select"]
    #[inline(always)]
    pub fn ads05(&mut self) -> Ads05W<Adads0Spec> {
        Ads05W::new(self, 5)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select"]
    #[inline(always)]
    pub fn ads06(&mut self) -> Ads06W<Adads0Spec> {
        Ads06W::new(self, 6)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select"]
    #[inline(always)]
    pub fn ads07(&mut self) -> Ads07W<Adads0Spec> {
        Ads07W::new(self, 7)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel AN008 Select"]
    #[inline(always)]
    pub fn ads08(&mut self) -> Ads08W<Adads0Spec> {
        Ads08W::new(self, 8)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel AN009 Select"]
    #[inline(always)]
    pub fn ads09(&mut self) -> Ads09W<Adads0Spec> {
        Ads09W::new(self, 9)
    }
    #[doc = "Bit 10 - A/D-Converted Value Addition/Average Channel AN010 Select"]
    #[inline(always)]
    pub fn ads10(&mut self) -> Ads10W<Adads0Spec> {
        Ads10W::new(self, 10)
    }
    #[doc = "Bit 11 - A/D-Converted Value Addition/Average Channel AN011 Select"]
    #[inline(always)]
    pub fn ads11(&mut self) -> Ads11W<Adads0Spec> {
        Ads11W::new(self, 11)
    }
    #[doc = "Bit 12 - A/D-Converted Value Addition/Average Channel AN012 Select"]
    #[inline(always)]
    pub fn ads12(&mut self) -> Ads12W<Adads0Spec> {
        Ads12W::new(self, 12)
    }
    #[doc = "Bit 13 - A/D-Converted Value Addition/Average Channel AN013 Select"]
    #[inline(always)]
    pub fn ads13(&mut self) -> Ads13W<Adads0Spec> {
        Ads13W::new(self, 13)
    }
    #[doc = "Bit 14 - A/D-Converted Value Addition/Average Channel AN014 Select"]
    #[inline(always)]
    pub fn ads14(&mut self) -> Ads14W<Adads0Spec> {
        Ads14W::new(self, 14)
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adads0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adads0Spec;
impl crate::RegisterSpec for Adads0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adads0::R`](R) reader structure"]
impl crate::Readable for Adads0Spec {}
#[doc = "`write(|w| ..)` method takes [`adads0::W`](W) writer structure"]
impl crate::Writable for Adads0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADADS0 to value 0"]
impl crate::Resettable for Adads0Spec {}

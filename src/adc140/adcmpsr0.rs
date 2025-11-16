#[doc = "Register `ADCMPSR0` reader"]
pub type R = crate::R<Adcmpsr0Spec>;
#[doc = "Register `ADCMPSR0` writer"]
pub type W = crate::W<Adcmpsr0Spec>;
#[doc = "Compare window A flag of AN000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha00 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha00> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA00` reader - Compare window A flag of AN000\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha00R = crate::BitReader<Cmpstcha00>;
impl Cmpstcha00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha00 {
        match self.bits {
            false => Cmpstcha00::_0,
            true => Cmpstcha00::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha00::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha00::_1
    }
}
#[doc = "Field `CMPSTCHA00` writer - Compare window A flag of AN000"]
pub type Cmpstcha00W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha00>;
impl<'a, REG> Cmpstcha00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha00::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha00::_1)
    }
}
#[doc = "Compare window A flag of AN001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha01 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha01> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA01` reader - Compare window A flag of AN001\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha01R = crate::BitReader<Cmpstcha01>;
impl Cmpstcha01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha01 {
        match self.bits {
            false => Cmpstcha01::_0,
            true => Cmpstcha01::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha01::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha01::_1
    }
}
#[doc = "Field `CMPSTCHA01` writer - Compare window A flag of AN001"]
pub type Cmpstcha01W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha01>;
impl<'a, REG> Cmpstcha01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha01::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha01::_1)
    }
}
#[doc = "Compare window A flag of AN002\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha02 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha02> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA02` reader - Compare window A flag of AN002\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha02R = crate::BitReader<Cmpstcha02>;
impl Cmpstcha02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha02 {
        match self.bits {
            false => Cmpstcha02::_0,
            true => Cmpstcha02::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha02::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha02::_1
    }
}
#[doc = "Field `CMPSTCHA02` writer - Compare window A flag of AN002"]
pub type Cmpstcha02W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha02>;
impl<'a, REG> Cmpstcha02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha02::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha02::_1)
    }
}
#[doc = "Compare window A flag of AN003\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha03 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha03> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA03` reader - Compare window A flag of AN003\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha03R = crate::BitReader<Cmpstcha03>;
impl Cmpstcha03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha03 {
        match self.bits {
            false => Cmpstcha03::_0,
            true => Cmpstcha03::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha03::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha03::_1
    }
}
#[doc = "Field `CMPSTCHA03` writer - Compare window A flag of AN003"]
pub type Cmpstcha03W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha03>;
impl<'a, REG> Cmpstcha03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha03::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha03::_1)
    }
}
#[doc = "Compare window A flag of AN004\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha04 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha04> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA04` reader - Compare window A flag of AN004\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha04R = crate::BitReader<Cmpstcha04>;
impl Cmpstcha04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha04 {
        match self.bits {
            false => Cmpstcha04::_0,
            true => Cmpstcha04::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha04::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha04::_1
    }
}
#[doc = "Field `CMPSTCHA04` writer - Compare window A flag of AN004"]
pub type Cmpstcha04W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha04>;
impl<'a, REG> Cmpstcha04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha04::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha04::_1)
    }
}
#[doc = "Compare window A flag of AN005\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha05 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha05> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA05` reader - Compare window A flag of AN005\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha05R = crate::BitReader<Cmpstcha05>;
impl Cmpstcha05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha05 {
        match self.bits {
            false => Cmpstcha05::_0,
            true => Cmpstcha05::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha05::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha05::_1
    }
}
#[doc = "Field `CMPSTCHA05` writer - Compare window A flag of AN005"]
pub type Cmpstcha05W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha05>;
impl<'a, REG> Cmpstcha05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha05::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha05::_1)
    }
}
#[doc = "Compare window A flag of AN006\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha06 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha06> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA06` reader - Compare window A flag of AN006\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha06R = crate::BitReader<Cmpstcha06>;
impl Cmpstcha06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha06 {
        match self.bits {
            false => Cmpstcha06::_0,
            true => Cmpstcha06::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha06::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha06::_1
    }
}
#[doc = "Field `CMPSTCHA06` writer - Compare window A flag of AN006"]
pub type Cmpstcha06W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha06>;
impl<'a, REG> Cmpstcha06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha06::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha06::_1)
    }
}
#[doc = "Compare window A flag of AN007\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha07 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha07> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA07` reader - Compare window A flag of AN007\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha07R = crate::BitReader<Cmpstcha07>;
impl Cmpstcha07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha07 {
        match self.bits {
            false => Cmpstcha07::_0,
            true => Cmpstcha07::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha07::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha07::_1
    }
}
#[doc = "Field `CMPSTCHA07` writer - Compare window A flag of AN007"]
pub type Cmpstcha07W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha07>;
impl<'a, REG> Cmpstcha07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha07::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha07::_1)
    }
}
#[doc = "Compare window A flag of AN008\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha08 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha08> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA08` reader - Compare window A flag of AN008\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha08R = crate::BitReader<Cmpstcha08>;
impl Cmpstcha08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha08 {
        match self.bits {
            false => Cmpstcha08::_0,
            true => Cmpstcha08::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha08::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha08::_1
    }
}
#[doc = "Field `CMPSTCHA08` writer - Compare window A flag of AN008"]
pub type Cmpstcha08W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha08>;
impl<'a, REG> Cmpstcha08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha08::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha08::_1)
    }
}
#[doc = "Compare window A flag of AN009\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha09 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha09> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA09` reader - Compare window A flag of AN009\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha09R = crate::BitReader<Cmpstcha09>;
impl Cmpstcha09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha09 {
        match self.bits {
            false => Cmpstcha09::_0,
            true => Cmpstcha09::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha09::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha09::_1
    }
}
#[doc = "Field `CMPSTCHA09` writer - Compare window A flag of AN009"]
pub type Cmpstcha09W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha09>;
impl<'a, REG> Cmpstcha09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha09::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha09::_1)
    }
}
#[doc = "Compare window A flag of AN010\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha10 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha10> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA10` reader - Compare window A flag of AN010\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha10R = crate::BitReader<Cmpstcha10>;
impl Cmpstcha10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha10 {
        match self.bits {
            false => Cmpstcha10::_0,
            true => Cmpstcha10::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha10::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha10::_1
    }
}
#[doc = "Field `CMPSTCHA10` writer - Compare window A flag of AN010"]
pub type Cmpstcha10W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha10>;
impl<'a, REG> Cmpstcha10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha10::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha10::_1)
    }
}
#[doc = "Compare window A flag of AN011\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha11 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha11> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA11` reader - Compare window A flag of AN011\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha11R = crate::BitReader<Cmpstcha11>;
impl Cmpstcha11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha11 {
        match self.bits {
            false => Cmpstcha11::_0,
            true => Cmpstcha11::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha11::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha11::_1
    }
}
#[doc = "Field `CMPSTCHA11` writer - Compare window A flag of AN011"]
pub type Cmpstcha11W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha11>;
impl<'a, REG> Cmpstcha11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha11::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha11::_1)
    }
}
#[doc = "Compare window A flag of AN012\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha12 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha12> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA12` reader - Compare window A flag of AN012\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha12R = crate::BitReader<Cmpstcha12>;
impl Cmpstcha12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha12 {
        match self.bits {
            false => Cmpstcha12::_0,
            true => Cmpstcha12::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha12::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha12::_1
    }
}
#[doc = "Field `CMPSTCHA12` writer - Compare window A flag of AN012"]
pub type Cmpstcha12W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha12>;
impl<'a, REG> Cmpstcha12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha12::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha12::_1)
    }
}
#[doc = "Compare window A flag of AN013\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha13 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha13> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA13` reader - Compare window A flag of AN013\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha13R = crate::BitReader<Cmpstcha13>;
impl Cmpstcha13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha13 {
        match self.bits {
            false => Cmpstcha13::_0,
            true => Cmpstcha13::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha13::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha13::_1
    }
}
#[doc = "Field `CMPSTCHA13` writer - Compare window A flag of AN013"]
pub type Cmpstcha13W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha13>;
impl<'a, REG> Cmpstcha13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha13::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha13::_1)
    }
}
#[doc = "Compare window A flag of AN014\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha14 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha14> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA14` reader - Compare window A flag of AN014\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha14R = crate::BitReader<Cmpstcha14>;
impl Cmpstcha14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha14 {
        match self.bits {
            false => Cmpstcha14::_0,
            true => Cmpstcha14::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha14::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha14::_1
    }
}
#[doc = "Field `CMPSTCHA14` writer - Compare window A flag of AN014"]
pub type Cmpstcha14W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha14>;
impl<'a, REG> Cmpstcha14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha14::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha14::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window A flag of AN000"]
    #[inline(always)]
    pub fn cmpstcha00(&self) -> Cmpstcha00R {
        Cmpstcha00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN001"]
    #[inline(always)]
    pub fn cmpstcha01(&self) -> Cmpstcha01R {
        Cmpstcha01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare window A flag of AN002"]
    #[inline(always)]
    pub fn cmpstcha02(&self) -> Cmpstcha02R {
        Cmpstcha02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare window A flag of AN003"]
    #[inline(always)]
    pub fn cmpstcha03(&self) -> Cmpstcha03R {
        Cmpstcha03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare window A flag of AN004"]
    #[inline(always)]
    pub fn cmpstcha04(&self) -> Cmpstcha04R {
        Cmpstcha04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare window A flag of AN005"]
    #[inline(always)]
    pub fn cmpstcha05(&self) -> Cmpstcha05R {
        Cmpstcha05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare window A flag of AN006"]
    #[inline(always)]
    pub fn cmpstcha06(&self) -> Cmpstcha06R {
        Cmpstcha06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare window A flag of AN007"]
    #[inline(always)]
    pub fn cmpstcha07(&self) -> Cmpstcha07R {
        Cmpstcha07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare window A flag of AN008"]
    #[inline(always)]
    pub fn cmpstcha08(&self) -> Cmpstcha08R {
        Cmpstcha08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare window A flag of AN009"]
    #[inline(always)]
    pub fn cmpstcha09(&self) -> Cmpstcha09R {
        Cmpstcha09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare window A flag of AN010"]
    #[inline(always)]
    pub fn cmpstcha10(&self) -> Cmpstcha10R {
        Cmpstcha10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare window A flag of AN011"]
    #[inline(always)]
    pub fn cmpstcha11(&self) -> Cmpstcha11R {
        Cmpstcha11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare window A flag of AN012"]
    #[inline(always)]
    pub fn cmpstcha12(&self) -> Cmpstcha12R {
        Cmpstcha12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare window A flag of AN013"]
    #[inline(always)]
    pub fn cmpstcha13(&self) -> Cmpstcha13R {
        Cmpstcha13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare window A flag of AN014"]
    #[inline(always)]
    pub fn cmpstcha14(&self) -> Cmpstcha14R {
        Cmpstcha14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSR0").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare window A flag of AN000"]
    #[inline(always)]
    pub fn cmpstcha00(&mut self) -> Cmpstcha00W<Adcmpsr0Spec> {
        Cmpstcha00W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN001"]
    #[inline(always)]
    pub fn cmpstcha01(&mut self) -> Cmpstcha01W<Adcmpsr0Spec> {
        Cmpstcha01W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare window A flag of AN002"]
    #[inline(always)]
    pub fn cmpstcha02(&mut self) -> Cmpstcha02W<Adcmpsr0Spec> {
        Cmpstcha02W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare window A flag of AN003"]
    #[inline(always)]
    pub fn cmpstcha03(&mut self) -> Cmpstcha03W<Adcmpsr0Spec> {
        Cmpstcha03W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare window A flag of AN004"]
    #[inline(always)]
    pub fn cmpstcha04(&mut self) -> Cmpstcha04W<Adcmpsr0Spec> {
        Cmpstcha04W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare window A flag of AN005"]
    #[inline(always)]
    pub fn cmpstcha05(&mut self) -> Cmpstcha05W<Adcmpsr0Spec> {
        Cmpstcha05W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare window A flag of AN006"]
    #[inline(always)]
    pub fn cmpstcha06(&mut self) -> Cmpstcha06W<Adcmpsr0Spec> {
        Cmpstcha06W::new(self, 6)
    }
    #[doc = "Bit 7 - Compare window A flag of AN007"]
    #[inline(always)]
    pub fn cmpstcha07(&mut self) -> Cmpstcha07W<Adcmpsr0Spec> {
        Cmpstcha07W::new(self, 7)
    }
    #[doc = "Bit 8 - Compare window A flag of AN008"]
    #[inline(always)]
    pub fn cmpstcha08(&mut self) -> Cmpstcha08W<Adcmpsr0Spec> {
        Cmpstcha08W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare window A flag of AN009"]
    #[inline(always)]
    pub fn cmpstcha09(&mut self) -> Cmpstcha09W<Adcmpsr0Spec> {
        Cmpstcha09W::new(self, 9)
    }
    #[doc = "Bit 10 - Compare window A flag of AN010"]
    #[inline(always)]
    pub fn cmpstcha10(&mut self) -> Cmpstcha10W<Adcmpsr0Spec> {
        Cmpstcha10W::new(self, 10)
    }
    #[doc = "Bit 11 - Compare window A flag of AN011"]
    #[inline(always)]
    pub fn cmpstcha11(&mut self) -> Cmpstcha11W<Adcmpsr0Spec> {
        Cmpstcha11W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare window A flag of AN012"]
    #[inline(always)]
    pub fn cmpstcha12(&mut self) -> Cmpstcha12W<Adcmpsr0Spec> {
        Cmpstcha12W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare window A flag of AN013"]
    #[inline(always)]
    pub fn cmpstcha13(&mut self) -> Cmpstcha13W<Adcmpsr0Spec> {
        Cmpstcha13W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare window A flag of AN014"]
    #[inline(always)]
    pub fn cmpstcha14(&mut self) -> Cmpstcha14W<Adcmpsr0Spec> {
        Cmpstcha14W::new(self, 14)
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmpsr0Spec;
impl crate::RegisterSpec for Adcmpsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpsr0::R`](R) reader structure"]
impl crate::Readable for Adcmpsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmpsr0::W`](W) writer structure"]
impl crate::Writable for Adcmpsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x7fff;
}
#[doc = "`reset()` method sets ADCMPSR0 to value 0"]
impl crate::Resettable for Adcmpsr0Spec {}

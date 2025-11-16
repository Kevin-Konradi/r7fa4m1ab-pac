#[doc = "Register `ADCMPLR0` reader"]
pub type R = crate::R<Adcmplr0Spec>;
#[doc = "Register `ADCMPLR0` writer"]
pub type W = crate::W<Adcmplr0Spec>;
#[doc = "Comparison condition of AN000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha00 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha00> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA00` reader - Comparison condition of AN000"]
pub type Cmplcha00R = crate::BitReader<Cmplcha00>;
impl Cmplcha00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha00 {
        match self.bits {
            false => Cmplcha00::_0,
            true => Cmplcha00::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha00::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha00::_1
    }
}
#[doc = "Field `CMPLCHA00` writer - Comparison condition of AN000"]
pub type Cmplcha00W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha00>;
impl<'a, REG> Cmplcha00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha00::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha00::_1)
    }
}
#[doc = "Comparison condition of AN001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha01 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha01> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA01` reader - Comparison condition of AN001"]
pub type Cmplcha01R = crate::BitReader<Cmplcha01>;
impl Cmplcha01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha01 {
        match self.bits {
            false => Cmplcha01::_0,
            true => Cmplcha01::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha01::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha01::_1
    }
}
#[doc = "Field `CMPLCHA01` writer - Comparison condition of AN001"]
pub type Cmplcha01W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha01>;
impl<'a, REG> Cmplcha01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha01::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha01::_1)
    }
}
#[doc = "Comparison condition of AN002\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha02 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha02> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA02` reader - Comparison condition of AN002"]
pub type Cmplcha02R = crate::BitReader<Cmplcha02>;
impl Cmplcha02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha02 {
        match self.bits {
            false => Cmplcha02::_0,
            true => Cmplcha02::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha02::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha02::_1
    }
}
#[doc = "Field `CMPLCHA02` writer - Comparison condition of AN002"]
pub type Cmplcha02W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha02>;
impl<'a, REG> Cmplcha02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha02::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha02::_1)
    }
}
#[doc = "Comparison condition of AN003\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha03 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha03> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA03` reader - Comparison condition of AN003"]
pub type Cmplcha03R = crate::BitReader<Cmplcha03>;
impl Cmplcha03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha03 {
        match self.bits {
            false => Cmplcha03::_0,
            true => Cmplcha03::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha03::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha03::_1
    }
}
#[doc = "Field `CMPLCHA03` writer - Comparison condition of AN003"]
pub type Cmplcha03W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha03>;
impl<'a, REG> Cmplcha03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha03::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha03::_1)
    }
}
#[doc = "Comparison condition of AN004\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha04 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha04> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA04` reader - Comparison condition of AN004"]
pub type Cmplcha04R = crate::BitReader<Cmplcha04>;
impl Cmplcha04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha04 {
        match self.bits {
            false => Cmplcha04::_0,
            true => Cmplcha04::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha04::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha04::_1
    }
}
#[doc = "Field `CMPLCHA04` writer - Comparison condition of AN004"]
pub type Cmplcha04W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha04>;
impl<'a, REG> Cmplcha04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha04::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha04::_1)
    }
}
#[doc = "Comparison condition of AN005\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha05 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha05> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA05` reader - Comparison condition of AN005"]
pub type Cmplcha05R = crate::BitReader<Cmplcha05>;
impl Cmplcha05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha05 {
        match self.bits {
            false => Cmplcha05::_0,
            true => Cmplcha05::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha05::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha05::_1
    }
}
#[doc = "Field `CMPLCHA05` writer - Comparison condition of AN005"]
pub type Cmplcha05W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha05>;
impl<'a, REG> Cmplcha05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha05::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha05::_1)
    }
}
#[doc = "Comparison condition of AN006\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha06 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha06> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA06` reader - Comparison condition of AN006"]
pub type Cmplcha06R = crate::BitReader<Cmplcha06>;
impl Cmplcha06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha06 {
        match self.bits {
            false => Cmplcha06::_0,
            true => Cmplcha06::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha06::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha06::_1
    }
}
#[doc = "Field `CMPLCHA06` writer - Comparison condition of AN006"]
pub type Cmplcha06W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha06>;
impl<'a, REG> Cmplcha06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha06::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha06::_1)
    }
}
#[doc = "Comparison condition of AN007\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha07 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha07> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA07` reader - Comparison condition of AN007"]
pub type Cmplcha07R = crate::BitReader<Cmplcha07>;
impl Cmplcha07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha07 {
        match self.bits {
            false => Cmplcha07::_0,
            true => Cmplcha07::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha07::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha07::_1
    }
}
#[doc = "Field `CMPLCHA07` writer - Comparison condition of AN007"]
pub type Cmplcha07W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha07>;
impl<'a, REG> Cmplcha07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha07::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha07::_1)
    }
}
#[doc = "Comparison condition of AN008\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha08 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha08> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA08` reader - Comparison condition of AN008"]
pub type Cmplcha08R = crate::BitReader<Cmplcha08>;
impl Cmplcha08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha08 {
        match self.bits {
            false => Cmplcha08::_0,
            true => Cmplcha08::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha08::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha08::_1
    }
}
#[doc = "Field `CMPLCHA08` writer - Comparison condition of AN008"]
pub type Cmplcha08W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha08>;
impl<'a, REG> Cmplcha08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha08::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha08::_1)
    }
}
#[doc = "Comparison condition of AN009\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha09 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha09> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA09` reader - Comparison condition of AN009"]
pub type Cmplcha09R = crate::BitReader<Cmplcha09>;
impl Cmplcha09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha09 {
        match self.bits {
            false => Cmplcha09::_0,
            true => Cmplcha09::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha09::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha09::_1
    }
}
#[doc = "Field `CMPLCHA09` writer - Comparison condition of AN009"]
pub type Cmplcha09W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha09>;
impl<'a, REG> Cmplcha09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha09::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha09::_1)
    }
}
#[doc = "Comparison condition of AN010\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha10 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha10> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA10` reader - Comparison condition of AN010"]
pub type Cmplcha10R = crate::BitReader<Cmplcha10>;
impl Cmplcha10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha10 {
        match self.bits {
            false => Cmplcha10::_0,
            true => Cmplcha10::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha10::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha10::_1
    }
}
#[doc = "Field `CMPLCHA10` writer - Comparison condition of AN010"]
pub type Cmplcha10W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha10>;
impl<'a, REG> Cmplcha10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha10::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha10::_1)
    }
}
#[doc = "Comparison condition of AN011\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha11 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha11> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA11` reader - Comparison condition of AN011"]
pub type Cmplcha11R = crate::BitReader<Cmplcha11>;
impl Cmplcha11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha11 {
        match self.bits {
            false => Cmplcha11::_0,
            true => Cmplcha11::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha11::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha11::_1
    }
}
#[doc = "Field `CMPLCHA11` writer - Comparison condition of AN011"]
pub type Cmplcha11W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha11>;
impl<'a, REG> Cmplcha11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha11::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha11::_1)
    }
}
#[doc = "Comparison condition of AN012\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha12 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha12> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA12` reader - Comparison condition of AN012"]
pub type Cmplcha12R = crate::BitReader<Cmplcha12>;
impl Cmplcha12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha12 {
        match self.bits {
            false => Cmplcha12::_0,
            true => Cmplcha12::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha12::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha12::_1
    }
}
#[doc = "Field `CMPLCHA12` writer - Comparison condition of AN012"]
pub type Cmplcha12W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha12>;
impl<'a, REG> Cmplcha12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha12::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha12::_1)
    }
}
#[doc = "Comparison condition of AN013\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha13 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha13> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA13` reader - Comparison condition of AN013"]
pub type Cmplcha13R = crate::BitReader<Cmplcha13>;
impl Cmplcha13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha13 {
        match self.bits {
            false => Cmplcha13::_0,
            true => Cmplcha13::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha13::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha13::_1
    }
}
#[doc = "Field `CMPLCHA13` writer - Comparison condition of AN013"]
pub type Cmplcha13W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha13>;
impl<'a, REG> Cmplcha13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha13::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha13::_1)
    }
}
#[doc = "Comparison condition of AN014\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha14 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha14> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA14` reader - Comparison condition of AN014"]
pub type Cmplcha14R = crate::BitReader<Cmplcha14>;
impl Cmplcha14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha14 {
        match self.bits {
            false => Cmplcha14::_0,
            true => Cmplcha14::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha14::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha14::_1
    }
}
#[doc = "Field `CMPLCHA14` writer - Comparison condition of AN014"]
pub type Cmplcha14W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha14>;
impl<'a, REG> Cmplcha14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha14::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha14::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparison condition of AN000"]
    #[inline(always)]
    pub fn cmplcha00(&self) -> Cmplcha00R {
        Cmplcha00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN001"]
    #[inline(always)]
    pub fn cmplcha01(&self) -> Cmplcha01R {
        Cmplcha01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison condition of AN002"]
    #[inline(always)]
    pub fn cmplcha02(&self) -> Cmplcha02R {
        Cmplcha02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison condition of AN003"]
    #[inline(always)]
    pub fn cmplcha03(&self) -> Cmplcha03R {
        Cmplcha03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison condition of AN004"]
    #[inline(always)]
    pub fn cmplcha04(&self) -> Cmplcha04R {
        Cmplcha04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison condition of AN005"]
    #[inline(always)]
    pub fn cmplcha05(&self) -> Cmplcha05R {
        Cmplcha05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison condition of AN006"]
    #[inline(always)]
    pub fn cmplcha06(&self) -> Cmplcha06R {
        Cmplcha06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison condition of AN007"]
    #[inline(always)]
    pub fn cmplcha07(&self) -> Cmplcha07R {
        Cmplcha07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison condition of AN008"]
    #[inline(always)]
    pub fn cmplcha08(&self) -> Cmplcha08R {
        Cmplcha08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison condition of AN009"]
    #[inline(always)]
    pub fn cmplcha09(&self) -> Cmplcha09R {
        Cmplcha09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparison condition of AN010"]
    #[inline(always)]
    pub fn cmplcha10(&self) -> Cmplcha10R {
        Cmplcha10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison condition of AN011"]
    #[inline(always)]
    pub fn cmplcha11(&self) -> Cmplcha11R {
        Cmplcha11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparison condition of AN012"]
    #[inline(always)]
    pub fn cmplcha12(&self) -> Cmplcha12R {
        Cmplcha12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparison condition of AN013"]
    #[inline(always)]
    pub fn cmplcha13(&self) -> Cmplcha13R {
        Cmplcha13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparison condition of AN014"]
    #[inline(always)]
    pub fn cmplcha14(&self) -> Cmplcha14R {
        Cmplcha14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPLR0")
            .field("cmplcha14", &self.cmplcha14())
            .field("cmplcha13", &self.cmplcha13())
            .field("cmplcha12", &self.cmplcha12())
            .field("cmplcha11", &self.cmplcha11())
            .field("cmplcha10", &self.cmplcha10())
            .field("cmplcha09", &self.cmplcha09())
            .field("cmplcha08", &self.cmplcha08())
            .field("cmplcha07", &self.cmplcha07())
            .field("cmplcha06", &self.cmplcha06())
            .field("cmplcha05", &self.cmplcha05())
            .field("cmplcha04", &self.cmplcha04())
            .field("cmplcha03", &self.cmplcha03())
            .field("cmplcha02", &self.cmplcha02())
            .field("cmplcha01", &self.cmplcha01())
            .field("cmplcha00", &self.cmplcha00())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Comparison condition of AN000"]
    #[inline(always)]
    pub fn cmplcha00(&mut self) -> Cmplcha00W<Adcmplr0Spec> {
        Cmplcha00W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN001"]
    #[inline(always)]
    pub fn cmplcha01(&mut self) -> Cmplcha01W<Adcmplr0Spec> {
        Cmplcha01W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparison condition of AN002"]
    #[inline(always)]
    pub fn cmplcha02(&mut self) -> Cmplcha02W<Adcmplr0Spec> {
        Cmplcha02W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparison condition of AN003"]
    #[inline(always)]
    pub fn cmplcha03(&mut self) -> Cmplcha03W<Adcmplr0Spec> {
        Cmplcha03W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparison condition of AN004"]
    #[inline(always)]
    pub fn cmplcha04(&mut self) -> Cmplcha04W<Adcmplr0Spec> {
        Cmplcha04W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparison condition of AN005"]
    #[inline(always)]
    pub fn cmplcha05(&mut self) -> Cmplcha05W<Adcmplr0Spec> {
        Cmplcha05W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparison condition of AN006"]
    #[inline(always)]
    pub fn cmplcha06(&mut self) -> Cmplcha06W<Adcmplr0Spec> {
        Cmplcha06W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparison condition of AN007"]
    #[inline(always)]
    pub fn cmplcha07(&mut self) -> Cmplcha07W<Adcmplr0Spec> {
        Cmplcha07W::new(self, 7)
    }
    #[doc = "Bit 8 - Comparison condition of AN008"]
    #[inline(always)]
    pub fn cmplcha08(&mut self) -> Cmplcha08W<Adcmplr0Spec> {
        Cmplcha08W::new(self, 8)
    }
    #[doc = "Bit 9 - Comparison condition of AN009"]
    #[inline(always)]
    pub fn cmplcha09(&mut self) -> Cmplcha09W<Adcmplr0Spec> {
        Cmplcha09W::new(self, 9)
    }
    #[doc = "Bit 10 - Comparison condition of AN010"]
    #[inline(always)]
    pub fn cmplcha10(&mut self) -> Cmplcha10W<Adcmplr0Spec> {
        Cmplcha10W::new(self, 10)
    }
    #[doc = "Bit 11 - Comparison condition of AN011"]
    #[inline(always)]
    pub fn cmplcha11(&mut self) -> Cmplcha11W<Adcmplr0Spec> {
        Cmplcha11W::new(self, 11)
    }
    #[doc = "Bit 12 - Comparison condition of AN012"]
    #[inline(always)]
    pub fn cmplcha12(&mut self) -> Cmplcha12W<Adcmplr0Spec> {
        Cmplcha12W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparison condition of AN013"]
    #[inline(always)]
    pub fn cmplcha13(&mut self) -> Cmplcha13W<Adcmplr0Spec> {
        Cmplcha13W::new(self, 13)
    }
    #[doc = "Bit 14 - Comparison condition of AN014"]
    #[inline(always)]
    pub fn cmplcha14(&mut self) -> Cmplcha14W<Adcmplr0Spec> {
        Cmplcha14W::new(self, 14)
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmplr0Spec;
impl crate::RegisterSpec for Adcmplr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmplr0::R`](R) reader structure"]
impl crate::Readable for Adcmplr0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmplr0::W`](W) writer structure"]
impl crate::Writable for Adcmplr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPLR0 to value 0"]
impl crate::Resettable for Adcmplr0Spec {}

#[doc = "Register `ADCMPLR1` reader"]
pub type R = crate::R<Adcmplr1Spec>;
#[doc = "Register `ADCMPLR1` writer"]
pub type W = crate::W<Adcmplr1Spec>;
#[doc = "Comparison condition of AN016\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha16 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha16> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA16` reader - Comparison condition of AN016"]
pub type Cmplcha16R = crate::BitReader<Cmplcha16>;
impl Cmplcha16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha16 {
        match self.bits {
            false => Cmplcha16::_0,
            true => Cmplcha16::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha16::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha16::_1
    }
}
#[doc = "Field `CMPLCHA16` writer - Comparison condition of AN016"]
pub type Cmplcha16W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha16>;
impl<'a, REG> Cmplcha16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha16::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha16::_1)
    }
}
#[doc = "Comparison condition of AN017\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha17 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha17> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA17` reader - Comparison condition of AN017"]
pub type Cmplcha17R = crate::BitReader<Cmplcha17>;
impl Cmplcha17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha17 {
        match self.bits {
            false => Cmplcha17::_0,
            true => Cmplcha17::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha17::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha17::_1
    }
}
#[doc = "Field `CMPLCHA17` writer - Comparison condition of AN017"]
pub type Cmplcha17W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha17>;
impl<'a, REG> Cmplcha17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha17::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha17::_1)
    }
}
#[doc = "Comparison condition of AN018\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha18 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha18> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA18` reader - Comparison condition of AN018"]
pub type Cmplcha18R = crate::BitReader<Cmplcha18>;
impl Cmplcha18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha18 {
        match self.bits {
            false => Cmplcha18::_0,
            true => Cmplcha18::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha18::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha18::_1
    }
}
#[doc = "Field `CMPLCHA18` writer - Comparison condition of AN018"]
pub type Cmplcha18W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha18>;
impl<'a, REG> Cmplcha18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha18::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha18::_1)
    }
}
#[doc = "Comparison condition of AN019\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha19 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha19> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA19` reader - Comparison condition of AN019"]
pub type Cmplcha19R = crate::BitReader<Cmplcha19>;
impl Cmplcha19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha19 {
        match self.bits {
            false => Cmplcha19::_0,
            true => Cmplcha19::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha19::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha19::_1
    }
}
#[doc = "Field `CMPLCHA19` writer - Comparison condition of AN019"]
pub type Cmplcha19W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha19>;
impl<'a, REG> Cmplcha19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha19::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha19::_1)
    }
}
#[doc = "Comparison condition of AN020\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha20 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha20> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA20` reader - Comparison condition of AN020"]
pub type Cmplcha20R = crate::BitReader<Cmplcha20>;
impl Cmplcha20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha20 {
        match self.bits {
            false => Cmplcha20::_0,
            true => Cmplcha20::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha20::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha20::_1
    }
}
#[doc = "Field `CMPLCHA20` writer - Comparison condition of AN020"]
pub type Cmplcha20W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha20>;
impl<'a, REG> Cmplcha20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha20::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha20::_1)
    }
}
#[doc = "Comparison condition of AN021\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha21 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha21> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA21` reader - Comparison condition of AN021"]
pub type Cmplcha21R = crate::BitReader<Cmplcha21>;
impl Cmplcha21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha21 {
        match self.bits {
            false => Cmplcha21::_0,
            true => Cmplcha21::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha21::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha21::_1
    }
}
#[doc = "Field `CMPLCHA21` writer - Comparison condition of AN021"]
pub type Cmplcha21W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha21>;
impl<'a, REG> Cmplcha21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha21::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha21::_1)
    }
}
#[doc = "Comparison condition of AN022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha22 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha22> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA22` reader - Comparison condition of AN022"]
pub type Cmplcha22R = crate::BitReader<Cmplcha22>;
impl Cmplcha22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha22 {
        match self.bits {
            false => Cmplcha22::_0,
            true => Cmplcha22::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha22::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha22::_1
    }
}
#[doc = "Field `CMPLCHA22` writer - Comparison condition of AN022"]
pub type Cmplcha22W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha22>;
impl<'a, REG> Cmplcha22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha22::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha22::_1)
    }
}
#[doc = "Comparison condition of AN023\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha23 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha23> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA23` reader - Comparison condition of AN023"]
pub type Cmplcha23R = crate::BitReader<Cmplcha23>;
impl Cmplcha23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha23 {
        match self.bits {
            false => Cmplcha23::_0,
            true => Cmplcha23::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha23::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha23::_1
    }
}
#[doc = "Field `CMPLCHA23` writer - Comparison condition of AN023"]
pub type Cmplcha23W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha23>;
impl<'a, REG> Cmplcha23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha23::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha23::_1)
    }
}
#[doc = "Comparison condition of AN024\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha24 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha24> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA24` reader - Comparison condition of AN024"]
pub type Cmplcha24R = crate::BitReader<Cmplcha24>;
impl Cmplcha24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha24 {
        match self.bits {
            false => Cmplcha24::_0,
            true => Cmplcha24::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha24::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha24::_1
    }
}
#[doc = "Field `CMPLCHA24` writer - Comparison condition of AN024"]
pub type Cmplcha24W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha24>;
impl<'a, REG> Cmplcha24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha24::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha24::_1)
    }
}
#[doc = "Comparison condition of AN025\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha25 {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<Cmplcha25> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLCHA25` reader - Comparison condition of AN025"]
pub type Cmplcha25R = crate::BitReader<Cmplcha25>;
impl Cmplcha25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha25 {
        match self.bits {
            false => Cmplcha25::_0,
            true => Cmplcha25::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha25::_0
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha25::_1
    }
}
#[doc = "Field `CMPLCHA25` writer - Comparison condition of AN025"]
pub type Cmplcha25W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha25>;
impl<'a, REG> Cmplcha25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha25::_0)
    }
    #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha25::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    pub fn cmplcha16(&self) -> Cmplcha16R {
        Cmplcha16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    pub fn cmplcha17(&self) -> Cmplcha17R {
        Cmplcha17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    pub fn cmplcha18(&self) -> Cmplcha18R {
        Cmplcha18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    pub fn cmplcha19(&self) -> Cmplcha19R {
        Cmplcha19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    pub fn cmplcha20(&self) -> Cmplcha20R {
        Cmplcha20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    pub fn cmplcha21(&self) -> Cmplcha21R {
        Cmplcha21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    pub fn cmplcha22(&self) -> Cmplcha22R {
        Cmplcha22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    pub fn cmplcha23(&self) -> Cmplcha23R {
        Cmplcha23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    pub fn cmplcha24(&self) -> Cmplcha24R {
        Cmplcha24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    pub fn cmplcha25(&self) -> Cmplcha25R {
        Cmplcha25R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPLR1")
            .field("cmplcha25", &self.cmplcha25())
            .field("cmplcha24", &self.cmplcha24())
            .field("cmplcha23", &self.cmplcha23())
            .field("cmplcha22", &self.cmplcha22())
            .field("cmplcha21", &self.cmplcha21())
            .field("cmplcha20", &self.cmplcha20())
            .field("cmplcha19", &self.cmplcha19())
            .field("cmplcha18", &self.cmplcha18())
            .field("cmplcha17", &self.cmplcha17())
            .field("cmplcha16", &self.cmplcha16())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    pub fn cmplcha16(&mut self) -> Cmplcha16W<Adcmplr1Spec> {
        Cmplcha16W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    pub fn cmplcha17(&mut self) -> Cmplcha17W<Adcmplr1Spec> {
        Cmplcha17W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    pub fn cmplcha18(&mut self) -> Cmplcha18W<Adcmplr1Spec> {
        Cmplcha18W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    pub fn cmplcha19(&mut self) -> Cmplcha19W<Adcmplr1Spec> {
        Cmplcha19W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    pub fn cmplcha20(&mut self) -> Cmplcha20W<Adcmplr1Spec> {
        Cmplcha20W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    pub fn cmplcha21(&mut self) -> Cmplcha21W<Adcmplr1Spec> {
        Cmplcha21W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    pub fn cmplcha22(&mut self) -> Cmplcha22W<Adcmplr1Spec> {
        Cmplcha22W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    pub fn cmplcha23(&mut self) -> Cmplcha23W<Adcmplr1Spec> {
        Cmplcha23W::new(self, 7)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    pub fn cmplcha24(&mut self) -> Cmplcha24W<Adcmplr1Spec> {
        Cmplcha24W::new(self, 8)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    pub fn cmplcha25(&mut self) -> Cmplcha25W<Adcmplr1Spec> {
        Cmplcha25W::new(self, 9)
    }
}
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmplr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmplr1Spec;
impl crate::RegisterSpec for Adcmplr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmplr1::R`](R) reader structure"]
impl crate::Readable for Adcmplr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmplr1::W`](W) writer structure"]
impl crate::Writable for Adcmplr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPLR1 to value 0"]
impl crate::Resettable for Adcmplr1Spec {}

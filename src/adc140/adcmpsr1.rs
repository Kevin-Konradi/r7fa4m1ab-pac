#[doc = "Register `ADCMPSR1` reader"]
pub type R = crate::R<Adcmpsr1Spec>;
#[doc = "Register `ADCMPSR1` writer"]
pub type W = crate::W<Adcmpsr1Spec>;
#[doc = "Compare window A flag of AN016\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha16 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha16> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA16` reader - Compare window A flag of AN016\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha16R = crate::BitReader<Cmpstcha16>;
impl Cmpstcha16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha16 {
        match self.bits {
            false => Cmpstcha16::_0,
            true => Cmpstcha16::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha16::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha16::_1
    }
}
#[doc = "Field `CMPSTCHA16` writer - Compare window A flag of AN016"]
pub type Cmpstcha16W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha16>;
impl<'a, REG> Cmpstcha16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha16::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha16::_1)
    }
}
#[doc = "Compare window A flag of AN017\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha17 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha17> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA17` reader - Compare window A flag of AN017\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha17R = crate::BitReader<Cmpstcha17>;
impl Cmpstcha17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha17 {
        match self.bits {
            false => Cmpstcha17::_0,
            true => Cmpstcha17::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha17::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha17::_1
    }
}
#[doc = "Field `CMPSTCHA17` writer - Compare window A flag of AN017"]
pub type Cmpstcha17W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha17>;
impl<'a, REG> Cmpstcha17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha17::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha17::_1)
    }
}
#[doc = "Compare window A flag of AN018\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha18 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha18> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA18` reader - Compare window A flag of AN018\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha18R = crate::BitReader<Cmpstcha18>;
impl Cmpstcha18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha18 {
        match self.bits {
            false => Cmpstcha18::_0,
            true => Cmpstcha18::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha18::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha18::_1
    }
}
#[doc = "Field `CMPSTCHA18` writer - Compare window A flag of AN018"]
pub type Cmpstcha18W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha18>;
impl<'a, REG> Cmpstcha18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha18::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha18::_1)
    }
}
#[doc = "Compare window A flag of AN019\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha19 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha19> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA19` reader - Compare window A flag of AN019\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha19R = crate::BitReader<Cmpstcha19>;
impl Cmpstcha19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha19 {
        match self.bits {
            false => Cmpstcha19::_0,
            true => Cmpstcha19::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha19::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha19::_1
    }
}
#[doc = "Field `CMPSTCHA19` writer - Compare window A flag of AN019"]
pub type Cmpstcha19W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha19>;
impl<'a, REG> Cmpstcha19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha19::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha19::_1)
    }
}
#[doc = "Compare window A flag of AN020\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha20 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha20> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA20` reader - Compare window A flag of AN020\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha20R = crate::BitReader<Cmpstcha20>;
impl Cmpstcha20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha20 {
        match self.bits {
            false => Cmpstcha20::_0,
            true => Cmpstcha20::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha20::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha20::_1
    }
}
#[doc = "Field `CMPSTCHA20` writer - Compare window A flag of AN020"]
pub type Cmpstcha20W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha20>;
impl<'a, REG> Cmpstcha20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha20::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha20::_1)
    }
}
#[doc = "Compare window A flag of AN021\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha21 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha21> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA21` reader - Compare window A flag of AN021\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha21R = crate::BitReader<Cmpstcha21>;
impl Cmpstcha21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha21 {
        match self.bits {
            false => Cmpstcha21::_0,
            true => Cmpstcha21::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha21::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha21::_1
    }
}
#[doc = "Field `CMPSTCHA21` writer - Compare window A flag of AN021"]
pub type Cmpstcha21W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha21>;
impl<'a, REG> Cmpstcha21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha21::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha21::_1)
    }
}
#[doc = "Compare window A flag of AN022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha22 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha22> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA22` reader - Compare window A flag of AN022\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha22R = crate::BitReader<Cmpstcha22>;
impl Cmpstcha22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha22 {
        match self.bits {
            false => Cmpstcha22::_0,
            true => Cmpstcha22::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha22::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha22::_1
    }
}
#[doc = "Field `CMPSTCHA22` writer - Compare window A flag of AN022"]
pub type Cmpstcha22W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha22>;
impl<'a, REG> Cmpstcha22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha22::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha22::_1)
    }
}
#[doc = "Compare window A flag of AN023\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha23 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha23> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA23` reader - Compare window A flag of AN023\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha23R = crate::BitReader<Cmpstcha23>;
impl Cmpstcha23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha23 {
        match self.bits {
            false => Cmpstcha23::_0,
            true => Cmpstcha23::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha23::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha23::_1
    }
}
#[doc = "Field `CMPSTCHA23` writer - Compare window A flag of AN023"]
pub type Cmpstcha23W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha23>;
impl<'a, REG> Cmpstcha23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha23::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha23::_1)
    }
}
#[doc = "Compare window A flag of AN024\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha24 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha24> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA24` reader - Compare window A flag of AN024\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha24R = crate::BitReader<Cmpstcha24>;
impl Cmpstcha24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha24 {
        match self.bits {
            false => Cmpstcha24::_0,
            true => Cmpstcha24::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha24::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha24::_1
    }
}
#[doc = "Field `CMPSTCHA24` writer - Compare window A flag of AN024"]
pub type Cmpstcha24W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha24>;
impl<'a, REG> Cmpstcha24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha24::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha24::_1)
    }
}
#[doc = "Compare window A flag of AN025\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha25 {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstcha25> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTCHA25` reader - Compare window A flag of AN025\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Cmpstcha25R = crate::BitReader<Cmpstcha25>;
impl Cmpstcha25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha25 {
        match self.bits {
            false => Cmpstcha25::_0,
            true => Cmpstcha25::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha25::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha25::_1
    }
}
#[doc = "Field `CMPSTCHA25` writer - Compare window A flag of AN025"]
pub type Cmpstcha25W<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstcha25>;
impl<'a, REG> Cmpstcha25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha25::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha25::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window A flag of AN016"]
    #[inline(always)]
    pub fn cmpstcha16(&self) -> Cmpstcha16R {
        Cmpstcha16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN017"]
    #[inline(always)]
    pub fn cmpstcha17(&self) -> Cmpstcha17R {
        Cmpstcha17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare window A flag of AN018"]
    #[inline(always)]
    pub fn cmpstcha18(&self) -> Cmpstcha18R {
        Cmpstcha18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare window A flag of AN019"]
    #[inline(always)]
    pub fn cmpstcha19(&self) -> Cmpstcha19R {
        Cmpstcha19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare window A flag of AN020"]
    #[inline(always)]
    pub fn cmpstcha20(&self) -> Cmpstcha20R {
        Cmpstcha20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare window A flag of AN021"]
    #[inline(always)]
    pub fn cmpstcha21(&self) -> Cmpstcha21R {
        Cmpstcha21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare window A flag of AN022"]
    #[inline(always)]
    pub fn cmpstcha22(&self) -> Cmpstcha22R {
        Cmpstcha22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare window A flag of AN023"]
    #[inline(always)]
    pub fn cmpstcha23(&self) -> Cmpstcha23R {
        Cmpstcha23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare window A flag of AN024"]
    #[inline(always)]
    pub fn cmpstcha24(&self) -> Cmpstcha24R {
        Cmpstcha24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare window A flag of AN025"]
    #[inline(always)]
    pub fn cmpstcha25(&self) -> Cmpstcha25R {
        Cmpstcha25R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSR1").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare window A flag of AN016"]
    #[inline(always)]
    pub fn cmpstcha16(&mut self) -> Cmpstcha16W<Adcmpsr1Spec> {
        Cmpstcha16W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN017"]
    #[inline(always)]
    pub fn cmpstcha17(&mut self) -> Cmpstcha17W<Adcmpsr1Spec> {
        Cmpstcha17W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare window A flag of AN018"]
    #[inline(always)]
    pub fn cmpstcha18(&mut self) -> Cmpstcha18W<Adcmpsr1Spec> {
        Cmpstcha18W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare window A flag of AN019"]
    #[inline(always)]
    pub fn cmpstcha19(&mut self) -> Cmpstcha19W<Adcmpsr1Spec> {
        Cmpstcha19W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare window A flag of AN020"]
    #[inline(always)]
    pub fn cmpstcha20(&mut self) -> Cmpstcha20W<Adcmpsr1Spec> {
        Cmpstcha20W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare window A flag of AN021"]
    #[inline(always)]
    pub fn cmpstcha21(&mut self) -> Cmpstcha21W<Adcmpsr1Spec> {
        Cmpstcha21W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare window A flag of AN022"]
    #[inline(always)]
    pub fn cmpstcha22(&mut self) -> Cmpstcha22W<Adcmpsr1Spec> {
        Cmpstcha22W::new(self, 6)
    }
    #[doc = "Bit 7 - Compare window A flag of AN023"]
    #[inline(always)]
    pub fn cmpstcha23(&mut self) -> Cmpstcha23W<Adcmpsr1Spec> {
        Cmpstcha23W::new(self, 7)
    }
    #[doc = "Bit 8 - Compare window A flag of AN024"]
    #[inline(always)]
    pub fn cmpstcha24(&mut self) -> Cmpstcha24W<Adcmpsr1Spec> {
        Cmpstcha24W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare window A flag of AN025"]
    #[inline(always)]
    pub fn cmpstcha25(&mut self) -> Cmpstcha25W<Adcmpsr1Spec> {
        Cmpstcha25W::new(self, 9)
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmpsr1Spec;
impl crate::RegisterSpec for Adcmpsr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpsr1::R`](R) reader structure"]
impl crate::Readable for Adcmpsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmpsr1::W`](W) writer structure"]
impl crate::Writable for Adcmpsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
#[doc = "`reset()` method sets ADCMPSR1 to value 0"]
impl crate::Resettable for Adcmpsr1Spec {}

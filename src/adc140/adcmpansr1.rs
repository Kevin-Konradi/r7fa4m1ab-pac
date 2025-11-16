#[doc = "Register `ADCMPANSR1` reader"]
pub type R = crate::R<Adcmpansr1Spec>;
#[doc = "Register `ADCMPANSR1` writer"]
pub type W = crate::W<Adcmpansr1Spec>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha16 {
    #[doc = "0: Excludes AN016 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN016 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha16> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA16` reader - AN016 Select"]
pub type Cmpcha16R = crate::BitReader<Cmpcha16>;
impl Cmpcha16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha16 {
        match self.bits {
            false => Cmpcha16::_0,
            true => Cmpcha16::_1,
        }
    }
    #[doc = "Excludes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha16::_0
    }
    #[doc = "Includes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha16::_1
    }
}
#[doc = "Field `CMPCHA16` writer - AN016 Select"]
pub type Cmpcha16W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha16>;
impl<'a, REG> Cmpcha16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha16::_0)
    }
    #[doc = "Includes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha16::_1)
    }
}
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha17 {
    #[doc = "0: Excludes AN017 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN017 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha17> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA17` reader - AN017 Select"]
pub type Cmpcha17R = crate::BitReader<Cmpcha17>;
impl Cmpcha17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha17 {
        match self.bits {
            false => Cmpcha17::_0,
            true => Cmpcha17::_1,
        }
    }
    #[doc = "Excludes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha17::_0
    }
    #[doc = "Includes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha17::_1
    }
}
#[doc = "Field `CMPCHA17` writer - AN017 Select"]
pub type Cmpcha17W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha17>;
impl<'a, REG> Cmpcha17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha17::_0)
    }
    #[doc = "Includes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha17::_1)
    }
}
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha18 {
    #[doc = "0: Excludes AN018 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN018 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha18> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA18` reader - AN018 Select"]
pub type Cmpcha18R = crate::BitReader<Cmpcha18>;
impl Cmpcha18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha18 {
        match self.bits {
            false => Cmpcha18::_0,
            true => Cmpcha18::_1,
        }
    }
    #[doc = "Excludes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha18::_0
    }
    #[doc = "Includes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha18::_1
    }
}
#[doc = "Field `CMPCHA18` writer - AN018 Select"]
pub type Cmpcha18W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha18>;
impl<'a, REG> Cmpcha18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha18::_0)
    }
    #[doc = "Includes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha18::_1)
    }
}
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha19 {
    #[doc = "0: Excludes AN019 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN019 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha19> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA19` reader - AN019 Select"]
pub type Cmpcha19R = crate::BitReader<Cmpcha19>;
impl Cmpcha19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha19 {
        match self.bits {
            false => Cmpcha19::_0,
            true => Cmpcha19::_1,
        }
    }
    #[doc = "Excludes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha19::_0
    }
    #[doc = "Includes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha19::_1
    }
}
#[doc = "Field `CMPCHA19` writer - AN019 Select"]
pub type Cmpcha19W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha19>;
impl<'a, REG> Cmpcha19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha19::_0)
    }
    #[doc = "Includes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha19::_1)
    }
}
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha20 {
    #[doc = "0: Excludes AN020 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN020 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha20> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA20` reader - AN020 Select"]
pub type Cmpcha20R = crate::BitReader<Cmpcha20>;
impl Cmpcha20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha20 {
        match self.bits {
            false => Cmpcha20::_0,
            true => Cmpcha20::_1,
        }
    }
    #[doc = "Excludes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha20::_0
    }
    #[doc = "Includes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha20::_1
    }
}
#[doc = "Field `CMPCHA20` writer - AN020 Select"]
pub type Cmpcha20W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha20>;
impl<'a, REG> Cmpcha20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha20::_0)
    }
    #[doc = "Includes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha20::_1)
    }
}
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha21 {
    #[doc = "0: Excludes AN021 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN021 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha21> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA21` reader - AN021 Select"]
pub type Cmpcha21R = crate::BitReader<Cmpcha21>;
impl Cmpcha21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha21 {
        match self.bits {
            false => Cmpcha21::_0,
            true => Cmpcha21::_1,
        }
    }
    #[doc = "Excludes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha21::_0
    }
    #[doc = "Includes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha21::_1
    }
}
#[doc = "Field `CMPCHA21` writer - AN021 Select"]
pub type Cmpcha21W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha21>;
impl<'a, REG> Cmpcha21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha21::_0)
    }
    #[doc = "Includes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha21::_1)
    }
}
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha22 {
    #[doc = "0: Excludes AN022 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN022 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha22> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA22` reader - AN022 Select"]
pub type Cmpcha22R = crate::BitReader<Cmpcha22>;
impl Cmpcha22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha22 {
        match self.bits {
            false => Cmpcha22::_0,
            true => Cmpcha22::_1,
        }
    }
    #[doc = "Excludes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha22::_0
    }
    #[doc = "Includes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha22::_1
    }
}
#[doc = "Field `CMPCHA22` writer - AN022 Select"]
pub type Cmpcha22W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha22>;
impl<'a, REG> Cmpcha22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha22::_0)
    }
    #[doc = "Includes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha22::_1)
    }
}
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha23 {
    #[doc = "0: Excludes AN023 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN023 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha23> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA23` reader - AN023 Select"]
pub type Cmpcha23R = crate::BitReader<Cmpcha23>;
impl Cmpcha23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha23 {
        match self.bits {
            false => Cmpcha23::_0,
            true => Cmpcha23::_1,
        }
    }
    #[doc = "Excludes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha23::_0
    }
    #[doc = "Includes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha23::_1
    }
}
#[doc = "Field `CMPCHA23` writer - AN023 Select"]
pub type Cmpcha23W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha23>;
impl<'a, REG> Cmpcha23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha23::_0)
    }
    #[doc = "Includes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha23::_1)
    }
}
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha24 {
    #[doc = "0: Excludes AN024 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN024 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha24> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA24` reader - AN024 Select"]
pub type Cmpcha24R = crate::BitReader<Cmpcha24>;
impl Cmpcha24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha24 {
        match self.bits {
            false => Cmpcha24::_0,
            true => Cmpcha24::_1,
        }
    }
    #[doc = "Excludes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha24::_0
    }
    #[doc = "Includes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha24::_1
    }
}
#[doc = "Field `CMPCHA24` writer - AN024 Select"]
pub type Cmpcha24W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha24>;
impl<'a, REG> Cmpcha24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha24::_0)
    }
    #[doc = "Includes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha24::_1)
    }
}
#[doc = "AN025 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha25 {
    #[doc = "0: Excludes AN025 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN025 from the compare window A target range."]
    _1 = 1,
}
impl From<Cmpcha25> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCHA25` reader - AN025 Select"]
pub type Cmpcha25R = crate::BitReader<Cmpcha25>;
impl Cmpcha25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha25 {
        match self.bits {
            false => Cmpcha25::_0,
            true => Cmpcha25::_1,
        }
    }
    #[doc = "Excludes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha25::_0
    }
    #[doc = "Includes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha25::_1
    }
}
#[doc = "Field `CMPCHA25` writer - AN025 Select"]
pub type Cmpcha25W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha25>;
impl<'a, REG> Cmpcha25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha25::_0)
    }
    #[doc = "Includes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha25::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn cmpcha16(&self) -> Cmpcha16R {
        Cmpcha16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn cmpcha17(&self) -> Cmpcha17R {
        Cmpcha17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn cmpcha18(&self) -> Cmpcha18R {
        Cmpcha18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn cmpcha19(&self) -> Cmpcha19R {
        Cmpcha19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn cmpcha20(&self) -> Cmpcha20R {
        Cmpcha20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn cmpcha21(&self) -> Cmpcha21R {
        Cmpcha21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn cmpcha22(&self) -> Cmpcha22R {
        Cmpcha22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn cmpcha23(&self) -> Cmpcha23R {
        Cmpcha23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn cmpcha24(&self) -> Cmpcha24R {
        Cmpcha24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn cmpcha25(&self) -> Cmpcha25R {
        Cmpcha25R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPANSR1")
            .field("cmpcha25", &self.cmpcha25())
            .field("cmpcha24", &self.cmpcha24())
            .field("cmpcha23", &self.cmpcha23())
            .field("cmpcha22", &self.cmpcha22())
            .field("cmpcha21", &self.cmpcha21())
            .field("cmpcha20", &self.cmpcha20())
            .field("cmpcha19", &self.cmpcha19())
            .field("cmpcha18", &self.cmpcha18())
            .field("cmpcha17", &self.cmpcha17())
            .field("cmpcha16", &self.cmpcha16())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn cmpcha16(&mut self) -> Cmpcha16W<Adcmpansr1Spec> {
        Cmpcha16W::new(self, 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn cmpcha17(&mut self) -> Cmpcha17W<Adcmpansr1Spec> {
        Cmpcha17W::new(self, 1)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn cmpcha18(&mut self) -> Cmpcha18W<Adcmpansr1Spec> {
        Cmpcha18W::new(self, 2)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn cmpcha19(&mut self) -> Cmpcha19W<Adcmpansr1Spec> {
        Cmpcha19W::new(self, 3)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn cmpcha20(&mut self) -> Cmpcha20W<Adcmpansr1Spec> {
        Cmpcha20W::new(self, 4)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn cmpcha21(&mut self) -> Cmpcha21W<Adcmpansr1Spec> {
        Cmpcha21W::new(self, 5)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn cmpcha22(&mut self) -> Cmpcha22W<Adcmpansr1Spec> {
        Cmpcha22W::new(self, 6)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn cmpcha23(&mut self) -> Cmpcha23W<Adcmpansr1Spec> {
        Cmpcha23W::new(self, 7)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn cmpcha24(&mut self) -> Cmpcha24W<Adcmpansr1Spec> {
        Cmpcha24W::new(self, 8)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn cmpcha25(&mut self) -> Cmpcha25W<Adcmpansr1Spec> {
        Cmpcha25W::new(self, 9)
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmpansr1Spec;
impl crate::RegisterSpec for Adcmpansr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpansr1::R`](R) reader structure"]
impl crate::Readable for Adcmpansr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmpansr1::W`](W) writer structure"]
impl crate::Writable for Adcmpansr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPANSR1 to value 0"]
impl crate::Resettable for Adcmpansr1Spec {}

#[doc = "Register `AMPTRM` reader"]
pub type R = crate::R<AmptrmSpec>;
#[doc = "Register `AMPTRM` writer"]
pub type W = crate::W<AmptrmSpec>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm00 {
    #[doc = "0: Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    _1 = 1,
}
impl From<Amptrm00> for bool {
    #[inline(always)]
    fn from(variant: Amptrm00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM00` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm00R = crate::BitReader<Amptrm00>;
impl Amptrm00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm00 {
        match self.bits {
            false => Amptrm00::_0,
            true => Amptrm00::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm00::_0
    }
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm00::_1
    }
}
#[doc = "Field `AMPTRM00` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm00W<'a, REG> = crate::BitWriter<'a, REG, Amptrm00>;
impl<'a, REG> Amptrm00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm00::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm00::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm01 {
    #[doc = "0: Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    _1 = 1,
}
impl From<Amptrm01> for bool {
    #[inline(always)]
    fn from(variant: Amptrm01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM01` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm01R = crate::BitReader<Amptrm01>;
impl Amptrm01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm01 {
        match self.bits {
            false => Amptrm01::_0,
            true => Amptrm01::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm01::_0
    }
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm01::_1
    }
}
#[doc = "Field `AMPTRM01` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm01W<'a, REG> = crate::BitWriter<'a, REG, Amptrm01>;
impl<'a, REG> Amptrm01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm01::_0)
    }
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm01::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm10 {
    #[doc = "0: Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    _1 = 1,
}
impl From<Amptrm10> for bool {
    #[inline(always)]
    fn from(variant: Amptrm10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM10` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm10R = crate::BitReader<Amptrm10>;
impl Amptrm10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm10 {
        match self.bits {
            false => Amptrm10::_0,
            true => Amptrm10::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm10::_0
    }
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm10::_1
    }
}
#[doc = "Field `AMPTRM10` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm10W<'a, REG> = crate::BitWriter<'a, REG, Amptrm10>;
impl<'a, REG> Amptrm10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm10::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm10::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm11 {
    #[doc = "0: Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    _1 = 1,
}
impl From<Amptrm11> for bool {
    #[inline(always)]
    fn from(variant: Amptrm11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM11` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm11R = crate::BitReader<Amptrm11>;
impl Amptrm11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm11 {
        match self.bits {
            false => Amptrm11::_0,
            true => Amptrm11::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm11::_0
    }
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm11::_1
    }
}
#[doc = "Field `AMPTRM11` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm11W<'a, REG> = crate::BitWriter<'a, REG, Amptrm11>;
impl<'a, REG> Amptrm11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm11::_0)
    }
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm11::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm20 {
    #[doc = "0: Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    _1 = 1,
}
impl From<Amptrm20> for bool {
    #[inline(always)]
    fn from(variant: Amptrm20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM20` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm20R = crate::BitReader<Amptrm20>;
impl Amptrm20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm20 {
        match self.bits {
            false => Amptrm20::_0,
            true => Amptrm20::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm20::_0
    }
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm20::_1
    }
}
#[doc = "Field `AMPTRM20` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm20W<'a, REG> = crate::BitWriter<'a, REG, Amptrm20>;
impl<'a, REG> Amptrm20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm20::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm20::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm21 {
    #[doc = "0: Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    _1 = 1,
}
impl From<Amptrm21> for bool {
    #[inline(always)]
    fn from(variant: Amptrm21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM21` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm21R = crate::BitReader<Amptrm21>;
impl Amptrm21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm21 {
        match self.bits {
            false => Amptrm21::_0,
            true => Amptrm21::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm21::_0
    }
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm21::_1
    }
}
#[doc = "Field `AMPTRM21` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm21W<'a, REG> = crate::BitWriter<'a, REG, Amptrm21>;
impl<'a, REG> Amptrm21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm21::_0)
    }
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm21::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm30 {
    #[doc = "0: Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    _1 = 1,
}
impl From<Amptrm30> for bool {
    #[inline(always)]
    fn from(variant: Amptrm30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM30` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm30R = crate::BitReader<Amptrm30>;
impl Amptrm30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm30 {
        match self.bits {
            false => Amptrm30::_0,
            true => Amptrm30::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm30::_0
    }
    #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm30::_1
    }
}
#[doc = "Field `AMPTRM30` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm30W<'a, REG> = crate::BitWriter<'a, REG, Amptrm30>;
impl<'a, REG> Amptrm30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm30::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm30::_1)
    }
}
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amptrm31 {
    #[doc = "0: Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    _1 = 1,
}
impl From<Amptrm31> for bool {
    #[inline(always)]
    fn from(variant: Amptrm31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPTRM31` reader - Operational amplifier function activation/stop trigger control"]
pub type Amptrm31R = crate::BitReader<Amptrm31>;
impl Amptrm31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrm31 {
        match self.bits {
            false => Amptrm31::_0,
            true => Amptrm31::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amptrm31::_0
    }
    #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amptrm31::_1
    }
}
#[doc = "Field `AMPTRM31` writer - Operational amplifier function activation/stop trigger control"]
pub type Amptrm31W<'a, REG> = crate::BitWriter<'a, REG, Amptrm31>;
impl<'a, REG> Amptrm31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm31::_0)
    }
    #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrm31::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm00(&self) -> Amptrm00R {
        Amptrm00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm01(&self) -> Amptrm01R {
        Amptrm01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm10(&self) -> Amptrm10R {
        Amptrm10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm11(&self) -> Amptrm11R {
        Amptrm11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm20(&self) -> Amptrm20R {
        Amptrm20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm21(&self) -> Amptrm21R {
        Amptrm21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm30(&self) -> Amptrm30R {
        Amptrm30R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm31(&self) -> Amptrm31R {
        Amptrm31R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPTRM")
            .field("amptrm31", &self.amptrm31())
            .field("amptrm30", &self.amptrm30())
            .field("amptrm21", &self.amptrm21())
            .field("amptrm20", &self.amptrm20())
            .field("amptrm11", &self.amptrm11())
            .field("amptrm10", &self.amptrm10())
            .field("amptrm01", &self.amptrm01())
            .field("amptrm00", &self.amptrm00())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm00(&mut self) -> Amptrm00W<AmptrmSpec> {
        Amptrm00W::new(self, 0)
    }
    #[doc = "Bit 1 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm01(&mut self) -> Amptrm01W<AmptrmSpec> {
        Amptrm01W::new(self, 1)
    }
    #[doc = "Bit 2 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm10(&mut self) -> Amptrm10W<AmptrmSpec> {
        Amptrm10W::new(self, 2)
    }
    #[doc = "Bit 3 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm11(&mut self) -> Amptrm11W<AmptrmSpec> {
        Amptrm11W::new(self, 3)
    }
    #[doc = "Bit 4 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm20(&mut self) -> Amptrm20W<AmptrmSpec> {
        Amptrm20W::new(self, 4)
    }
    #[doc = "Bit 5 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm21(&mut self) -> Amptrm21W<AmptrmSpec> {
        Amptrm21W::new(self, 5)
    }
    #[doc = "Bit 6 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm30(&mut self) -> Amptrm30W<AmptrmSpec> {
        Amptrm30W::new(self, 6)
    }
    #[doc = "Bit 7 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm31(&mut self) -> Amptrm31W<AmptrmSpec> {
        Amptrm31W::new(self, 7)
    }
}
#[doc = "Operational amplifier trigger mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`amptrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmptrmSpec;
impl crate::RegisterSpec for AmptrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`amptrm::R`](R) reader structure"]
impl crate::Readable for AmptrmSpec {}
#[doc = "`write(|w| ..)` method takes [`amptrm::W`](W) writer structure"]
impl crate::Writable for AmptrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMPTRM to value 0"]
impl crate::Resettable for AmptrmSpec {}

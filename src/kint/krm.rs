#[doc = "Register `KRM` reader"]
pub type R = crate::R<KrmSpec>;
#[doc = "Register `KRM` writer"]
pub type W = crate::W<KrmSpec>;
#[doc = "Key interrupt mode control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm0 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm0> for bool {
    #[inline(always)]
    fn from(variant: Krm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM0` reader - Key interrupt mode control 0"]
pub type Krm0R = crate::BitReader<Krm0>;
impl Krm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm0 {
        match self.bits {
            false => Krm0::_0,
            true => Krm0::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm0::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm0::_1
    }
}
#[doc = "Field `KRM0` writer - Key interrupt mode control 0"]
pub type Krm0W<'a, REG> = crate::BitWriter<'a, REG, Krm0>;
impl<'a, REG> Krm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm0::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm0::_1)
    }
}
#[doc = "Key interrupt mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm1 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm1> for bool {
    #[inline(always)]
    fn from(variant: Krm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM1` reader - Key interrupt mode control 1"]
pub type Krm1R = crate::BitReader<Krm1>;
impl Krm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm1 {
        match self.bits {
            false => Krm1::_0,
            true => Krm1::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm1::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm1::_1
    }
}
#[doc = "Field `KRM1` writer - Key interrupt mode control 1"]
pub type Krm1W<'a, REG> = crate::BitWriter<'a, REG, Krm1>;
impl<'a, REG> Krm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm1::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm1::_1)
    }
}
#[doc = "Key interrupt mode control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm2 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm2> for bool {
    #[inline(always)]
    fn from(variant: Krm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM2` reader - Key interrupt mode control 2"]
pub type Krm2R = crate::BitReader<Krm2>;
impl Krm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm2 {
        match self.bits {
            false => Krm2::_0,
            true => Krm2::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm2::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm2::_1
    }
}
#[doc = "Field `KRM2` writer - Key interrupt mode control 2"]
pub type Krm2W<'a, REG> = crate::BitWriter<'a, REG, Krm2>;
impl<'a, REG> Krm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm2::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm2::_1)
    }
}
#[doc = "Key interrupt mode control 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm3 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm3> for bool {
    #[inline(always)]
    fn from(variant: Krm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM3` reader - Key interrupt mode control 3"]
pub type Krm3R = crate::BitReader<Krm3>;
impl Krm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm3 {
        match self.bits {
            false => Krm3::_0,
            true => Krm3::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm3::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm3::_1
    }
}
#[doc = "Field `KRM3` writer - Key interrupt mode control 3"]
pub type Krm3W<'a, REG> = crate::BitWriter<'a, REG, Krm3>;
impl<'a, REG> Krm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm3::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm3::_1)
    }
}
#[doc = "Key interrupt mode control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm4 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm4> for bool {
    #[inline(always)]
    fn from(variant: Krm4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM4` reader - Key interrupt mode control 4"]
pub type Krm4R = crate::BitReader<Krm4>;
impl Krm4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm4 {
        match self.bits {
            false => Krm4::_0,
            true => Krm4::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm4::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm4::_1
    }
}
#[doc = "Field `KRM4` writer - Key interrupt mode control 4"]
pub type Krm4W<'a, REG> = crate::BitWriter<'a, REG, Krm4>;
impl<'a, REG> Krm4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm4::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm4::_1)
    }
}
#[doc = "Key interrupt mode control 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm5 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm5> for bool {
    #[inline(always)]
    fn from(variant: Krm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM5` reader - Key interrupt mode control 5"]
pub type Krm5R = crate::BitReader<Krm5>;
impl Krm5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm5 {
        match self.bits {
            false => Krm5::_0,
            true => Krm5::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm5::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm5::_1
    }
}
#[doc = "Field `KRM5` writer - Key interrupt mode control 5"]
pub type Krm5W<'a, REG> = crate::BitWriter<'a, REG, Krm5>;
impl<'a, REG> Krm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm5::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm5::_1)
    }
}
#[doc = "Key interrupt mode control 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm6 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm6> for bool {
    #[inline(always)]
    fn from(variant: Krm6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM6` reader - Key interrupt mode control 6"]
pub type Krm6R = crate::BitReader<Krm6>;
impl Krm6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm6 {
        match self.bits {
            false => Krm6::_0,
            true => Krm6::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm6::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm6::_1
    }
}
#[doc = "Field `KRM6` writer - Key interrupt mode control 6"]
pub type Krm6W<'a, REG> = crate::BitWriter<'a, REG, Krm6>;
impl<'a, REG> Krm6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm6::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm6::_1)
    }
}
#[doc = "Key interrupt mode control 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krm7 {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<Krm7> for bool {
    #[inline(always)]
    fn from(variant: Krm7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRM7` reader - Key interrupt mode control 7"]
pub type Krm7R = crate::BitReader<Krm7>;
impl Krm7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krm7 {
        match self.bits {
            false => Krm7::_0,
            true => Krm7::_1,
        }
    }
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krm7::_0
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krm7::_1
    }
}
#[doc = "Field `KRM7` writer - Key interrupt mode control 7"]
pub type Krm7W<'a, REG> = crate::BitWriter<'a, REG, Krm7>;
impl<'a, REG> Krm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krm7::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krm7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key interrupt mode control 0"]
    #[inline(always)]
    pub fn krm0(&self) -> Krm0R {
        Krm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key interrupt mode control 1"]
    #[inline(always)]
    pub fn krm1(&self) -> Krm1R {
        Krm1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key interrupt mode control 2"]
    #[inline(always)]
    pub fn krm2(&self) -> Krm2R {
        Krm2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key interrupt mode control 3"]
    #[inline(always)]
    pub fn krm3(&self) -> Krm3R {
        Krm3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key interrupt mode control 4"]
    #[inline(always)]
    pub fn krm4(&self) -> Krm4R {
        Krm4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key interrupt mode control 5"]
    #[inline(always)]
    pub fn krm5(&self) -> Krm5R {
        Krm5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Key interrupt mode control 6"]
    #[inline(always)]
    pub fn krm6(&self) -> Krm6R {
        Krm6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Key interrupt mode control 7"]
    #[inline(always)]
    pub fn krm7(&self) -> Krm7R {
        Krm7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KRM")
            .field("krm7", &self.krm7())
            .field("krm6", &self.krm6())
            .field("krm5", &self.krm5())
            .field("krm4", &self.krm4())
            .field("krm3", &self.krm3())
            .field("krm2", &self.krm2())
            .field("krm1", &self.krm1())
            .field("krm0", &self.krm0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Key interrupt mode control 0"]
    #[inline(always)]
    pub fn krm0(&mut self) -> Krm0W<KrmSpec> {
        Krm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Key interrupt mode control 1"]
    #[inline(always)]
    pub fn krm1(&mut self) -> Krm1W<KrmSpec> {
        Krm1W::new(self, 1)
    }
    #[doc = "Bit 2 - Key interrupt mode control 2"]
    #[inline(always)]
    pub fn krm2(&mut self) -> Krm2W<KrmSpec> {
        Krm2W::new(self, 2)
    }
    #[doc = "Bit 3 - Key interrupt mode control 3"]
    #[inline(always)]
    pub fn krm3(&mut self) -> Krm3W<KrmSpec> {
        Krm3W::new(self, 3)
    }
    #[doc = "Bit 4 - Key interrupt mode control 4"]
    #[inline(always)]
    pub fn krm4(&mut self) -> Krm4W<KrmSpec> {
        Krm4W::new(self, 4)
    }
    #[doc = "Bit 5 - Key interrupt mode control 5"]
    #[inline(always)]
    pub fn krm5(&mut self) -> Krm5W<KrmSpec> {
        Krm5W::new(self, 5)
    }
    #[doc = "Bit 6 - Key interrupt mode control 6"]
    #[inline(always)]
    pub fn krm6(&mut self) -> Krm6W<KrmSpec> {
        Krm6W::new(self, 6)
    }
    #[doc = "Bit 7 - Key interrupt mode control 7"]
    #[inline(always)]
    pub fn krm7(&mut self) -> Krm7W<KrmSpec> {
        Krm7W::new(self, 7)
    }
}
#[doc = "KEY Return Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrmSpec;
impl crate::RegisterSpec for KrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krm::R`](R) reader structure"]
impl crate::Readable for KrmSpec {}
#[doc = "`write(|w| ..)` method takes [`krm::W`](W) writer structure"]
impl crate::Writable for KrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KRM to value 0"]
impl crate::Resettable for KrmSpec {}

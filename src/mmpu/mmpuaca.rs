#[doc = "Register `MMPUACA%s` reader"]
pub type R = crate::R<MmpuacaSpec>;
#[doc = "Register `MMPUACA%s` writer"]
pub type W = crate::W<MmpuacaSpec>;
#[doc = "Region enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Group m Region n unit is disabled"]
    _0 = 0,
    #[doc = "1: Group m Region n unit is enabled"]
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Region enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    #[doc = "Group m Region n unit is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    #[doc = "Group m Region n unit is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
#[doc = "Field `ENABLE` writer - Region enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Group m Region n unit is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    #[doc = "Group m Region n unit is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rp {
    #[doc = "0: Read permission"]
    _0 = 0,
    #[doc = "1: Read protection"]
    _1 = 1,
}
impl From<Rp> for bool {
    #[inline(always)]
    fn from(variant: Rp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP` reader - Read protection"]
pub type RpR = crate::BitReader<Rp>;
impl RpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rp {
        match self.bits {
            false => Rp::_0,
            true => Rp::_1,
        }
    }
    #[doc = "Read permission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rp::_0
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rp::_1
    }
}
#[doc = "Field `RP` writer - Read protection"]
pub type RpW<'a, REG> = crate::BitWriter<'a, REG, Rp>;
impl<'a, REG> RpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rp::_0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rp::_1)
    }
}
#[doc = "Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp {
    #[doc = "0: Write permission"]
    _0 = 0,
    #[doc = "1: Write protection"]
    _1 = 1,
}
impl From<Wp> for bool {
    #[inline(always)]
    fn from(variant: Wp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP` reader - Write protection"]
pub type WpR = crate::BitReader<Wp>;
impl WpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp {
        match self.bits {
            false => Wp::_0,
            true => Wp::_1,
        }
    }
    #[doc = "Write permission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp::_0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp::_1
    }
}
#[doc = "Field `WP` writer - Write protection"]
pub type WpW<'a, REG> = crate::BitWriter<'a, REG, Wp>;
impl<'a, REG> WpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUACA")
            .field("wp", &self.wp())
            .field("rp", &self.rp())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MmpuacaSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rp(&mut self) -> RpW<MmpuacaSpec> {
        RpW::new(self, 1)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<MmpuacaSpec> {
        WpW::new(self, 2)
    }
}
#[doc = "Group A Region %s Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuaca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuaca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmpuacaSpec;
impl crate::RegisterSpec for MmpuacaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mmpuaca::R`](R) reader structure"]
impl crate::Readable for MmpuacaSpec {}
#[doc = "`write(|w| ..)` method takes [`mmpuaca::W`](W) writer structure"]
impl crate::Writable for MmpuacaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMPUACA%s to value 0"]
impl crate::Resettable for MmpuacaSpec {}

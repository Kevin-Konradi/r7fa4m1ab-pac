#[doc = "Register `AMPMC` reader"]
pub type R = crate::R<AmpmcSpec>;
#[doc = "Register `AMPMC` writer"]
pub type W = crate::W<AmpmcSpec>;
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amppc0 {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<Amppc0> for bool {
    #[inline(always)]
    fn from(variant: Amppc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPPC0` reader - Operational amplifier precharge control status"]
pub type Amppc0R = crate::BitReader<Amppc0>;
impl Amppc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amppc0 {
        match self.bits {
            false => Amppc0::_0,
            true => Amppc0::_1,
        }
    }
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amppc0::_0
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amppc0::_1
    }
}
#[doc = "Field `AMPPC0` writer - Operational amplifier precharge control status"]
pub type Amppc0W<'a, REG> = crate::BitWriter<'a, REG, Amppc0>;
impl<'a, REG> Amppc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc0::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc0::_1)
    }
}
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amppc1 {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<Amppc1> for bool {
    #[inline(always)]
    fn from(variant: Amppc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPPC1` reader - Operational amplifier precharge control status"]
pub type Amppc1R = crate::BitReader<Amppc1>;
impl Amppc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amppc1 {
        match self.bits {
            false => Amppc1::_0,
            true => Amppc1::_1,
        }
    }
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amppc1::_0
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amppc1::_1
    }
}
#[doc = "Field `AMPPC1` writer - Operational amplifier precharge control status"]
pub type Amppc1W<'a, REG> = crate::BitWriter<'a, REG, Amppc1>;
impl<'a, REG> Amppc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc1::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc1::_1)
    }
}
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amppc2 {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<Amppc2> for bool {
    #[inline(always)]
    fn from(variant: Amppc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPPC2` reader - Operational amplifier precharge control status"]
pub type Amppc2R = crate::BitReader<Amppc2>;
impl Amppc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amppc2 {
        match self.bits {
            false => Amppc2::_0,
            true => Amppc2::_1,
        }
    }
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amppc2::_0
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amppc2::_1
    }
}
#[doc = "Field `AMPPC2` writer - Operational amplifier precharge control status"]
pub type Amppc2W<'a, REG> = crate::BitWriter<'a, REG, Amppc2>;
impl<'a, REG> Amppc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc2::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc2::_1)
    }
}
#[doc = "Operational amplifier precharge control status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amppc3 {
    #[doc = "0: Precharging is stopped."]
    _0 = 0,
    #[doc = "1: Precharging is enabled."]
    _1 = 1,
}
impl From<Amppc3> for bool {
    #[inline(always)]
    fn from(variant: Amppc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPPC3` reader - Operational amplifier precharge control status"]
pub type Amppc3R = crate::BitReader<Amppc3>;
impl Amppc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amppc3 {
        match self.bits {
            false => Amppc3::_0,
            true => Amppc3::_1,
        }
    }
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Amppc3::_0
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Amppc3::_1
    }
}
#[doc = "Field `AMPPC3` writer - Operational amplifier precharge control status"]
pub type Amppc3W<'a, REG> = crate::BitWriter<'a, REG, Amppc3>;
impl<'a, REG> Amppc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharging is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc3::_0)
    }
    #[doc = "Precharging is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Amppc3::_1)
    }
}
#[doc = "Operation mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampsp {
    #[doc = "0: Low-power mode (low-speed)."]
    _0 = 0,
    #[doc = "1: High-speed mode."]
    _1 = 1,
}
impl From<Ampsp> for bool {
    #[inline(always)]
    fn from(variant: Ampsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPSP` reader - Operation mode selection"]
pub type AmpspR = crate::BitReader<Ampsp>;
impl AmpspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampsp {
        match self.bits {
            false => Ampsp::_0,
            true => Ampsp::_1,
        }
    }
    #[doc = "Low-power mode (low-speed)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampsp::_0
    }
    #[doc = "High-speed mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampsp::_1
    }
}
#[doc = "Field `AMPSP` writer - Operation mode selection"]
pub type AmpspW<'a, REG> = crate::BitWriter<'a, REG, Ampsp>;
impl<'a, REG> AmpspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-power mode (low-speed)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampsp::_0)
    }
    #[doc = "High-speed mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampsp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc0(&self) -> Amppc0R {
        Amppc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc1(&self) -> Amppc1R {
        Amppc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc2(&self) -> Amppc2R {
        Amppc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc3(&self) -> Amppc3R {
        Amppc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(&self) -> AmpspR {
        AmpspR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPMC")
            .field("ampsp", &self.ampsp())
            .field("amppc3", &self.amppc3())
            .field("amppc2", &self.amppc2())
            .field("amppc1", &self.amppc1())
            .field("amppc0", &self.amppc0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc0(&mut self) -> Amppc0W<AmpmcSpec> {
        Amppc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc1(&mut self) -> Amppc1W<AmpmcSpec> {
        Amppc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc2(&mut self) -> Amppc2W<AmpmcSpec> {
        Amppc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc3(&mut self) -> Amppc3W<AmpmcSpec> {
        Amppc3W::new(self, 3)
    }
    #[doc = "Bit 7 - Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(&mut self) -> AmpspW<AmpmcSpec> {
        AmpspW::new(self, 7)
    }
}
#[doc = "Operational amplifier mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpmcSpec;
impl crate::RegisterSpec for AmpmcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ampmc::R`](R) reader structure"]
impl crate::Readable for AmpmcSpec {}
#[doc = "`write(|w| ..)` method takes [`ampmc::W`](W) writer structure"]
impl crate::Writable for AmpmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMPMC to value 0"]
impl crate::Resettable for AmpmcSpec {}

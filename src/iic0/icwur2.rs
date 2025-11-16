#[doc = "Register `ICWUR2` reader"]
pub type R = crate::R<Icwur2Spec>;
#[doc = "Register `ICWUR2` writer"]
pub type W = crate::W<Icwur2Spec>;
#[doc = "Wake-up Function Synchronous Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wusen {
    #[doc = "0: IIC asynchronous circuit enable"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable"]
    _1 = 1,
}
impl From<Wusen> for bool {
    #[inline(always)]
    fn from(variant: Wusen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUSEN` reader - Wake-up Function Synchronous Enable"]
pub type WusenR = crate::BitReader<Wusen>;
impl WusenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wusen {
        match self.bits {
            false => Wusen::_0,
            true => Wusen::_1,
        }
    }
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wusen::_0
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wusen::_1
    }
}
#[doc = "Field `WUSEN` writer - Wake-up Function Synchronous Enable"]
pub type WusenW<'a, REG> = crate::BitWriter<'a, REG, Wusen>;
impl<'a, REG> WusenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wusen::_0)
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wusen::_1)
    }
}
#[doc = "Wake-up Function Asynchronous Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuasyf {
    #[doc = "0: IIC synchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC asynchronous circuit enable condition."]
    _1 = 1,
}
impl From<Wuasyf> for bool {
    #[inline(always)]
    fn from(variant: Wuasyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUASYF` reader - Wake-up Function Asynchronous Operation Status Flag"]
pub type WuasyfR = crate::BitReader<Wuasyf>;
impl WuasyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuasyf {
        match self.bits {
            false => Wuasyf::_0,
            true => Wuasyf::_1,
        }
    }
    #[doc = "IIC synchronous circuit enable condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuasyf::_0
    }
    #[doc = "IIC asynchronous circuit enable condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuasyf::_1
    }
}
#[doc = "Wake-up Function Synchronous Operation Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wusyf {
    #[doc = "0: IIC asynchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable condition."]
    _1 = 1,
}
impl From<Wusyf> for bool {
    #[inline(always)]
    fn from(variant: Wusyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUSYF` reader - Wake-up Function Synchronous Operation Status Flag"]
pub type WusyfR = crate::BitReader<Wusyf>;
impl WusyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wusyf {
        match self.bits {
            false => Wusyf::_0,
            true => Wusyf::_1,
        }
    }
    #[doc = "IIC asynchronous circuit enable condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wusyf::_0
    }
    #[doc = "IIC synchronous circuit enable condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wusyf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    pub fn wusen(&self) -> WusenR {
        WusenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Function Asynchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wuasyf(&self) -> WuasyfR {
        WuasyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Function Synchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wusyf(&self) -> WusyfR {
        WusyfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICWUR2")
            .field("wusyf", &self.wusyf())
            .field("wuasyf", &self.wuasyf())
            .field("wusen", &self.wusen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    pub fn wusen(&mut self) -> WusenW<Icwur2Spec> {
        WusenW::new(self, 0)
    }
}
#[doc = "I2C Bus Wake up Unit Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icwur2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icwur2Spec;
impl crate::RegisterSpec for Icwur2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icwur2::R`](R) reader structure"]
impl crate::Readable for Icwur2Spec {}
#[doc = "`write(|w| ..)` method takes [`icwur2::W`](W) writer structure"]
impl crate::Writable for Icwur2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICWUR2 to value 0xfd"]
impl crate::Resettable for Icwur2Spec {
    const RESET_VALUE: u8 = 0xfd;
}

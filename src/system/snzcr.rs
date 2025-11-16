#[doc = "Register `SNZCR` reader"]
pub type R = crate::R<SnzcrSpec>;
#[doc = "Register `SNZCR` writer"]
pub type W = crate::W<SnzcrSpec>;
#[doc = "RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdreqen {
    #[doc = "0: Ignore RXD0 falling edge in Software Standby mode."]
    _0 = 0,
    #[doc = "1: Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    _1 = 1,
}
impl From<Rxdreqen> for bool {
    #[inline(always)]
    fn from(variant: Rxdreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREQEN` reader - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
pub type RxdreqenR = crate::BitReader<Rxdreqen>;
impl RxdreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdreqen {
        match self.bits {
            false => Rxdreqen::_0,
            true => Rxdreqen::_1,
        }
    }
    #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdreqen::_0
    }
    #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdreqen::_1
    }
}
#[doc = "Field `RXDREQEN` writer - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
pub type RxdreqenW<'a, REG> = crate::BitWriter<'a, REG, Rxdreqen>;
impl<'a, REG> RxdreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdreqen::_0)
    }
    #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdreqen::_1)
    }
}
#[doc = "DTC Enable in Snooze Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzdtcen {
    #[doc = "0: Disable DTC operation"]
    _0 = 0,
    #[doc = "1: Enable DTC operation"]
    _1 = 1,
}
impl From<Snzdtcen> for bool {
    #[inline(always)]
    fn from(variant: Snzdtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZDTCEN` reader - DTC Enable in Snooze Mode"]
pub type SnzdtcenR = crate::BitReader<Snzdtcen>;
impl SnzdtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzdtcen {
        match self.bits {
            false => Snzdtcen::_0,
            true => Snzdtcen::_1,
        }
    }
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzdtcen::_0
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzdtcen::_1
    }
}
#[doc = "Field `SNZDTCEN` writer - DTC Enable in Snooze Mode"]
pub type SnzdtcenW<'a, REG> = crate::BitWriter<'a, REG, Snzdtcen>;
impl<'a, REG> SnzdtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_0)
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_1)
    }
}
#[doc = "Snooze Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snze {
    #[doc = "0: Disable Snooze Mode"]
    _0 = 0,
    #[doc = "1: Enable Snooze Mode"]
    _1 = 1,
}
impl From<Snze> for bool {
    #[inline(always)]
    fn from(variant: Snze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZE` reader - Snooze Mode Enable"]
pub type SnzeR = crate::BitReader<Snze>;
impl SnzeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snze {
        match self.bits {
            false => Snze::_0,
            true => Snze::_1,
        }
    }
    #[doc = "Disable Snooze Mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snze::_0
    }
    #[doc = "Enable Snooze Mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snze::_1
    }
}
#[doc = "Field `SNZE` writer - Snooze Mode Enable"]
pub type SnzeW<'a, REG> = crate::BitWriter<'a, REG, Snze>;
impl<'a, REG> SnzeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Snooze Mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_0)
    }
    #[doc = "Enable Snooze Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn rxdreqen(&self) -> RxdreqenR {
        RxdreqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub fn snzdtcen(&self) -> SnzdtcenR {
        SnzdtcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze Mode Enable"]
    #[inline(always)]
    pub fn snze(&self) -> SnzeR {
        SnzeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZCR")
            .field("snze", &self.snze())
            .field("snzdtcen", &self.snzdtcen())
            .field("rxdreqen", &self.rxdreqen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn rxdreqen(&mut self) -> RxdreqenW<SnzcrSpec> {
        RxdreqenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub fn snzdtcen(&mut self) -> SnzdtcenW<SnzcrSpec> {
        SnzdtcenW::new(self, 1)
    }
    #[doc = "Bit 7 - Snooze Mode Enable"]
    #[inline(always)]
    pub fn snze(&mut self) -> SnzeW<SnzcrSpec> {
        SnzeW::new(self, 7)
    }
}
#[doc = "Snooze Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnzcrSpec;
impl crate::RegisterSpec for SnzcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzcr::R`](R) reader structure"]
impl crate::Readable for SnzcrSpec {}
#[doc = "`write(|w| ..)` method takes [`snzcr::W`](W) writer structure"]
impl crate::Writable for SnzcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNZCR to value 0"]
impl crate::Resettable for SnzcrSpec {}

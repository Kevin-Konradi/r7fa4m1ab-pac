#[doc = "Register `ICWUR` reader"]
pub type R = crate::R<IcwurSpec>;
#[doc = "Register `ICWUR` writer"]
pub type W = crate::W<IcwurSpec>;
#[doc = "Wakeup Analog Filter Additional Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuafa {
    #[doc = "0: Do not add the wakeup analog filter"]
    _0 = 0,
    #[doc = "1: Add the wakeup analog filter."]
    _1 = 1,
}
impl From<Wuafa> for bool {
    #[inline(always)]
    fn from(variant: Wuafa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUAFA` reader - Wakeup Analog Filter Additional Selection"]
pub type WuafaR = crate::BitReader<Wuafa>;
impl WuafaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuafa {
        match self.bits {
            false => Wuafa::_0,
            true => Wuafa::_1,
        }
    }
    #[doc = "Do not add the wakeup analog filter"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuafa::_0
    }
    #[doc = "Add the wakeup analog filter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuafa::_1
    }
}
#[doc = "Field `WUAFA` writer - Wakeup Analog Filter Additional Selection"]
pub type WuafaW<'a, REG> = crate::BitWriter<'a, REG, Wuafa>;
impl<'a, REG> WuafaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not add the wakeup analog filter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuafa::_0)
    }
    #[doc = "Add the wakeup analog filter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuafa::_1)
    }
}
#[doc = "ACK bit for Wakeup Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuack {
    #[doc = "0: State of synchronous operation"]
    _0 = 0,
    #[doc = "1: State of asynchronous operation"]
    _1 = 1,
}
impl From<Wuack> for bool {
    #[inline(always)]
    fn from(variant: Wuack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUACK` reader - ACK bit for Wakeup Mode"]
pub type WuackR = crate::BitReader<Wuack>;
impl WuackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuack {
        match self.bits {
            false => Wuack::_0,
            true => Wuack::_1,
        }
    }
    #[doc = "State of synchronous operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuack::_0
    }
    #[doc = "State of asynchronous operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuack::_1
    }
}
#[doc = "Field `WUACK` writer - ACK bit for Wakeup Mode"]
pub type WuackW<'a, REG> = crate::BitWriter<'a, REG, Wuack>;
impl<'a, REG> WuackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "State of synchronous operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuack::_0)
    }
    #[doc = "State of asynchronous operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuack::_1)
    }
}
#[doc = "Wakeup Event Occurrence Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf {
    #[doc = "0: Slave address does not match during wakeup function"]
    _0 = 0,
    #[doc = "1: Slave address matches during wakeup function."]
    _1 = 1,
}
impl From<Wuf> for bool {
    #[inline(always)]
    fn from(variant: Wuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF` reader - Wakeup Event Occurrence Flag"]
pub type WufR = crate::BitReader<Wuf>;
impl WufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf {
        match self.bits {
            false => Wuf::_0,
            true => Wuf::_1,
        }
    }
    #[doc = "Slave address does not match during wakeup function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf::_0
    }
    #[doc = "Slave address matches during wakeup function."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf::_1
    }
}
#[doc = "Field `WUF` writer - Wakeup Event Occurrence Flag"]
pub type WufW<'a, REG> = crate::BitWriter<'a, REG, Wuf>;
impl<'a, REG> WufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address does not match during wakeup function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf::_0)
    }
    #[doc = "Slave address matches during wakeup function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf::_1)
    }
}
#[doc = "Wakeup Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuie {
    #[doc = "0: Wakeup Interrupt Request (IIC0_WUI) disabled"]
    _0 = 0,
    #[doc = "1: Wakeup Interrupt Request (IIC0_WUI) enabled."]
    _1 = 1,
}
impl From<Wuie> for bool {
    #[inline(always)]
    fn from(variant: Wuie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUIE` reader - Wakeup Interrupt Request Enable"]
pub type WuieR = crate::BitReader<Wuie>;
impl WuieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuie {
        match self.bits {
            false => Wuie::_0,
            true => Wuie::_1,
        }
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuie::_0
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuie::_1
    }
}
#[doc = "Field `WUIE` writer - Wakeup Interrupt Request Enable"]
pub type WuieW<'a, REG> = crate::BitWriter<'a, REG, Wuie>;
impl<'a, REG> WuieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::_0)
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuie::_1)
    }
}
#[doc = "Wakeup Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wue {
    #[doc = "0: Wakeup function disabled"]
    _0 = 0,
    #[doc = "1: Wakeup function enabled."]
    _1 = 1,
}
impl From<Wue> for bool {
    #[inline(always)]
    fn from(variant: Wue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUE` reader - Wakeup Function Enable"]
pub type WueR = crate::BitReader<Wue>;
impl WueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wue {
        match self.bits {
            false => Wue::_0,
            true => Wue::_1,
        }
    }
    #[doc = "Wakeup function disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wue::_0
    }
    #[doc = "Wakeup function enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wue::_1
    }
}
#[doc = "Field `WUE` writer - Wakeup Function Enable"]
pub type WueW<'a, REG> = crate::BitWriter<'a, REG, Wue>;
impl<'a, REG> WueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup function disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wue::_0)
    }
    #[doc = "Wakeup function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wue::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    pub fn wuafa(&self) -> WuafaR {
        WuafaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ACK bit for Wakeup Mode"]
    #[inline(always)]
    pub fn wuack(&self) -> WuackR {
        WuackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Event Occurrence Flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Interrupt Request Enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WuieR {
        WuieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Function Enable"]
    #[inline(always)]
    pub fn wue(&self) -> WueR {
        WueR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICWUR")
            .field("wue", &self.wue())
            .field("wuie", &self.wuie())
            .field("wuf", &self.wuf())
            .field("wuack", &self.wuack())
            .field("wuafa", &self.wuafa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    pub fn wuafa(&mut self) -> WuafaW<IcwurSpec> {
        WuafaW::new(self, 0)
    }
    #[doc = "Bit 4 - ACK bit for Wakeup Mode"]
    #[inline(always)]
    pub fn wuack(&mut self) -> WuackW<IcwurSpec> {
        WuackW::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup Event Occurrence Flag"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WufW<IcwurSpec> {
        WufW::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup Interrupt Request Enable"]
    #[inline(always)]
    pub fn wuie(&mut self) -> WuieW<IcwurSpec> {
        WuieW::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Function Enable"]
    #[inline(always)]
    pub fn wue(&mut self) -> WueW<IcwurSpec> {
        WueW::new(self, 7)
    }
}
#[doc = "I2C Bus Wake Up Unit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icwur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcwurSpec;
impl crate::RegisterSpec for IcwurSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icwur::R`](R) reader structure"]
impl crate::Readable for IcwurSpec {}
#[doc = "`write(|w| ..)` method takes [`icwur::W`](W) writer structure"]
impl crate::Writable for IcwurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICWUR to value 0x10"]
impl crate::Resettable for IcwurSpec {
    const RESET_VALUE: u8 = 0x10;
}

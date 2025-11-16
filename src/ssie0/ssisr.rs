#[doc = "Register `SSISR` reader"]
pub type R = crate::R<SsisrSpec>;
#[doc = "Register `SSISR` writer"]
pub type W = crate::W<SsisrSpec>;
#[doc = "Idle Mode Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iirq {
    #[doc = "0: In the communication state"]
    _0 = 0,
    #[doc = "1: In the idle state"]
    _1 = 1,
}
impl From<Iirq> for bool {
    #[inline(always)]
    fn from(variant: Iirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIRQ` reader - Idle Mode Status Flag"]
pub type IirqR = crate::BitReader<Iirq>;
impl IirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iirq {
        match self.bits {
            false => Iirq::_0,
            true => Iirq::_1,
        }
    }
    #[doc = "In the communication state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iirq::_0
    }
    #[doc = "In the idle state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iirq::_1
    }
}
#[doc = "Receive Overflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roirq {
    #[doc = "0: No receive overflow error is generated"]
    _0 = 0,
    #[doc = "1: A receive overflow error is generated."]
    _1 = 1,
}
impl From<Roirq> for bool {
    #[inline(always)]
    fn from(variant: Roirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROIRQ` reader - Receive Overflow Error Status Flag"]
pub type RoirqR = crate::BitReader<Roirq>;
impl RoirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Roirq {
        match self.bits {
            false => Roirq::_0,
            true => Roirq::_1,
        }
    }
    #[doc = "No receive overflow error is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Roirq::_0
    }
    #[doc = "A receive overflow error is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Roirq::_1
    }
}
#[doc = "Field `ROIRQ` writer - Receive Overflow Error Status Flag"]
pub type RoirqW<'a, REG> = crate::BitWriter<'a, REG, Roirq>;
impl<'a, REG> RoirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive overflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Roirq::_0)
    }
    #[doc = "A receive overflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Roirq::_1)
    }
}
#[doc = "Receive Underflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ruirq {
    #[doc = "0: No receive underflow error is generated"]
    _0 = 0,
    #[doc = "1: A receive underflow error is generated."]
    _1 = 1,
}
impl From<Ruirq> for bool {
    #[inline(always)]
    fn from(variant: Ruirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUIRQ` reader - Receive Underflow Error Status Flag"]
pub type RuirqR = crate::BitReader<Ruirq>;
impl RuirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ruirq {
        match self.bits {
            false => Ruirq::_0,
            true => Ruirq::_1,
        }
    }
    #[doc = "No receive underflow error is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ruirq::_0
    }
    #[doc = "A receive underflow error is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ruirq::_1
    }
}
#[doc = "Field `RUIRQ` writer - Receive Underflow Error Status Flag"]
pub type RuirqW<'a, REG> = crate::BitWriter<'a, REG, Ruirq>;
impl<'a, REG> RuirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive underflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ruirq::_0)
    }
    #[doc = "A receive underflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ruirq::_1)
    }
}
#[doc = "Transmit Overflow Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toirq {
    #[doc = "0: No transmit overflow error is generated"]
    _0 = 0,
    #[doc = "1: A transmit overflow error is generated."]
    _1 = 1,
}
impl From<Toirq> for bool {
    #[inline(always)]
    fn from(variant: Toirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIRQ` reader - Transmit Overflow Error Status Flag"]
pub type ToirqR = crate::BitReader<Toirq>;
impl ToirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toirq {
        match self.bits {
            false => Toirq::_0,
            true => Toirq::_1,
        }
    }
    #[doc = "No transmit overflow error is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toirq::_0
    }
    #[doc = "A transmit overflow error is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toirq::_1
    }
}
#[doc = "Field `TOIRQ` writer - Transmit Overflow Error Status Flag"]
pub type ToirqW<'a, REG> = crate::BitWriter<'a, REG, Toirq>;
impl<'a, REG> ToirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmit overflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toirq::_0)
    }
    #[doc = "A transmit overflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toirq::_1)
    }
}
#[doc = "Transmit Underflow Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuirq {
    #[doc = "0: No transmit underflow error is generated"]
    _0 = 0,
    #[doc = "1: A transmit underflow error is generated."]
    _1 = 1,
}
impl From<Tuirq> for bool {
    #[inline(always)]
    fn from(variant: Tuirq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUIRQ` reader - Transmit Underflow Error Status flag"]
pub type TuirqR = crate::BitReader<Tuirq>;
impl TuirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tuirq {
        match self.bits {
            false => Tuirq::_0,
            true => Tuirq::_1,
        }
    }
    #[doc = "No transmit underflow error is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tuirq::_0
    }
    #[doc = "A transmit underflow error is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tuirq::_1
    }
}
#[doc = "Field `TUIRQ` writer - Transmit Underflow Error Status flag"]
pub type TuirqW<'a, REG> = crate::BitWriter<'a, REG, Tuirq>;
impl<'a, REG> TuirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmit underflow error is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tuirq::_0)
    }
    #[doc = "A transmit underflow error is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tuirq::_1)
    }
}
impl R {
    #[doc = "Bit 25 - Idle Mode Status Flag"]
    #[inline(always)]
    pub fn iirq(&self) -> IirqR {
        IirqR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Overflow Error Status Flag"]
    #[inline(always)]
    pub fn roirq(&self) -> RoirqR {
        RoirqR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Underflow Error Status Flag"]
    #[inline(always)]
    pub fn ruirq(&self) -> RuirqR {
        RuirqR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Overflow Error Status Flag"]
    #[inline(always)]
    pub fn toirq(&self) -> ToirqR {
        ToirqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Underflow Error Status flag"]
    #[inline(always)]
    pub fn tuirq(&self) -> TuirqR {
        TuirqR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSISR")
            .field("tuirq", &self.tuirq())
            .field("toirq", &self.toirq())
            .field("ruirq", &self.ruirq())
            .field("roirq", &self.roirq())
            .field("iirq", &self.iirq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 26 - Receive Overflow Error Status Flag"]
    #[inline(always)]
    pub fn roirq(&mut self) -> RoirqW<SsisrSpec> {
        RoirqW::new(self, 26)
    }
    #[doc = "Bit 27 - Receive Underflow Error Status Flag"]
    #[inline(always)]
    pub fn ruirq(&mut self) -> RuirqW<SsisrSpec> {
        RuirqW::new(self, 27)
    }
    #[doc = "Bit 28 - Transmit Overflow Error Status Flag"]
    #[inline(always)]
    pub fn toirq(&mut self) -> ToirqW<SsisrSpec> {
        ToirqW::new(self, 28)
    }
    #[doc = "Bit 29 - Transmit Underflow Error Status flag"]
    #[inline(always)]
    pub fn tuirq(&mut self) -> TuirqW<SsisrSpec> {
        TuirqW::new(self, 29)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsisrSpec;
impl crate::RegisterSpec for SsisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssisr::R`](R) reader structure"]
impl crate::Readable for SsisrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssisr::W`](W) writer structure"]
impl crate::Writable for SsisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSISR to value 0x0200_0000"]
impl crate::Resettable for SsisrSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}

#[doc = "Register `SPTR` reader"]
pub type R = crate::R<SptrSpec>;
#[doc = "Register `SPTR` writer"]
pub type W = crate::W<SptrSpec>;
#[doc = "Serial input data monitor bit (The state of the RXD terminal is shown.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmon {
    #[doc = "0: RXD pin is low."]
    _0 = 0,
    #[doc = "1: RXD pin is high."]
    _1 = 1,
}
impl From<Rxdmon> for bool {
    #[inline(always)]
    fn from(variant: Rxdmon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMON` reader - Serial input data monitor bit (The state of the RXD terminal is shown.)"]
pub type RxdmonR = crate::BitReader<Rxdmon>;
impl RxdmonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmon {
        match self.bits {
            false => Rxdmon::_0,
            true => Rxdmon::_1,
        }
    }
    #[doc = "RXD pin is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdmon::_0
    }
    #[doc = "RXD pin is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdmon::_1
    }
}
#[doc = "Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spb2dt {
    #[doc = "0: Low level is output on TXD pin"]
    _0 = 0,
    #[doc = "1: High level is output on TXD pin"]
    _1 = 1,
}
impl From<Spb2dt> for bool {
    #[inline(always)]
    fn from(variant: Spb2dt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPB2DT` reader - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
pub type Spb2dtR = crate::BitReader<Spb2dt>;
impl Spb2dtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spb2dt {
        match self.bits {
            false => Spb2dt::_0,
            true => Spb2dt::_1,
        }
    }
    #[doc = "Low level is output on TXD pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spb2dt::_0
    }
    #[doc = "High level is output on TXD pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spb2dt::_1
    }
}
#[doc = "Field `SPB2DT` writer - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
pub type Spb2dtW<'a, REG> = crate::BitWriter<'a, REG, Spb2dt>;
impl<'a, REG> Spb2dtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level is output on TXD pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2dt::_0)
    }
    #[doc = "High level is output on TXD pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2dt::_1)
    }
}
#[doc = "Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spb2io {
    #[doc = "0: The value of SPB2DT bit is not output in TXD pin."]
    _0 = 0,
    #[doc = "1: The value of SPB2DT bit is output in TXD pin."]
    _1 = 1,
}
impl From<Spb2io> for bool {
    #[inline(always)]
    fn from(variant: Spb2io) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPB2IO` reader - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
pub type Spb2ioR = crate::BitReader<Spb2io>;
impl Spb2ioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spb2io {
        match self.bits {
            false => Spb2io::_0,
            true => Spb2io::_1,
        }
    }
    #[doc = "The value of SPB2DT bit is not output in TXD pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spb2io::_0
    }
    #[doc = "The value of SPB2DT bit is output in TXD pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spb2io::_1
    }
}
#[doc = "Field `SPB2IO` writer - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
pub type Spb2ioW<'a, REG> = crate::BitWriter<'a, REG, Spb2io>;
impl<'a, REG> Spb2ioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value of SPB2DT bit is not output in TXD pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2io::_0)
    }
    #[doc = "The value of SPB2DT bit is output in TXD pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2io::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Serial input data monitor bit (The state of the RXD terminal is shown.)"]
    #[inline(always)]
    pub fn rxdmon(&self) -> RxdmonR {
        RxdmonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    pub fn spb2dt(&self) -> Spb2dtR {
        Spb2dtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    pub fn spb2io(&self) -> Spb2ioR {
        Spb2ioR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPTR")
            .field("spb2io", &self.spb2io())
            .field("spb2dt", &self.spb2dt())
            .field("rxdmon", &self.rxdmon())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    pub fn spb2dt(&mut self) -> Spb2dtW<SptrSpec> {
        Spb2dtW::new(self, 1)
    }
    #[doc = "Bit 2 - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    pub fn spb2io(&mut self) -> Spb2ioW<SptrSpec> {
        Spb2ioW::new(self, 2)
    }
}
#[doc = "Serial Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SptrSpec;
impl crate::RegisterSpec for SptrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sptr::R`](R) reader structure"]
impl crate::Readable for SptrSpec {}
#[doc = "`write(|w| ..)` method takes [`sptr::W`](W) writer structure"]
impl crate::Writable for SptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPTR to value 0x03"]
impl crate::Resettable for SptrSpec {
    const RESET_VALUE: u8 = 0x03;
}

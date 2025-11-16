#[doc = "Register `FTDRH` writer"]
pub type W = crate::W<FtdrhSpec>;
#[doc = "Field `TDATH` writer - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TdathW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpbt {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<Mpbt> for bool {
    #[inline(always)]
    fn from(variant: Mpbt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPBT` writer - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG, Mpbt>;
impl<'a, REG> MpbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_0)
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<FtdrhSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdath(&mut self) -> TdathW<FtdrhSpec> {
        TdathW::new(self, 0)
    }
    #[doc = "Bit 1 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<FtdrhSpec> {
        MpbtW::new(self, 1)
    }
}
#[doc = "Transmit FIFO Data Register H\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtdrhSpec;
impl crate::RegisterSpec for FtdrhSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ftdrh::W`](W) writer structure"]
impl crate::Writable for FtdrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTDRH to value 0xff"]
impl crate::Resettable for FtdrhSpec {
    const RESET_VALUE: u8 = 0xff;
}

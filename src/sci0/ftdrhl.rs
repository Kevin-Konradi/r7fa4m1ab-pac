#[doc = "Register `FTDRHL` writer"]
pub type W = crate::W<FtdrhlSpec>;
#[doc = "Field `TDAT` writer - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TdatW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
impl core::fmt::Debug for crate::generic::Reg<FtdrhlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:8 - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdat(&mut self) -> TdatW<FtdrhlSpec> {
        TdatW::new(self, 0)
    }
    #[doc = "Bit 9 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<FtdrhlSpec> {
        MpbtW::new(self, 9)
    }
}
#[doc = "Transmit FIFO Data Register HL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrhl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtdrhlSpec;
impl crate::RegisterSpec for FtdrhlSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`ftdrhl::W`](W) writer structure"]
impl crate::Writable for FtdrhlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTDRHL to value 0xffff"]
impl crate::Resettable for FtdrhlSpec {
    const RESET_VALUE: u16 = 0xffff;
}

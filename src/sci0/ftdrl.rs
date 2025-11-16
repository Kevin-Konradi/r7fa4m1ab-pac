#[doc = "Register `FTDRL` writer"]
pub type W = crate::W<FtdrlSpec>;
#[doc = "Field `TDATL` writer - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TdatlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<FtdrlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdatl(&mut self) -> TdatlW<FtdrlSpec> {
        TdatlW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Data Register L\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtdrlSpec;
impl crate::RegisterSpec for FtdrlSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ftdrl::W`](W) writer structure"]
impl crate::Writable for FtdrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTDRL to value 0xff"]
impl crate::Resettable for FtdrlSpec {
    const RESET_VALUE: u8 = 0xff;
}

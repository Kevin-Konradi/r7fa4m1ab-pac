#[doc = "Register `RFPCR` writer"]
pub type W = crate::W<RfpcrSpec>;
#[doc = "Field `RFPCR` writer - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
pub type RfpcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<RfpcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
    #[inline(always)]
    pub fn rfpcr(&mut self) -> RfpcrW<RfpcrSpec> {
        RfpcrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Pointer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfpcrSpec;
impl crate::RegisterSpec for RfpcrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`rfpcr::W`](W) writer structure"]
impl crate::Writable for RfpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFPCR to value 0"]
impl crate::Resettable for RfpcrSpec {}

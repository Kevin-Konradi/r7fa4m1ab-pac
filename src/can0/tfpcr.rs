#[doc = "Register `TFPCR` writer"]
pub type W = crate::W<TfpcrSpec>;
#[doc = "Field `TFPCR` writer - The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR."]
pub type TfpcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<TfpcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR."]
    #[inline(always)]
    pub fn tfpcr(&mut self) -> TfpcrW<TfpcrSpec> {
        TfpcrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Pointer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfpcrSpec;
impl crate::RegisterSpec for TfpcrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`tfpcr::W`](W) writer structure"]
impl crate::Writable for TfpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFPCR to value 0"]
impl crate::Resettable for TfpcrSpec {}

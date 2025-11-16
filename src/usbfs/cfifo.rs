#[doc = "Register `CFIFO` reader"]
pub type R = crate::R<CfifoSpec>;
#[doc = "Register `CFIFO` writer"]
pub type W = crate::W<CfifoSpec>;
#[doc = "Field `FIFOPORT` reader - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FifoportR = crate::FieldReader<u16>;
#[doc = "Field `FIFOPORT` writer - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FifoportW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(&self) -> FifoportR {
        FifoportR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFIFO")
            .field("fifoport", &self.fifoport())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<CfifoSpec> {
        FifoportW::new(self, 0)
    }
}
#[doc = "CFIFO Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfifoSpec;
impl crate::RegisterSpec for CfifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfifo::R`](R) reader structure"]
impl crate::Readable for CfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfifo::W`](W) writer structure"]
impl crate::Writable for CfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFIFO to value 0"]
impl crate::Resettable for CfifoSpec {}

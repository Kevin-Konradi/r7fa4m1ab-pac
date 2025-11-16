#[doc = "Register `D1FIFO` reader"]
pub type R = crate::R<D1fifoSpec>;
#[doc = "Register `D1FIFO` writer"]
pub type W = crate::W<D1fifoSpec>;
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
        f.debug_struct("D1FIFO")
            .field("fifoport", &self.fifoport())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<D1fifoSpec> {
        FifoportW::new(self, 0)
    }
}
#[doc = "D1FIFO Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1fifoSpec;
impl crate::RegisterSpec for D1fifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`d1fifo::R`](R) reader structure"]
impl crate::Readable for D1fifoSpec {}
#[doc = "`write(|w| ..)` method takes [`d1fifo::W`](W) writer structure"]
impl crate::Writable for D1fifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1FIFO to value 0"]
impl crate::Resettable for D1fifoSpec {}

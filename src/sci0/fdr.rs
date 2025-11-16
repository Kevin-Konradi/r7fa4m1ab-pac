#[doc = "Register `FDR` reader"]
pub type R = crate::R<FdrSpec>;
#[doc = "Field `R` reader - Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
pub type RR = crate::FieldReader;
#[doc = "Field `T` reader - Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
pub type TR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDR")
            .field("t", &self.t())
            .field("r", &self.r())
            .finish()
    }
}
#[doc = "FIFO Data Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FdrSpec {}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FdrSpec {}

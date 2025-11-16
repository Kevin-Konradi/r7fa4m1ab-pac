#[doc = "Register `FRDRL` reader"]
pub type R = crate::R<FrdrlSpec>;
#[doc = "Field `RDATL` reader - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
pub type RdatlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
    #[inline(always)]
    pub fn rdatl(&self) -> RdatlR {
        RdatlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDRL")
            .field("rdatl", &self.rdatl())
            .finish()
    }
}
#[doc = "Receive FIFO Data Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`frdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrdrlSpec;
impl crate::RegisterSpec for FrdrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`frdrl::R`](R) reader structure"]
impl crate::Readable for FrdrlSpec {}
#[doc = "`reset()` method sets FRDRL to value 0"]
impl crate::Resettable for FrdrlSpec {}

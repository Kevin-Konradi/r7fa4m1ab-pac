#[doc = "Register `ICDRR` reader"]
pub type R = crate::R<IcdrrSpec>;
#[doc = "Field `ICDRR` reader - 8-bit register that stores the received data"]
pub type IcdrrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit register that stores the received data"]
    #[inline(always)]
    pub fn icdrr(&self) -> IcdrrR {
        IcdrrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICDRR")
            .field("icdrr", &self.icdrr())
            .finish()
    }
}
#[doc = "I2C Bus Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdrrSpec;
impl crate::RegisterSpec for IcdrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icdrr::R`](R) reader structure"]
impl crate::Readable for IcdrrSpec {}
#[doc = "`reset()` method sets ICDRR to value 0"]
impl crate::Resettable for IcdrrSpec {}

#[doc = "Register `TECR` reader"]
pub type R = crate::R<TecrSpec>;
#[doc = "Field `TECR` reader - Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
pub type TecrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[inline(always)]
    pub fn tecr(&self) -> TecrR {
        TecrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TECR").field("tecr", &self.tecr()).finish()
    }
}
#[doc = "Transmit Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TecrSpec;
impl crate::RegisterSpec for TecrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tecr::R`](R) reader structure"]
impl crate::Readable for TecrSpec {}
#[doc = "`reset()` method sets TECR to value 0"]
impl crate::Resettable for TecrSpec {}

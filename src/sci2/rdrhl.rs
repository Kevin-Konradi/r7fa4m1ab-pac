#[doc = "Register `RDRHL` reader"]
pub type R = crate::R<RdrhlSpec>;
#[doc = "Field `RDRHL` reader - RDRHL is an 16-bit register that stores receive data."]
pub type RdrhlR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RDRHL is an 16-bit register that stores receive data."]
    #[inline(always)]
    pub fn rdrhl(&self) -> RdrhlR {
        RdrhlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDRHL")
            .field("rdrhl", &self.rdrhl())
            .finish()
    }
}
#[doc = "Receive 9-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdrhl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrhlSpec;
impl crate::RegisterSpec for RdrhlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rdrhl::R`](R) reader structure"]
impl crate::Readable for RdrhlSpec {}
#[doc = "`reset()` method sets RDRHL to value 0"]
impl crate::Resettable for RdrhlSpec {}

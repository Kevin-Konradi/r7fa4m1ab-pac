#[doc = "Register `ADTSDR` reader"]
pub type R = crate::R<AdtsdrSpec>;
#[doc = "Field `ADTSDR` reader - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output."]
pub type AdtsdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output."]
    #[inline(always)]
    pub fn adtsdr(&self) -> AdtsdrR {
        AdtsdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADTSDR")
            .field("adtsdr", &self.adtsdr())
            .finish()
    }
}
#[doc = "A/D Temperature Sensor Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adtsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdtsdrSpec;
impl crate::RegisterSpec for AdtsdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adtsdr::R`](R) reader structure"]
impl crate::Readable for AdtsdrSpec {}
#[doc = "`reset()` method sets ADTSDR to value 0"]
impl crate::Resettable for AdtsdrSpec {}

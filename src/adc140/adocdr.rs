#[doc = "Register `ADOCDR` reader"]
pub type R = crate::R<AdocdrSpec>;
#[doc = "Field `ADOCDR` reader - This is a 16-bit read-only register for storing the A/D result of internal reference voltage."]
pub type AdocdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16-bit read-only register for storing the A/D result of internal reference voltage."]
    #[inline(always)]
    pub fn adocdr(&self) -> AdocdrR {
        AdocdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADOCDR")
            .field("adocdr", &self.adocdr())
            .finish()
    }
}
#[doc = "A/D Internal Reference Voltage Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adocdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdocdrSpec;
impl crate::RegisterSpec for AdocdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adocdr::R`](R) reader structure"]
impl crate::Readable for AdocdrSpec {}
#[doc = "`reset()` method sets ADOCDR to value 0"]
impl crate::Resettable for AdocdrSpec {}

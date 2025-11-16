#[doc = "Register `ADDBLDR` reader"]
pub type R = crate::R<AddbldrSpec>;
#[doc = "Field `ADDBLDR` reader - This is a 16-bit read-only register for storing the result of A/D conversion in response to the second trigger in double trigger mode."]
pub type AddbldrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16-bit read-only register for storing the result of A/D conversion in response to the second trigger in double trigger mode."]
    #[inline(always)]
    pub fn addbldr(&self) -> AddbldrR {
        AddbldrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDBLDR")
            .field("addbldr", &self.addbldr())
            .finish()
    }
}
#[doc = "A/D Data Duplication Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddbldrSpec;
impl crate::RegisterSpec for AddbldrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addbldr::R`](R) reader structure"]
impl crate::Readable for AddbldrSpec {}
#[doc = "`reset()` method sets ADDBLDR to value 0"]
impl crate::Resettable for AddbldrSpec {}

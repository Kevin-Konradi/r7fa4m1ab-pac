#[doc = "Register `BUS%sERRADD` reader"]
pub type R = crate::R<BuserraddSpec>;
#[doc = "Field `BERAD` reader - Bus Error Address When a bus error occurs, It stores an error address."]
pub type BeradR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error Address When a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub fn berad(&self) -> BeradR {
        BeradR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRADD")
            .field("berad", &self.berad())
            .finish()
    }
}
#[doc = "Bus Error Address Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`buserradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuserraddSpec;
impl crate::RegisterSpec for BuserraddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buserradd::R`](R) reader structure"]
impl crate::Readable for BuserraddSpec {}
#[doc = "`reset()` method sets BUS%sERRADD to value 0"]
impl crate::Resettable for BuserraddSpec {}

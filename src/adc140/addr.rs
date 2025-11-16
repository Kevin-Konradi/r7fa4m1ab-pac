#[doc = "Register `ADDR%s` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Field `ADDR` reader - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion."]
pub type AddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR").field("addr", &self.addr()).finish()
    }
}
#[doc = "A/D Data Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`reset()` method sets ADDR%s to value 0"]
impl crate::Resettable for AddrSpec {}

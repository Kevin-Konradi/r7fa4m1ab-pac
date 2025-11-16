#[doc = "Register `RSECCP%s` reader"]
pub type R = crate::R<RseccpSpec>;
#[doc = "Field `SEC1` reader - 1-Second Capture Capture value for the ones place of seconds"]
pub type Sec1R = crate::FieldReader;
#[doc = "Field `SEC10` reader - 10-Second Capture Capture value for the tens place of seconds"]
pub type Sec10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 1-Second Capture Capture value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Capture Capture value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSECCP")
            .field("sec10", &self.sec10())
            .field("sec1", &self.sec1())
            .finish()
    }
}
#[doc = "Second Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rseccp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RseccpSpec;
impl crate::RegisterSpec for RseccpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rseccp::R`](R) reader structure"]
impl crate::Readable for RseccpSpec {}
#[doc = "`reset()` method sets RSECCP%s to value 0"]
impl crate::Resettable for RseccpSpec {}

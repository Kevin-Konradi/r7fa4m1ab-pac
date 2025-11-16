#[doc = "Register `RMINCP%s` reader"]
pub type R = crate::R<RmincpSpec>;
#[doc = "Field `MIN1` reader - 1-Minute Capture Capture value for the ones place of minutes"]
pub type Min1R = crate::FieldReader;
#[doc = "Field `MIN10` reader - 10-Minute Capture Capture value for the tens place of minutes"]
pub type Min10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(&self) -> Min1R {
        Min1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(&self) -> Min10R {
        Min10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMINCP")
            .field("min10", &self.min10())
            .field("min1", &self.min1())
            .finish()
    }
}
#[doc = "Minute Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rmincp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmincpSpec;
impl crate::RegisterSpec for RmincpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmincp::R`](R) reader structure"]
impl crate::Readable for RmincpSpec {}
#[doc = "`reset()` method sets RMINCP%s to value 0"]
impl crate::Resettable for RmincpSpec {}

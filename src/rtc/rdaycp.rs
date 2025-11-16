#[doc = "Register `RDAYCP%s` reader"]
pub type R = crate::R<RdaycpSpec>;
#[doc = "Field `DATE1` reader - 1-Day Capture Capture value for the ones place of minutes"]
pub type Date1R = crate::FieldReader;
#[doc = "Field `DATE10` reader - 10-Day Capture Capture value for the tens place of minutes"]
pub type Date10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 1-Day Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Day Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYCP")
            .field("date10", &self.date10())
            .field("date1", &self.date1())
            .finish()
    }
}
#[doc = "Date Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rdaycp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdaycpSpec;
impl crate::RegisterSpec for RdaycpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdaycp::R`](R) reader structure"]
impl crate::Readable for RdaycpSpec {}
#[doc = "`reset()` method sets RDAYCP%s to value 0"]
impl crate::Resettable for RdaycpSpec {}

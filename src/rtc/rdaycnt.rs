#[doc = "Register `RDAYCNT` reader"]
pub type R = crate::R<RdaycntSpec>;
#[doc = "Register `RDAYCNT` writer"]
pub type W = crate::W<RdaycntSpec>;
#[doc = "Field `DATE1` reader - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
pub type Date1R = crate::FieldReader;
#[doc = "Field `DATE1` writer - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
pub type Date1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATE10` reader - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
pub type Date10R = crate::FieldReader;
#[doc = "Field `DATE10` writer - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
pub type Date10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYCNT")
            .field("date10", &self.date10())
            .field("date1", &self.date1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn date1(&mut self) -> Date1W<RdaycntSpec> {
        Date1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    pub fn date10(&mut self) -> Date10W<RdaycntSpec> {
        Date10W::new(self, 4)
    }
}
#[doc = "Day Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdaycntSpec;
impl crate::RegisterSpec for RdaycntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdaycnt::R`](R) reader structure"]
impl crate::Readable for RdaycntSpec {}
#[doc = "`write(|w| ..)` method takes [`rdaycnt::W`](W) writer structure"]
impl crate::Writable for RdaycntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDAYCNT to value 0"]
impl crate::Resettable for RdaycntSpec {}

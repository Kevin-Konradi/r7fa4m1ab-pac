#[doc = "Register `RMONCNT` reader"]
pub type R = crate::R<RmoncntSpec>;
#[doc = "Register `RMONCNT` writer"]
pub type W = crate::W<RmoncntSpec>;
#[doc = "Field `MON1` reader - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type Mon1R = crate::FieldReader;
#[doc = "Field `MON1` writer - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type Mon1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MON10` reader - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type Mon10R = crate::BitReader;
#[doc = "Field `MON10` writer - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type Mon10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn mon1(&self) -> Mon1R {
        Mon1R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub fn mon10(&self) -> Mon10R {
        Mon10R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMONCNT")
            .field("mon10", &self.mon10())
            .field("mon1", &self.mon1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn mon1(&mut self) -> Mon1W<RmoncntSpec> {
        Mon1W::new(self, 0)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub fn mon10(&mut self) -> Mon10W<RmoncntSpec> {
        Mon10W::new(self, 4)
    }
}
#[doc = "Month Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmoncntSpec;
impl crate::RegisterSpec for RmoncntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmoncnt::R`](R) reader structure"]
impl crate::Readable for RmoncntSpec {}
#[doc = "`write(|w| ..)` method takes [`rmoncnt::W`](W) writer structure"]
impl crate::Writable for RmoncntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMONCNT to value 0"]
impl crate::Resettable for RmoncntSpec {}

#[doc = "Register `RYRCNT` reader"]
pub type R = crate::R<RyrcntSpec>;
#[doc = "Register `RYRCNT` writer"]
pub type W = crate::W<RyrcntSpec>;
#[doc = "Field `YR1` reader - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
pub type Yr1R = crate::FieldReader;
#[doc = "Field `YR1` writer - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
pub type Yr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YR10` reader - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
pub type Yr10R = crate::FieldReader;
#[doc = "Field `YR10` writer - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
pub type Yr10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn yr1(&self) -> Yr1R {
        Yr1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    pub fn yr10(&self) -> Yr10R {
        Yr10R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RYRCNT")
            .field("yr10", &self.yr10())
            .field("yr1", &self.yr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn yr1(&mut self) -> Yr1W<RyrcntSpec> {
        Yr1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    pub fn yr10(&mut self) -> Yr10W<RyrcntSpec> {
        Yr10W::new(self, 4)
    }
}
#[doc = "Year Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ryrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RyrcntSpec;
impl crate::RegisterSpec for RyrcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ryrcnt::R`](R) reader structure"]
impl crate::Readable for RyrcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ryrcnt::W`](W) writer structure"]
impl crate::Writable for RyrcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RYRCNT to value 0"]
impl crate::Resettable for RyrcntSpec {}

#[doc = "Register `RYRAR` reader"]
pub type R = crate::R<RyrarSpec>;
#[doc = "Register `RYRAR` writer"]
pub type W = crate::W<RyrarSpec>;
#[doc = "Field `YR1` reader - 1 Year Value for the ones place of years"]
pub type Yr1R = crate::FieldReader;
#[doc = "Field `YR1` writer - 1 Year Value for the ones place of years"]
pub type Yr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YR10` reader - 10 Years Value for the tens place of years"]
pub type Yr10R = crate::FieldReader;
#[doc = "Field `YR10` writer - 10 Years Value for the tens place of years"]
pub type Yr10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    pub fn yr1(&self) -> Yr1R {
        Yr1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    pub fn yr10(&self) -> Yr10R {
        Yr10R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RYRAR")
            .field("yr10", &self.yr10())
            .field("yr1", &self.yr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    pub fn yr1(&mut self) -> Yr1W<RyrarSpec> {
        Yr1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    pub fn yr10(&mut self) -> Yr10W<RyrarSpec> {
        Yr10W::new(self, 4)
    }
}
#[doc = "Year Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ryrar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RyrarSpec;
impl crate::RegisterSpec for RyrarSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ryrar::R`](R) reader structure"]
impl crate::Readable for RyrarSpec {}
#[doc = "`write(|w| ..)` method takes [`ryrar::W`](W) writer structure"]
impl crate::Writable for RyrarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RYRAR to value 0"]
impl crate::Resettable for RyrarSpec {}

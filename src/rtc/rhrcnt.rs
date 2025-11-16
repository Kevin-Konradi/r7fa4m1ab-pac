#[doc = "Register `RHRCNT` reader"]
pub type R = crate::R<RhrcntSpec>;
#[doc = "Register `RHRCNT` writer"]
pub type W = crate::W<RhrcntSpec>;
#[doc = "Field `HR1` reader - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
pub type Hr1R = crate::FieldReader;
#[doc = "Field `HR1` writer - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
pub type Hr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HR10` reader - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
pub type Hr10R = crate::FieldReader;
#[doc = "Field `HR10` writer - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
pub type Hr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Time Counter Setting for a.m./p.m.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: a.m."]
    _0 = 0,
    #[doc = "1: p.m."]
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Time Counter Setting for a.m./p.m."]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    #[doc = "a.m."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    #[doc = "p.m."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
#[doc = "Field `PM` writer - Time Counter Setting for a.m./p.m."]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a.m."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    #[doc = "p.m."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn hr1(&self) -> Hr1R {
        Hr1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    pub fn hr10(&self) -> Hr10R {
        Hr10R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHRCNT")
            .field("pm", &self.pm())
            .field("hr10", &self.hr10())
            .field("hr1", &self.hr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn hr1(&mut self) -> Hr1W<RhrcntSpec> {
        Hr1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    pub fn hr10(&mut self) -> Hr10W<RhrcntSpec> {
        Hr10W::new(self, 4)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<RhrcntSpec> {
        PmW::new(self, 6)
    }
}
#[doc = "Hour Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RhrcntSpec;
impl crate::RegisterSpec for RhrcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rhrcnt::R`](R) reader structure"]
impl crate::Readable for RhrcntSpec {}
#[doc = "`write(|w| ..)` method takes [`rhrcnt::W`](W) writer structure"]
impl crate::Writable for RhrcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RHRCNT to value 0"]
impl crate::Resettable for RhrcntSpec {}

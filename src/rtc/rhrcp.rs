#[doc = "Register `RHRCP%s` reader"]
pub type R = crate::R<RhrcpSpec>;
#[doc = "Field `HR1` reader - 1-Minute Capture Capture value for the ones place of minutes"]
pub type Hr1R = crate::FieldReader;
#[doc = "Field `HR10` reader - 10-Minute Capture Capture value for the tens place of minutes"]
pub type Hr10R = crate::FieldReader;
#[doc = "A.m./p.m. select for time counter setting.\n\nValue on reset: 0"]
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
#[doc = "Field `PM` reader - A.m./p.m. select for time counter setting."]
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
impl R {
    #[doc = "Bits 0:3 - 1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn hr1(&self) -> Hr1R {
        Hr1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn hr10(&self) -> Hr10R {
        Hr10R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - A.m./p.m. select for time counter setting."]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHRCP")
            .field("pm", &self.pm())
            .field("hr10", &self.hr10())
            .field("hr1", &self.hr1())
            .finish()
    }
}
#[doc = "Hour Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rhrcp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RhrcpSpec;
impl crate::RegisterSpec for RhrcpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rhrcp::R`](R) reader structure"]
impl crate::Readable for RhrcpSpec {}
#[doc = "`reset()` method sets RHRCP%s to value 0"]
impl crate::Resettable for RhrcpSpec {}

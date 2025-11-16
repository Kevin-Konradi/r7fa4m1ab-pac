#[doc = "Register `FLWT` reader"]
pub type R = crate::R<FlwtSpec>;
#[doc = "Register `FLWT` writer"]
pub type W = crate::W<FlwtSpec>;
#[doc = "These bits represent the ratio of the CPU clock period to the Flash memory access time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flwt {
    #[doc = "0: zero wait"]
    _000 = 0,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Flwt> for u8 {
    #[inline(always)]
    fn from(variant: Flwt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flwt {
    type Ux = u8;
}
impl crate::IsEnum for Flwt {}
#[doc = "Field `FLWT` reader - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FlwtR = crate::FieldReader<Flwt>;
impl FlwtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flwt {
        match self.bits {
            0 => Flwt::_000,
            _ => Flwt::Others,
        }
    }
    #[doc = "zero wait"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Flwt::_000
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Flwt::Others)
    }
}
#[doc = "Field `FLWT` writer - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FlwtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flwt, crate::Safe>;
impl<'a, REG> FlwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "zero wait"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Flwt::_000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Flwt::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    pub fn flwt(&self) -> FlwtR {
        FlwtR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLWT").field("flwt", &self.flwt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    pub fn flwt(&mut self) -> FlwtW<FlwtSpec> {
        FlwtW::new(self, 0)
    }
}
#[doc = "Flash Wait Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlwtSpec;
impl crate::RegisterSpec for FlwtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flwt::R`](R) reader structure"]
impl crate::Readable for FlwtSpec {}
#[doc = "`write(|w| ..)` method takes [`flwt::W`](W) writer structure"]
impl crate::Writable for FlwtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLWT to value 0"]
impl crate::Resettable for FlwtSpec {}

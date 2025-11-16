#[doc = "Register `ADCMPBSR` reader"]
pub type R = crate::R<AdcmpbsrSpec>;
#[doc = "Register `ADCMPBSR` writer"]
pub type W = crate::W<AdcmpbsrSpec>;
#[doc = "Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstb {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstb> for bool {
    #[inline(always)]
    fn from(variant: Cmpstb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTB` reader - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CmpstbR = crate::BitReader<Cmpstb>;
impl CmpstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstb {
        match self.bits {
            false => Cmpstb::_0,
            true => Cmpstb::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstb::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstb::_1
    }
}
#[doc = "Field `CMPSTB` writer - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
pub type CmpstbW<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstb>;
impl<'a, REG> CmpstbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstb::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstb::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    pub fn cmpstb(&self) -> CmpstbR {
        CmpstbR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPBSR").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    pub fn cmpstb(&mut self) -> CmpstbW<AdcmpbsrSpec> {
        CmpstbW::new(self, 0)
    }
}
#[doc = "A/D Compare Function Window B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmpbsrSpec;
impl crate::RegisterSpec for AdcmpbsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpbsr::R`](R) reader structure"]
impl crate::Readable for AdcmpbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpbsr::W`](W) writer structure"]
impl crate::Writable for AdcmpbsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets ADCMPBSR to value 0"]
impl crate::Resettable for AdcmpbsrSpec {}

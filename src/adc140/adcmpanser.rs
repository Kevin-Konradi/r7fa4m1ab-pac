#[doc = "Register `ADCMPANSER` reader"]
pub type R = crate::R<AdcmpanserSpec>;
#[doc = "Register `ADCMPANSER` writer"]
pub type W = crate::W<AdcmpanserSpec>;
#[doc = "Temperature sensor output Compare selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmptsa {
    #[doc = "0: Excludes the temperature sensor output from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes the temperature sensor output in the compare window A target range."]
    _1 = 1,
}
impl From<Cmptsa> for bool {
    #[inline(always)]
    fn from(variant: Cmptsa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTSA` reader - Temperature sensor output Compare selection bit."]
pub type CmptsaR = crate::BitReader<Cmptsa>;
impl CmptsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmptsa {
        match self.bits {
            false => Cmptsa::_0,
            true => Cmptsa::_1,
        }
    }
    #[doc = "Excludes the temperature sensor output from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmptsa::_0
    }
    #[doc = "Includes the temperature sensor output in the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmptsa::_1
    }
}
#[doc = "Field `CMPTSA` writer - Temperature sensor output Compare selection bit."]
pub type CmptsaW<'a, REG> = crate::BitWriter<'a, REG, Cmptsa>;
impl<'a, REG> CmptsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes the temperature sensor output from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptsa::_0)
    }
    #[doc = "Includes the temperature sensor output in the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptsa::_1)
    }
}
#[doc = "Internal reference voltage Compare selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpoca {
    #[doc = "0: Excludes the internal reference voltage from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes the internal reference voltage in the compare window A target range."]
    _1 = 1,
}
impl From<Cmpoca> for bool {
    #[inline(always)]
    fn from(variant: Cmpoca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOCA` reader - Internal reference voltage Compare selection bit."]
pub type CmpocaR = crate::BitReader<Cmpoca>;
impl CmpocaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpoca {
        match self.bits {
            false => Cmpoca::_0,
            true => Cmpoca::_1,
        }
    }
    #[doc = "Excludes the internal reference voltage from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpoca::_0
    }
    #[doc = "Includes the internal reference voltage in the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpoca::_1
    }
}
#[doc = "Field `CMPOCA` writer - Internal reference voltage Compare selection bit."]
pub type CmpocaW<'a, REG> = crate::BitWriter<'a, REG, Cmpoca>;
impl<'a, REG> CmpocaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes the internal reference voltage from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoca::_0)
    }
    #[doc = "Includes the internal reference voltage in the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoca::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature sensor output Compare selection bit."]
    #[inline(always)]
    pub fn cmptsa(&self) -> CmptsaR {
        CmptsaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal reference voltage Compare selection bit."]
    #[inline(always)]
    pub fn cmpoca(&self) -> CmpocaR {
        CmpocaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPANSER")
            .field("cmpoca", &self.cmpoca())
            .field("cmptsa", &self.cmptsa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor output Compare selection bit."]
    #[inline(always)]
    pub fn cmptsa(&mut self) -> CmptsaW<AdcmpanserSpec> {
        CmptsaW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal reference voltage Compare selection bit."]
    #[inline(always)]
    pub fn cmpoca(&mut self) -> CmpocaW<AdcmpanserSpec> {
        CmpocaW::new(self, 1)
    }
}
#[doc = "A/D Compare Function Window A Extended Input Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmpanserSpec;
impl crate::RegisterSpec for AdcmpanserSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpanser::R`](R) reader structure"]
impl crate::Readable for AdcmpanserSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpanser::W`](W) writer structure"]
impl crate::Writable for AdcmpanserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPANSER to value 0"]
impl crate::Resettable for AdcmpanserSpec {}

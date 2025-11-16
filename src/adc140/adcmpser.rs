#[doc = "Register `ADCMPSER` reader"]
pub type R = crate::R<AdcmpserSpec>;
#[doc = "Register `ADCMPSER` writer"]
pub type W = crate::W<AdcmpserSpec>;
#[doc = "Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpsttsa {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpsttsa> for bool {
    #[inline(always)]
    fn from(variant: Cmpsttsa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTTSA` reader - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CmpsttsaR = crate::BitReader<Cmpsttsa>;
impl CmpsttsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpsttsa {
        match self.bits {
            false => Cmpsttsa::_0,
            true => Cmpsttsa::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpsttsa::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpsttsa::_1
    }
}
#[doc = "Field `CMPSTTSA` writer - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
pub type CmpsttsaW<'a, REG> = crate::BitWriter0C<'a, REG, Cmpsttsa>;
impl<'a, REG> CmpsttsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsttsa::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsttsa::_1)
    }
}
#[doc = "Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstoca {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<Cmpstoca> for bool {
    #[inline(always)]
    fn from(variant: Cmpstoca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPSTOCA` reader - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CmpstocaR = crate::BitReader<Cmpstoca>;
impl CmpstocaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstoca {
        match self.bits {
            false => Cmpstoca::_0,
            true => Cmpstoca::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstoca::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstoca::_1
    }
}
#[doc = "Field `CMPSTOCA` writer - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
pub type CmpstocaW<'a, REG> = crate::BitWriter0C<'a, REG, Cmpstoca>;
impl<'a, REG> CmpstocaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstoca::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstoca::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpsttsa(&self) -> CmpsttsaR {
        CmpsttsaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpstoca(&self) -> CmpstocaR {
        CmpstocaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSER").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpsttsa(&mut self) -> CmpsttsaW<AdcmpserSpec> {
        CmpsttsaW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpstoca(&mut self) -> CmpstocaW<AdcmpserSpec> {
        CmpstocaW::new(self, 1)
    }
}
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmpserSpec;
impl crate::RegisterSpec for AdcmpserSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpser::R`](R) reader structure"]
impl crate::Readable for AdcmpserSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpser::W`](W) writer structure"]
impl crate::Writable for AdcmpserSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets ADCMPSER to value 0"]
impl crate::Resettable for AdcmpserSpec {}

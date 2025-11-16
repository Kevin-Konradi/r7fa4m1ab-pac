#[doc = "Register `ADEXICR` reader"]
pub type R = crate::R<AdexicrSpec>;
#[doc = "Register `ADEXICR` writer"]
pub type W = crate::W<AdexicrSpec>;
#[doc = "Temperature Sensor Output A/D converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssad {
    #[doc = "0: Temperature sensor output A/D-converted value addition/average mode is not selected."]
    _0 = 0,
    #[doc = "1: Temperature sensor output A/D-converted value addition/average mode is selected."]
    _1 = 1,
}
impl From<Tssad> for bool {
    #[inline(always)]
    fn from(variant: Tssad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSSAD` reader - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
pub type TssadR = crate::BitReader<Tssad>;
impl TssadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tssad {
        match self.bits {
            false => Tssad::_0,
            true => Tssad::_1,
        }
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tssad::_0
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tssad::_1
    }
}
#[doc = "Field `TSSAD` writer - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
pub type TssadW<'a, REG> = crate::BitWriter<'a, REG, Tssad>;
impl<'a, REG> TssadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tssad::_0)
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssad::_1)
    }
}
#[doc = "Internal Reference Voltage A/D converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocsad {
    #[doc = "0: Internal reference voltage A/D-converted value addition/average mode is not selected."]
    _0 = 0,
    #[doc = "1: Internal reference voltage A/D-converted value addition/average mode is selected."]
    _1 = 1,
}
impl From<Ocsad> for bool {
    #[inline(always)]
    fn from(variant: Ocsad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCSAD` reader - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
pub type OcsadR = crate::BitReader<Ocsad>;
impl OcsadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocsad {
        match self.bits {
            false => Ocsad::_0,
            true => Ocsad::_1,
        }
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocsad::_0
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocsad::_1
    }
}
#[doc = "Field `OCSAD` writer - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
pub type OcsadW<'a, REG> = crate::BitWriter<'a, REG, Ocsad>;
impl<'a, REG> OcsadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsad::_0)
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsad::_1)
    }
}
#[doc = "Temperature Sensor Output A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssa {
    #[doc = "0: The temperature sensor output is not selected."]
    _0 = 0,
    #[doc = "1: The temperature sensor output is selected."]
    _1 = 1,
}
impl From<Tssa> for bool {
    #[inline(always)]
    fn from(variant: Tssa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSSA` reader - Temperature Sensor Output A/D Conversion Select"]
pub type TssaR = crate::BitReader<Tssa>;
impl TssaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tssa {
        match self.bits {
            false => Tssa::_0,
            true => Tssa::_1,
        }
    }
    #[doc = "The temperature sensor output is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tssa::_0
    }
    #[doc = "The temperature sensor output is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tssa::_1
    }
}
#[doc = "Field `TSSA` writer - Temperature Sensor Output A/D Conversion Select"]
pub type TssaW<'a, REG> = crate::BitWriter<'a, REG, Tssa>;
impl<'a, REG> TssaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The temperature sensor output is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tssa::_0)
    }
    #[doc = "The temperature sensor output is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssa::_1)
    }
}
#[doc = "Internal Reference Voltage A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocsa {
    #[doc = "0: The internal reference voltage is not selected."]
    _0 = 0,
    #[doc = "1: The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    _1 = 1,
}
impl From<Ocsa> for bool {
    #[inline(always)]
    fn from(variant: Ocsa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCSA` reader - Internal Reference Voltage A/D Conversion Select"]
pub type OcsaR = crate::BitReader<Ocsa>;
impl OcsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocsa {
        match self.bits {
            false => Ocsa::_0,
            true => Ocsa::_1,
        }
    }
    #[doc = "The internal reference voltage is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocsa::_0
    }
    #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocsa::_1
    }
}
#[doc = "Field `OCSA` writer - Internal Reference Voltage A/D Conversion Select"]
pub type OcsaW<'a, REG> = crate::BitWriter<'a, REG, Ocsa>;
impl<'a, REG> OcsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The internal reference voltage is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsa::_0)
    }
    #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsa::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn tssad(&self) -> TssadR {
        TssadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(&self) -> OcsadR {
        OcsadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(&self) -> TssaR {
        TssaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(&self) -> OcsaR {
        OcsaR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADEXICR")
            .field("ocsa", &self.ocsa())
            .field("tssa", &self.tssa())
            .field("ocsad", &self.ocsad())
            .field("tssad", &self.tssad())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn tssad(&mut self) -> TssadW<AdexicrSpec> {
        TssadW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(&mut self) -> OcsadW<AdexicrSpec> {
        OcsadW::new(self, 1)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(&mut self) -> TssaW<AdexicrSpec> {
        TssaW::new(self, 8)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(&mut self) -> OcsaW<AdexicrSpec> {
        OcsaW::new(self, 9)
    }
}
#[doc = "A/D Conversion Extended Input Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adexicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdexicrSpec;
impl crate::RegisterSpec for AdexicrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adexicr::R`](R) reader structure"]
impl crate::Readable for AdexicrSpec {}
#[doc = "`write(|w| ..)` method takes [`adexicr::W`](W) writer structure"]
impl crate::Writable for AdexicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADEXICR to value 0"]
impl crate::Resettable for AdexicrSpec {}

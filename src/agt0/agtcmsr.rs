#[doc = "Register `AGTCMSR` reader"]
pub type R = crate::R<AgtcmsrSpec>;
#[doc = "Register `AGTCMSR` writer"]
pub type W = crate::W<AgtcmsrSpec>;
#[doc = "Compare match A register enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmea {
    #[doc = "0: Disable compare match A register"]
    _0 = 0,
    #[doc = "1: Enable compare match A register"]
    _1 = 1,
}
impl From<Tcmea> for bool {
    #[inline(always)]
    fn from(variant: Tcmea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMEA` reader - Compare match A register enable"]
pub type TcmeaR = crate::BitReader<Tcmea>;
impl TcmeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcmea {
        match self.bits {
            false => Tcmea::_0,
            true => Tcmea::_1,
        }
    }
    #[doc = "Disable compare match A register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmea::_0
    }
    #[doc = "Enable compare match A register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmea::_1
    }
}
#[doc = "Field `TCMEA` writer - Compare match A register enable"]
pub type TcmeaW<'a, REG> = crate::BitWriter<'a, REG, Tcmea>;
impl<'a, REG> TcmeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable compare match A register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmea::_0)
    }
    #[doc = "Enable compare match A register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmea::_1)
    }
}
#[doc = "AGTOA output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toea {
    #[doc = "0: AGTOA output disabled (port)"]
    _0 = 0,
    #[doc = "1: AGTOA output enabled"]
    _1 = 1,
}
impl From<Toea> for bool {
    #[inline(always)]
    fn from(variant: Toea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOEA` reader - AGTOA output enable"]
pub type ToeaR = crate::BitReader<Toea>;
impl ToeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toea {
        match self.bits {
            false => Toea::_0,
            true => Toea::_1,
        }
    }
    #[doc = "AGTOA output disabled (port)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toea::_0
    }
    #[doc = "AGTOA output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toea::_1
    }
}
#[doc = "Field `TOEA` writer - AGTOA output enable"]
pub type ToeaW<'a, REG> = crate::BitWriter<'a, REG, Toea>;
impl<'a, REG> ToeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGTOA output disabled (port)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toea::_0)
    }
    #[doc = "AGTOA output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toea::_1)
    }
}
#[doc = "AGTOA polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topola {
    #[doc = "0: AGTOA Output is started at low"]
    _0 = 0,
    #[doc = "1: AGTOA Output is started at high"]
    _1 = 1,
}
impl From<Topola> for bool {
    #[inline(always)]
    fn from(variant: Topola) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOPOLA` reader - AGTOA polarity select"]
pub type TopolaR = crate::BitReader<Topola>;
impl TopolaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Topola {
        match self.bits {
            false => Topola::_0,
            true => Topola::_1,
        }
    }
    #[doc = "AGTOA Output is started at low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Topola::_0
    }
    #[doc = "AGTOA Output is started at high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Topola::_1
    }
}
#[doc = "Field `TOPOLA` writer - AGTOA polarity select"]
pub type TopolaW<'a, REG> = crate::BitWriter<'a, REG, Topola>;
impl<'a, REG> TopolaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGTOA Output is started at low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Topola::_0)
    }
    #[doc = "AGTOA Output is started at high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Topola::_1)
    }
}
#[doc = "Compare match B register enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmeb {
    #[doc = "0: Disable compare match B register"]
    _0 = 0,
    #[doc = "1: Enable compare match B register"]
    _1 = 1,
}
impl From<Tcmeb> for bool {
    #[inline(always)]
    fn from(variant: Tcmeb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMEB` reader - Compare match B register enable"]
pub type TcmebR = crate::BitReader<Tcmeb>;
impl TcmebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcmeb {
        match self.bits {
            false => Tcmeb::_0,
            true => Tcmeb::_1,
        }
    }
    #[doc = "Disable compare match B register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmeb::_0
    }
    #[doc = "Enable compare match B register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmeb::_1
    }
}
#[doc = "Field `TCMEB` writer - Compare match B register enable"]
pub type TcmebW<'a, REG> = crate::BitWriter<'a, REG, Tcmeb>;
impl<'a, REG> TcmebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable compare match B register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmeb::_0)
    }
    #[doc = "Enable compare match B register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmeb::_1)
    }
}
#[doc = "AGTOB output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toeb {
    #[doc = "0: AGTOB output disabled (port)"]
    _0 = 0,
    #[doc = "1: AGTOB output enabled"]
    _1 = 1,
}
impl From<Toeb> for bool {
    #[inline(always)]
    fn from(variant: Toeb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOEB` reader - AGTOB output enable"]
pub type ToebR = crate::BitReader<Toeb>;
impl ToebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toeb {
        match self.bits {
            false => Toeb::_0,
            true => Toeb::_1,
        }
    }
    #[doc = "AGTOB output disabled (port)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toeb::_0
    }
    #[doc = "AGTOB output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toeb::_1
    }
}
#[doc = "Field `TOEB` writer - AGTOB output enable"]
pub type ToebW<'a, REG> = crate::BitWriter<'a, REG, Toeb>;
impl<'a, REG> ToebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGTOB output disabled (port)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toeb::_0)
    }
    #[doc = "AGTOB output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toeb::_1)
    }
}
#[doc = "AGTOB polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topolb {
    #[doc = "0: AGTOB Output is started at low"]
    _0 = 0,
    #[doc = "1: AGTOB Output is started at high"]
    _1 = 1,
}
impl From<Topolb> for bool {
    #[inline(always)]
    fn from(variant: Topolb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOPOLB` reader - AGTOB polarity select"]
pub type TopolbR = crate::BitReader<Topolb>;
impl TopolbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Topolb {
        match self.bits {
            false => Topolb::_0,
            true => Topolb::_1,
        }
    }
    #[doc = "AGTOB Output is started at low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Topolb::_0
    }
    #[doc = "AGTOB Output is started at high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Topolb::_1
    }
}
#[doc = "Field `TOPOLB` writer - AGTOB polarity select"]
pub type TopolbW<'a, REG> = crate::BitWriter<'a, REG, Topolb>;
impl<'a, REG> TopolbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGTOB Output is started at low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Topolb::_0)
    }
    #[doc = "AGTOB Output is started at high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Topolb::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare match A register enable"]
    #[inline(always)]
    pub fn tcmea(&self) -> TcmeaR {
        TcmeaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGTOA output enable"]
    #[inline(always)]
    pub fn toea(&self) -> ToeaR {
        ToeaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AGTOA polarity select"]
    #[inline(always)]
    pub fn topola(&self) -> TopolaR {
        TopolaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare match B register enable"]
    #[inline(always)]
    pub fn tcmeb(&self) -> TcmebR {
        TcmebR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AGTOB output enable"]
    #[inline(always)]
    pub fn toeb(&self) -> ToebR {
        ToebR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AGTOB polarity select"]
    #[inline(always)]
    pub fn topolb(&self) -> TopolbR {
        TopolbR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCMSR")
            .field("topolb", &self.topolb())
            .field("toeb", &self.toeb())
            .field("tcmeb", &self.tcmeb())
            .field("topola", &self.topola())
            .field("toea", &self.toea())
            .field("tcmea", &self.tcmea())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare match A register enable"]
    #[inline(always)]
    pub fn tcmea(&mut self) -> TcmeaW<AgtcmsrSpec> {
        TcmeaW::new(self, 0)
    }
    #[doc = "Bit 1 - AGTOA output enable"]
    #[inline(always)]
    pub fn toea(&mut self) -> ToeaW<AgtcmsrSpec> {
        ToeaW::new(self, 1)
    }
    #[doc = "Bit 2 - AGTOA polarity select"]
    #[inline(always)]
    pub fn topola(&mut self) -> TopolaW<AgtcmsrSpec> {
        TopolaW::new(self, 2)
    }
    #[doc = "Bit 4 - Compare match B register enable"]
    #[inline(always)]
    pub fn tcmeb(&mut self) -> TcmebW<AgtcmsrSpec> {
        TcmebW::new(self, 4)
    }
    #[doc = "Bit 5 - AGTOB output enable"]
    #[inline(always)]
    pub fn toeb(&mut self) -> ToebW<AgtcmsrSpec> {
        ToebW::new(self, 5)
    }
    #[doc = "Bit 6 - AGTOB polarity select"]
    #[inline(always)]
    pub fn topolb(&mut self) -> TopolbW<AgtcmsrSpec> {
        TopolbW::new(self, 6)
    }
}
#[doc = "AGT Compare Match Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtcmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtcmsrSpec;
impl crate::RegisterSpec for AgtcmsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtcmsr::R`](R) reader structure"]
impl crate::Readable for AgtcmsrSpec {}
#[doc = "`write(|w| ..)` method takes [`agtcmsr::W`](W) writer structure"]
impl crate::Writable for AgtcmsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTCMSR to value 0"]
impl crate::Resettable for AgtcmsrSpec {}

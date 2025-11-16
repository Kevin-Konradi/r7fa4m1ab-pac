#[doc = "Register `CTSUCR1` reader"]
pub type R = crate::R<Ctsucr1Spec>;
#[doc = "Register `CTSUCR1` writer"]
pub type W = crate::W<Ctsucr1Spec>;
#[doc = "CTSU Power Supply Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsupon {
    #[doc = "0: Powered off the CTSU"]
    _0 = 0,
    #[doc = "1: Powered on the CTSU"]
    _1 = 1,
}
impl From<Ctsupon> for bool {
    #[inline(always)]
    fn from(variant: Ctsupon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUPON` reader - CTSU Power Supply Enable"]
pub type CtsuponR = crate::BitReader<Ctsupon>;
impl CtsuponR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsupon {
        match self.bits {
            false => Ctsupon::_0,
            true => Ctsupon::_1,
        }
    }
    #[doc = "Powered off the CTSU"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsupon::_0
    }
    #[doc = "Powered on the CTSU"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsupon::_1
    }
}
#[doc = "Field `CTSUPON` writer - CTSU Power Supply Enable"]
pub type CtsuponW<'a, REG> = crate::BitWriter<'a, REG, Ctsupon>;
impl<'a, REG> CtsuponW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powered off the CTSU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsupon::_0)
    }
    #[doc = "Powered on the CTSU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsupon::_1)
    }
}
#[doc = "CTSU LPF Capacitance Charging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsucsw {
    #[doc = "0: Turned off capacitance switch"]
    _0 = 0,
    #[doc = "1: Turned on capacitance switch"]
    _1 = 1,
}
impl From<Ctsucsw> for bool {
    #[inline(always)]
    fn from(variant: Ctsucsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUCSW` reader - CTSU LPF Capacitance Charging Control"]
pub type CtsucswR = crate::BitReader<Ctsucsw>;
impl CtsucswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsucsw {
        match self.bits {
            false => Ctsucsw::_0,
            true => Ctsucsw::_1,
        }
    }
    #[doc = "Turned off capacitance switch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsucsw::_0
    }
    #[doc = "Turned on capacitance switch"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsucsw::_1
    }
}
#[doc = "Field `CTSUCSW` writer - CTSU LPF Capacitance Charging Control"]
pub type CtsucswW<'a, REG> = crate::BitWriter<'a, REG, Ctsucsw>;
impl<'a, REG> CtsucswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turned off capacitance switch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsucsw::_0)
    }
    #[doc = "Turned on capacitance switch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsucsw::_1)
    }
}
#[doc = "CTSU Power Supply Operating Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuatune0 {
    #[doc = "0: Normal operating mode"]
    _0 = 0,
    #[doc = "1: Low-voltage operating mode"]
    _1 = 1,
}
impl From<Ctsuatune0> for bool {
    #[inline(always)]
    fn from(variant: Ctsuatune0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUATUNE0` reader - CTSU Power Supply Operating Mode Setting"]
pub type Ctsuatune0R = crate::BitReader<Ctsuatune0>;
impl Ctsuatune0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuatune0 {
        match self.bits {
            false => Ctsuatune0::_0,
            true => Ctsuatune0::_1,
        }
    }
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuatune0::_0
    }
    #[doc = "Low-voltage operating mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuatune0::_1
    }
}
#[doc = "Field `CTSUATUNE0` writer - CTSU Power Supply Operating Mode Setting"]
pub type Ctsuatune0W<'a, REG> = crate::BitWriter<'a, REG, Ctsuatune0>;
impl<'a, REG> Ctsuatune0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuatune0::_0)
    }
    #[doc = "Low-voltage operating mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuatune0::_1)
    }
}
#[doc = "CTSU Power Supply Capacity Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuatune1 {
    #[doc = "0: Normal output"]
    _0 = 0,
    #[doc = "1: High-current output"]
    _1 = 1,
}
impl From<Ctsuatune1> for bool {
    #[inline(always)]
    fn from(variant: Ctsuatune1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUATUNE1` reader - CTSU Power Supply Capacity Adjustment"]
pub type Ctsuatune1R = crate::BitReader<Ctsuatune1>;
impl Ctsuatune1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuatune1 {
        match self.bits {
            false => Ctsuatune1::_0,
            true => Ctsuatune1::_1,
        }
    }
    #[doc = "Normal output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuatune1::_0
    }
    #[doc = "High-current output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuatune1::_1
    }
}
#[doc = "Field `CTSUATUNE1` writer - CTSU Power Supply Capacity Adjustment"]
pub type Ctsuatune1W<'a, REG> = crate::BitWriter<'a, REG, Ctsuatune1>;
impl<'a, REG> Ctsuatune1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuatune1::_0)
    }
    #[doc = "High-current output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuatune1::_1)
    }
}
#[doc = "CTSU Operating Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuclk {
    #[doc = "0: PCLK"]
    _00 = 0,
    #[doc = "1: PCLK/2 (PCLK divided by 2)"]
    _01 = 1,
    #[doc = "2: PCLK/2 (PCLK divided by 4)"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Ctsuclk> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuclk {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuclk {}
#[doc = "Field `CTSUCLK` reader - CTSU Operating Clock Select"]
pub type CtsuclkR = crate::FieldReader<Ctsuclk>;
impl CtsuclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuclk {
        match self.bits {
            0 => Ctsuclk::_00,
            1 => Ctsuclk::_01,
            2 => Ctsuclk::_10,
            3 => Ctsuclk::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctsuclk::_00
    }
    #[doc = "PCLK/2 (PCLK divided by 2)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctsuclk::_01
    }
    #[doc = "PCLK/2 (PCLK divided by 4)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctsuclk::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctsuclk::_11
    }
}
#[doc = "Field `CTSUCLK` writer - CTSU Operating Clock Select"]
pub type CtsuclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctsuclk, crate::Safe>;
impl<'a, REG> CtsuclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclk::_00)
    }
    #[doc = "PCLK/2 (PCLK divided by 2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclk::_01)
    }
    #[doc = "PCLK/2 (PCLK divided by 4)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclk::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclk::_11)
    }
}
#[doc = "CTSU Measurement Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsumd {
    #[doc = "0: Self-capacitance single scan mode"]
    _00 = 0,
    #[doc = "1: Self-capacitance multi-scan mode"]
    _01 = 1,
    #[doc = "2: Mutual capacitance simple scan mode"]
    _10 = 2,
    #[doc = "3: Mutual capacitance full scan mode"]
    _11 = 3,
}
impl From<Ctsumd> for u8 {
    #[inline(always)]
    fn from(variant: Ctsumd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsumd {
    type Ux = u8;
}
impl crate::IsEnum for Ctsumd {}
#[doc = "Field `CTSUMD` reader - CTSU Measurement Mode Select"]
pub type CtsumdR = crate::FieldReader<Ctsumd>;
impl CtsumdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsumd {
        match self.bits {
            0 => Ctsumd::_00,
            1 => Ctsumd::_01,
            2 => Ctsumd::_10,
            3 => Ctsumd::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Self-capacitance single scan mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctsumd::_00
    }
    #[doc = "Self-capacitance multi-scan mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctsumd::_01
    }
    #[doc = "Mutual capacitance simple scan mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctsumd::_10
    }
    #[doc = "Mutual capacitance full scan mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctsumd::_11
    }
}
#[doc = "Field `CTSUMD` writer - CTSU Measurement Mode Select"]
pub type CtsumdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctsumd, crate::Safe>;
impl<'a, REG> CtsumdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Self-capacitance single scan mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsumd::_00)
    }
    #[doc = "Self-capacitance multi-scan mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsumd::_01)
    }
    #[doc = "Mutual capacitance simple scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsumd::_10)
    }
    #[doc = "Mutual capacitance full scan mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsumd::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Power Supply Enable"]
    #[inline(always)]
    pub fn ctsupon(&self) -> CtsuponR {
        CtsuponR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU LPF Capacitance Charging Control"]
    #[inline(always)]
    pub fn ctsucsw(&self) -> CtsucswR {
        CtsucswR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    pub fn ctsuatune0(&self) -> Ctsuatune0R {
        Ctsuatune0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTSU Power Supply Capacity Adjustment"]
    #[inline(always)]
    pub fn ctsuatune1(&self) -> Ctsuatune1R {
        Ctsuatune1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CTSU Operating Clock Select"]
    #[inline(always)]
    pub fn ctsuclk(&self) -> CtsuclkR {
        CtsuclkR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - CTSU Measurement Mode Select"]
    #[inline(always)]
    pub fn ctsumd(&self) -> CtsumdR {
        CtsumdR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCR1")
            .field("ctsumd", &self.ctsumd())
            .field("ctsuclk", &self.ctsuclk())
            .field("ctsuatune1", &self.ctsuatune1())
            .field("ctsuatune0", &self.ctsuatune0())
            .field("ctsucsw", &self.ctsucsw())
            .field("ctsupon", &self.ctsupon())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Power Supply Enable"]
    #[inline(always)]
    pub fn ctsupon(&mut self) -> CtsuponW<Ctsucr1Spec> {
        CtsuponW::new(self, 0)
    }
    #[doc = "Bit 1 - CTSU LPF Capacitance Charging Control"]
    #[inline(always)]
    pub fn ctsucsw(&mut self) -> CtsucswW<Ctsucr1Spec> {
        CtsucswW::new(self, 1)
    }
    #[doc = "Bit 2 - CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    pub fn ctsuatune0(&mut self) -> Ctsuatune0W<Ctsucr1Spec> {
        Ctsuatune0W::new(self, 2)
    }
    #[doc = "Bit 3 - CTSU Power Supply Capacity Adjustment"]
    #[inline(always)]
    pub fn ctsuatune1(&mut self) -> Ctsuatune1W<Ctsucr1Spec> {
        Ctsuatune1W::new(self, 3)
    }
    #[doc = "Bits 4:5 - CTSU Operating Clock Select"]
    #[inline(always)]
    pub fn ctsuclk(&mut self) -> CtsuclkW<Ctsucr1Spec> {
        CtsuclkW::new(self, 4)
    }
    #[doc = "Bits 6:7 - CTSU Measurement Mode Select"]
    #[inline(always)]
    pub fn ctsumd(&mut self) -> CtsumdW<Ctsucr1Spec> {
        CtsumdW::new(self, 6)
    }
}
#[doc = "CTSU Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsucr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsucr1Spec;
impl crate::RegisterSpec for Ctsucr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsucr1::R`](R) reader structure"]
impl crate::Readable for Ctsucr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsucr1::W`](W) writer structure"]
impl crate::Writable for Ctsucr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCR1 to value 0"]
impl crate::Resettable for Ctsucr1Spec {}

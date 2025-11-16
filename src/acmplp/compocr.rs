#[doc = "Register `COMPOCR` reader"]
pub type R = crate::R<CompocrSpec>;
#[doc = "Register `COMPOCR` writer"]
pub type W = crate::W<CompocrSpec>;
#[doc = "ACMPLP0 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0oe {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0oe> for bool {
    #[inline(always)]
    fn from(variant: C0oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0OE` reader - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0oeR = crate::BitReader<C0oe>;
impl C0oeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0oe {
        match self.bits {
            false => C0oe::_0,
            true => C0oe::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0oe::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0oe::_1
    }
}
#[doc = "Field `C0OE` writer - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0oeW<'a, REG> = crate::BitWriter<'a, REG, C0oe>;
impl<'a, REG> C0oeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0oe::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0oe::_1)
    }
}
#[doc = "ACMPLP0 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0op {
    #[doc = "0: Non inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C0op> for bool {
    #[inline(always)]
    fn from(variant: C0op) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0OP` reader - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0opR = crate::BitReader<C0op>;
impl C0opR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0op {
        match self.bits {
            false => C0op::_0,
            true => C0op::_1,
        }
    }
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0op::_0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0op::_1
    }
}
#[doc = "Field `C0OP` writer - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0opW<'a, REG> = crate::BitWriter<'a, REG, C0op>;
impl<'a, REG> C0opW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0op::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0op::_1)
    }
}
#[doc = "ACMPLP1 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1oe {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1oe> for bool {
    #[inline(always)]
    fn from(variant: C1oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1OE` reader - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1oeR = crate::BitReader<C1oe>;
impl C1oeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1oe {
        match self.bits {
            false => C1oe::_0,
            true => C1oe::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1oe::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1oe::_1
    }
}
#[doc = "Field `C1OE` writer - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1oeW<'a, REG> = crate::BitWriter<'a, REG, C1oe>;
impl<'a, REG> C1oeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1oe::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1oe::_1)
    }
}
#[doc = "ACMPLP1 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1op {
    #[doc = "0: Non inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C1op> for bool {
    #[inline(always)]
    fn from(variant: C1op) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1OP` reader - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1opR = crate::BitReader<C1op>;
impl C1opR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1op {
        match self.bits {
            false => C1op::_0,
            true => C1op::_1,
        }
    }
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1op::_0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1op::_1
    }
}
#[doc = "Field `C1OP` writer - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1opW<'a, REG> = crate::BitWriter<'a, REG, C1op>;
impl<'a, REG> C1opW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1op::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1op::_1)
    }
}
#[doc = "ACMPLP0/ACMPLP1 Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spdmd {
    #[doc = "0: Comparator low-speed mode"]
    _0 = 0,
    #[doc = "1: Comparator high-speed mode"]
    _1 = 1,
}
impl From<Spdmd> for bool {
    #[inline(always)]
    fn from(variant: Spdmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDMD` reader - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SpdmdR = crate::BitReader<Spdmd>;
impl SpdmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spdmd {
        match self.bits {
            false => Spdmd::_0,
            true => Spdmd::_1,
        }
    }
    #[doc = "Comparator low-speed mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spdmd::_0
    }
    #[doc = "Comparator high-speed mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spdmd::_1
    }
}
#[doc = "Field `SPDMD` writer - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SpdmdW<'a, REG> = crate::BitWriter<'a, REG, Spdmd>;
impl<'a, REG> SpdmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator low-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spdmd::_0)
    }
    #[doc = "Comparator high-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spdmd::_1)
    }
}
impl R {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c0oe(&self) -> C0oeR {
        C0oeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c0op(&self) -> C0opR {
        C0opR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c1oe(&self) -> C1oeR {
        C1oeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c1op(&self) -> C1opR {
        C1opR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub fn spdmd(&self) -> SpdmdR {
        SpdmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPOCR")
            .field("spdmd", &self.spdmd())
            .field("c1op", &self.c1op())
            .field("c1oe", &self.c1oe())
            .field("c0op", &self.c0op())
            .field("c0oe", &self.c0oe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c0oe(&mut self) -> C0oeW<CompocrSpec> {
        C0oeW::new(self, 1)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c0op(&mut self) -> C0opW<CompocrSpec> {
        C0opW::new(self, 2)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c1oe(&mut self) -> C1oeW<CompocrSpec> {
        C1oeW::new(self, 5)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c1op(&mut self) -> C1opW<CompocrSpec> {
        C1opW::new(self, 6)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub fn spdmd(&mut self) -> SpdmdW<CompocrSpec> {
        SpdmdW::new(self, 7)
    }
}
#[doc = "ACMPLP Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompocrSpec;
impl crate::RegisterSpec for CompocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compocr::R`](R) reader structure"]
impl crate::Readable for CompocrSpec {}
#[doc = "`write(|w| ..)` method takes [`compocr::W`](W) writer structure"]
impl crate::Writable for CompocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPOCR to value 0"]
impl crate::Resettable for CompocrSpec {}

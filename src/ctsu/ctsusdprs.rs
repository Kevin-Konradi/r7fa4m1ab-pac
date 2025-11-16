#[doc = "Register `CTSUSDPRS` reader"]
pub type R = crate::R<CtsusdprsSpec>;
#[doc = "Register `CTSUSDPRS` writer"]
pub type W = crate::W<CtsusdprsSpec>;
#[doc = "Field `CTSUPRRATIO` reader - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
pub type CtsuprratioR = crate::FieldReader;
#[doc = "Field `CTSUPRRATIO` writer - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
pub type CtsuprratioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "CTSU Base Period and Pulse Count Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuprmode {
    #[doc = "0: 510 pulses"]
    _00 = 0,
    #[doc = "1: 126 pulses"]
    _01 = 1,
    #[doc = "2: 62 pulses (recommended setting value)"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Ctsuprmode> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuprmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuprmode {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuprmode {}
#[doc = "Field `CTSUPRMODE` reader - CTSU Base Period and Pulse Count Setting"]
pub type CtsuprmodeR = crate::FieldReader<Ctsuprmode>;
impl CtsuprmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuprmode {
        match self.bits {
            0 => Ctsuprmode::_00,
            1 => Ctsuprmode::_01,
            2 => Ctsuprmode::_10,
            3 => Ctsuprmode::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "510 pulses"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctsuprmode::_00
    }
    #[doc = "126 pulses"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctsuprmode::_01
    }
    #[doc = "62 pulses (recommended setting value)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctsuprmode::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctsuprmode::_11
    }
}
#[doc = "Field `CTSUPRMODE` writer - CTSU Base Period and Pulse Count Setting"]
pub type CtsuprmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctsuprmode, crate::Safe>;
impl<'a, REG> CtsuprmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "510 pulses"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuprmode::_00)
    }
    #[doc = "126 pulses"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuprmode::_01)
    }
    #[doc = "62 pulses (recommended setting value)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuprmode::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuprmode::_11)
    }
}
#[doc = "CTSU High-Pass Noise Reduction Function Off Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsusoff {
    #[doc = "0: High-pass noise reduction function turned on"]
    _0 = 0,
    #[doc = "1: High-pass noise reduction function turned off"]
    _1 = 1,
}
impl From<Ctsusoff> for bool {
    #[inline(always)]
    fn from(variant: Ctsusoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUSOFF` reader - CTSU High-Pass Noise Reduction Function Off Setting"]
pub type CtsusoffR = crate::BitReader<Ctsusoff>;
impl CtsusoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsusoff {
        match self.bits {
            false => Ctsusoff::_0,
            true => Ctsusoff::_1,
        }
    }
    #[doc = "High-pass noise reduction function turned on"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsusoff::_0
    }
    #[doc = "High-pass noise reduction function turned off"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsusoff::_1
    }
}
#[doc = "Field `CTSUSOFF` writer - CTSU High-Pass Noise Reduction Function Off Setting"]
pub type CtsusoffW<'a, REG> = crate::BitWriter<'a, REG, Ctsusoff>;
impl<'a, REG> CtsusoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-pass noise reduction function turned on"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusoff::_0)
    }
    #[doc = "High-pass noise reduction function turned off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusoff::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[inline(always)]
    pub fn ctsuprratio(&self) -> CtsuprratioR {
        CtsuprratioR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - CTSU Base Period and Pulse Count Setting"]
    #[inline(always)]
    pub fn ctsuprmode(&self) -> CtsuprmodeR {
        CtsuprmodeR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting"]
    #[inline(always)]
    pub fn ctsusoff(&self) -> CtsusoffR {
        CtsusoffR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSDPRS")
            .field("ctsusoff", &self.ctsusoff())
            .field("ctsuprmode", &self.ctsuprmode())
            .field("ctsuprratio", &self.ctsuprratio())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[inline(always)]
    pub fn ctsuprratio(&mut self) -> CtsuprratioW<CtsusdprsSpec> {
        CtsuprratioW::new(self, 0)
    }
    #[doc = "Bits 4:5 - CTSU Base Period and Pulse Count Setting"]
    #[inline(always)]
    pub fn ctsuprmode(&mut self) -> CtsuprmodeW<CtsusdprsSpec> {
        CtsuprmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting"]
    #[inline(always)]
    pub fn ctsusoff(&mut self) -> CtsusoffW<CtsusdprsSpec> {
        CtsusoffW::new(self, 6)
    }
}
#[doc = "CTSU Synchronous Noise Reduction Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsusdprs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusdprs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsusdprsSpec;
impl crate::RegisterSpec for CtsusdprsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsusdprs::R`](R) reader structure"]
impl crate::Readable for CtsusdprsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctsusdprs::W`](W) writer structure"]
impl crate::Writable for CtsusdprsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUSDPRS to value 0"]
impl crate::Resettable for CtsusdprsSpec {}

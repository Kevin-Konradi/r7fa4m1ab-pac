#[doc = "Register `RCR1` reader"]
pub type R = crate::R<Rcr1Spec>;
#[doc = "Register `RCR1` writer"]
pub type W = crate::W<Rcr1Spec>;
#[doc = "Alarm Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aie {
    #[doc = "0: An alarm interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: An alarm interrupt request is enabled."]
    _1 = 1,
}
impl From<Aie> for bool {
    #[inline(always)]
    fn from(variant: Aie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIE` reader - Alarm Interrupt Enable"]
pub type AieR = crate::BitReader<Aie>;
impl AieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aie {
        match self.bits {
            false => Aie::_0,
            true => Aie::_1,
        }
    }
    #[doc = "An alarm interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aie::_0
    }
    #[doc = "An alarm interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aie::_1
    }
}
#[doc = "Field `AIE` writer - Alarm Interrupt Enable"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG, Aie>;
impl<'a, REG> AieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alarm interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aie::_0)
    }
    #[doc = "An alarm interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aie::_1)
    }
}
#[doc = "Carry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cie {
    #[doc = "0: A carry interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: A carry interrupt request is enabled."]
    _1 = 1,
}
impl From<Cie> for bool {
    #[inline(always)]
    fn from(variant: Cie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIE` reader - Carry Interrupt Enable"]
pub type CieR = crate::BitReader<Cie>;
impl CieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cie {
        match self.bits {
            false => Cie::_0,
            true => Cie::_1,
        }
    }
    #[doc = "A carry interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cie::_0
    }
    #[doc = "A carry interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cie::_1
    }
}
#[doc = "Field `CIE` writer - Carry Interrupt Enable"]
pub type CieW<'a, REG> = crate::BitWriter<'a, REG, Cie>;
impl<'a, REG> CieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A carry interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::_0)
    }
    #[doc = "A carry interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::_1)
    }
}
#[doc = "Periodic Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pie {
    #[doc = "0: A periodic interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: A periodic interrupt request is enabled."]
    _1 = 1,
}
impl From<Pie> for bool {
    #[inline(always)]
    fn from(variant: Pie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIE` reader - Periodic Interrupt Enable"]
pub type PieR = crate::BitReader<Pie>;
impl PieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pie {
        match self.bits {
            false => Pie::_0,
            true => Pie::_1,
        }
    }
    #[doc = "A periodic interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pie::_0
    }
    #[doc = "A periodic interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pie::_1
    }
}
#[doc = "Field `PIE` writer - Periodic Interrupt Enable"]
pub type PieW<'a, REG> = crate::BitWriter<'a, REG, Pie>;
impl<'a, REG> PieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A periodic interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pie::_0)
    }
    #[doc = "A periodic interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pie::_1)
    }
}
#[doc = "RTCOUT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcos {
    #[doc = "0: RTCOUT outputs 1 Hz."]
    _0 = 0,
    #[doc = "1: RTCOUT outputs 64 Hz."]
    _1 = 1,
}
impl From<Rtcos> for bool {
    #[inline(always)]
    fn from(variant: Rtcos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOS` reader - RTCOUT Output Select"]
pub type RtcosR = crate::BitReader<Rtcos>;
impl RtcosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcos {
        match self.bits {
            false => Rtcos::_0,
            true => Rtcos::_1,
        }
    }
    #[doc = "RTCOUT outputs 1 Hz."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcos::_0
    }
    #[doc = "RTCOUT outputs 64 Hz."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcos::_1
    }
}
#[doc = "Field `RTCOS` writer - RTCOUT Output Select"]
pub type RtcosW<'a, REG> = crate::BitWriter<'a, REG, Rtcos>;
impl<'a, REG> RtcosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCOUT outputs 1 Hz."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcos::_0)
    }
    #[doc = "RTCOUT outputs 64 Hz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcos::_1)
    }
}
#[doc = "Periodic Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pes {
    #[doc = "6: A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    _0110 = 6,
    #[doc = "7: A periodic interrupt is generated every 1/128 second."]
    _0111 = 7,
    #[doc = "8: A periodic interrupt is generated every 1/64 second."]
    _1000 = 8,
    #[doc = "9: A periodic interrupt is generated every 1/32 second."]
    _1001 = 9,
    #[doc = "10: A periodic interrupt is generated every 1/16 second."]
    _1010 = 10,
    #[doc = "11: A periodic interrupt is generated every 1/8 second."]
    _1011 = 11,
    #[doc = "12: A periodic interrupt is generated every 1/4 second."]
    _1100 = 12,
    #[doc = "13: A periodic interrupt is generated every 1/2 second."]
    _1101 = 13,
    #[doc = "14: A periodic interrupt is generated every 1 second."]
    _1110 = 14,
    #[doc = "15: A periodic interrupt is generated every 2 seconds."]
    _1111 = 15,
    #[doc = "0: No periodic interrupts are generated."]
    Others = 0,
}
impl From<Pes> for u8 {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pes {
    type Ux = u8;
}
impl crate::IsEnum for Pes {}
#[doc = "Field `PES` reader - Periodic Interrupt Select"]
pub type PesR = crate::FieldReader<Pes>;
impl PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            6 => Pes::_0110,
            7 => Pes::_0111,
            8 => Pes::_1000,
            9 => Pes::_1001,
            10 => Pes::_1010,
            11 => Pes::_1011,
            12 => Pes::_1100,
            13 => Pes::_1101,
            14 => Pes::_1110,
            15 => Pes::_1111,
            _ => Pes::Others,
        }
    }
    #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Pes::_0110
    }
    #[doc = "A periodic interrupt is generated every 1/128 second."]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Pes::_0111
    }
    #[doc = "A periodic interrupt is generated every 1/64 second."]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Pes::_1000
    }
    #[doc = "A periodic interrupt is generated every 1/32 second."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Pes::_1001
    }
    #[doc = "A periodic interrupt is generated every 1/16 second."]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Pes::_1010
    }
    #[doc = "A periodic interrupt is generated every 1/8 second."]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Pes::_1011
    }
    #[doc = "A periodic interrupt is generated every 1/4 second."]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Pes::_1100
    }
    #[doc = "A periodic interrupt is generated every 1/2 second."]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Pes::_1101
    }
    #[doc = "A periodic interrupt is generated every 1 second."]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Pes::_1110
    }
    #[doc = "A periodic interrupt is generated every 2 seconds."]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Pes::_1111
    }
    #[doc = "No periodic interrupts are generated."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pes::Others)
    }
}
#[doc = "Field `PES` writer - Periodic Interrupt Select"]
pub type PesW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pes, crate::Safe>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0110)
    }
    #[doc = "A periodic interrupt is generated every 1/128 second."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0111)
    }
    #[doc = "A periodic interrupt is generated every 1/64 second."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1000)
    }
    #[doc = "A periodic interrupt is generated every 1/32 second."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1001)
    }
    #[doc = "A periodic interrupt is generated every 1/16 second."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1010)
    }
    #[doc = "A periodic interrupt is generated every 1/8 second."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1011)
    }
    #[doc = "A periodic interrupt is generated every 1/4 second."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1100)
    }
    #[doc = "A periodic interrupt is generated every 1/2 second."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1101)
    }
    #[doc = "A periodic interrupt is generated every 1 second."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1110)
    }
    #[doc = "A periodic interrupt is generated every 2 seconds."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_1111)
    }
    #[doc = "No periodic interrupts are generated."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(&self) -> PieR {
        PieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(&self) -> RtcosR {
        RtcosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new((self.bits >> 4) & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR1")
            .field("pes", &self.pes())
            .field("rtcos", &self.rtcos())
            .field("pie", &self.pie())
            .field("cie", &self.cie())
            .field("aie", &self.aie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AieW<Rcr1Spec> {
        AieW::new(self, 0)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<Rcr1Spec> {
        CieW::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(&mut self) -> PieW<Rcr1Spec> {
        PieW::new(self, 2)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(&mut self) -> RtcosW<Rcr1Spec> {
        RtcosW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(&mut self) -> PesW<Rcr1Spec> {
        PesW::new(self, 4)
    }
}
#[doc = "RTC Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr1Spec;
impl crate::RegisterSpec for Rcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcr1::R`](R) reader structure"]
impl crate::Readable for Rcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr1::W`](W) writer structure"]
impl crate::Writable for Rcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCR1 to value 0"]
impl crate::Resettable for Rcr1Spec {}

#[doc = "Register `RTCCR%s` reader"]
pub type R = crate::R<RtccrSpec>;
#[doc = "Register `RTCCR%s` writer"]
pub type W = crate::W<RtccrSpec>;
#[doc = "Time Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcct {
    #[doc = "0: No event is detected."]
    _00 = 0,
    #[doc = "1: Rising edge is detected."]
    _01 = 1,
    #[doc = "2: Falling edge is detected."]
    _10 = 2,
    #[doc = "3: Both edges are detected."]
    _11 = 3,
}
impl From<Tcct> for u8 {
    #[inline(always)]
    fn from(variant: Tcct) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcct {
    type Ux = u8;
}
impl crate::IsEnum for Tcct {}
#[doc = "Field `TCCT` reader - Time Capture Control"]
pub type TcctR = crate::FieldReader<Tcct>;
impl TcctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcct {
        match self.bits {
            0 => Tcct::_00,
            1 => Tcct::_01,
            2 => Tcct::_10,
            3 => Tcct::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcct::_00
    }
    #[doc = "Rising edge is detected."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcct::_01
    }
    #[doc = "Falling edge is detected."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcct::_10
    }
    #[doc = "Both edges are detected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcct::_11
    }
}
#[doc = "Field `TCCT` writer - Time Capture Control"]
pub type TcctW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcct, crate::Safe>;
impl<'a, REG> TcctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_00)
    }
    #[doc = "Rising edge is detected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_01)
    }
    #[doc = "Falling edge is detected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_10)
    }
    #[doc = "Both edges are detected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_11)
    }
}
#[doc = "Time Capture Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcst {
    #[doc = "0: No event is detected."]
    _0 = 0,
    #[doc = "1: An event is detected."]
    _1 = 1,
}
impl From<Tcst> for bool {
    #[inline(always)]
    fn from(variant: Tcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCST` reader - Time Capture Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TcstR = crate::BitReader<Tcst>;
impl TcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcst {
        match self.bits {
            false => Tcst::_0,
            true => Tcst::_1,
        }
    }
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcst::_0
    }
    #[doc = "An event is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcst::_1
    }
}
#[doc = "Field `TCST` writer - Time Capture Status"]
pub type TcstW<'a, REG> = crate::BitWriter0C<'a, REG, Tcst>;
impl<'a, REG> TcstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_0)
    }
    #[doc = "An event is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_1)
    }
}
#[doc = "Time Capture Noise Filter Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcnf {
    #[doc = "0: The noise filter is off."]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: The noise filter is on (count source)."]
    _10 = 2,
    #[doc = "3: The noise filter is on (count source by divided by 32)."]
    _11 = 3,
}
impl From<Tcnf> for u8 {
    #[inline(always)]
    fn from(variant: Tcnf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcnf {
    type Ux = u8;
}
impl crate::IsEnum for Tcnf {}
#[doc = "Field `TCNF` reader - Time Capture Noise Filter Control"]
pub type TcnfR = crate::FieldReader<Tcnf>;
impl TcnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcnf {
        match self.bits {
            0 => Tcnf::_00,
            1 => Tcnf::_01,
            2 => Tcnf::_10,
            3 => Tcnf::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The noise filter is off."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcnf::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcnf::_01
    }
    #[doc = "The noise filter is on (count source)."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcnf::_10
    }
    #[doc = "The noise filter is on (count source by divided by 32)."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcnf::_11
    }
}
#[doc = "Field `TCNF` writer - Time Capture Noise Filter Control"]
pub type TcnfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcnf, crate::Safe>;
impl<'a, REG> TcnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The noise filter is off."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_01)
    }
    #[doc = "The noise filter is on (count source)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_10)
    }
    #[doc = "The noise filter is on (count source by divided by 32)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    pub fn tcct(&self) -> TcctR {
        TcctR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    pub fn tcst(&self) -> TcstR {
        TcstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    pub fn tcnf(&self) -> TcnfR {
        TcnfR::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCR")
            .field("tcnf", &self.tcnf())
            .field("tcct", &self.tcct())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    pub fn tcct(&mut self) -> TcctW<RtccrSpec> {
        TcctW::new(self, 0)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    pub fn tcst(&mut self) -> TcstW<RtccrSpec> {
        TcstW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    pub fn tcnf(&mut self) -> TcnfW<RtccrSpec> {
        TcnfW::new(self, 4)
    }
}
#[doc = "Time Capture Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccrSpec;
impl crate::RegisterSpec for RtccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccr::R`](R) reader structure"]
impl crate::Readable for RtccrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccr::W`](W) writer structure"]
impl crate::Writable for RtccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x04;
}
#[doc = "`reset()` method sets RTCCR%s to value 0"]
impl crate::Resettable for RtccrSpec {}

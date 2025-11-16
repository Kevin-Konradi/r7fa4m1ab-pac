#[doc = "Register `ADCMPBNSR` reader"]
pub type R = crate::R<AdcmpbnsrSpec>;
#[doc = "Register `ADCMPBNSR` writer"]
pub type W = crate::W<AdcmpbnsrSpec>;
#[doc = "Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpchb {
    #[doc = "0: AN000"]
    _0x00 = 0,
    #[doc = "1: AN001"]
    _0x01 = 1,
    #[doc = "2: AN002"]
    _0x02 = 2,
    #[doc = "3: AN003"]
    _0x03 = 3,
    #[doc = "4: AN004"]
    _0x04 = 4,
    #[doc = "5: AN005"]
    _0x05 = 5,
    #[doc = "6: AN006"]
    _0x06 = 6,
    #[doc = "7: AN007"]
    _0x07 = 7,
    #[doc = "8: AN008"]
    _0x08 = 8,
    #[doc = "9: AN009"]
    _0x09 = 9,
    #[doc = "10: AN010"]
    _0x0a = 10,
    #[doc = "11: AN011"]
    _0x0b = 11,
    #[doc = "12: AN012"]
    _0x0c = 12,
    #[doc = "13: AN013"]
    _0x0d = 13,
    #[doc = "14: AN014"]
    _0x0e = 14,
    #[doc = "15: AN015"]
    _0x0f = 15,
    #[doc = "16: AN016"]
    _0x10 = 16,
    #[doc = "17: AN017"]
    _0x11 = 17,
    #[doc = "18: AN018"]
    _0x12 = 18,
    #[doc = "19: AN019"]
    _0x13 = 19,
    #[doc = "20: AN020"]
    _0x14 = 20,
    #[doc = "21: AN021"]
    _0x15 = 21,
    #[doc = "22: AN022"]
    _0x16 = 22,
    #[doc = "23: AN023"]
    _0x17 = 23,
    #[doc = "24: AN024"]
    _0x18 = 24,
    #[doc = "25: AN025"]
    _0x19 = 25,
    #[doc = "26: AN026"]
    _0x1a = 26,
    #[doc = "27: AN027"]
    _0x1b = 27,
    #[doc = "32: Temperature sensor"]
    _0x20 = 32,
    #[doc = "33: Internal reference voltage"]
    _0x21 = 33,
    #[doc = "63: No channel is selected"]
    _0x3f = 63,
    #[doc = "28: Setting prohibited"]
    Others = 28,
}
impl From<Cmpchb> for u8 {
    #[inline(always)]
    fn from(variant: Cmpchb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpchb {
    type Ux = u8;
}
impl crate::IsEnum for Cmpchb {}
#[doc = "Field `CMPCHB` reader - Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
pub type CmpchbR = crate::FieldReader<Cmpchb>;
impl CmpchbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpchb {
        match self.bits {
            0 => Cmpchb::_0x00,
            1 => Cmpchb::_0x01,
            2 => Cmpchb::_0x02,
            3 => Cmpchb::_0x03,
            4 => Cmpchb::_0x04,
            5 => Cmpchb::_0x05,
            6 => Cmpchb::_0x06,
            7 => Cmpchb::_0x07,
            8 => Cmpchb::_0x08,
            9 => Cmpchb::_0x09,
            10 => Cmpchb::_0x0a,
            11 => Cmpchb::_0x0b,
            12 => Cmpchb::_0x0c,
            13 => Cmpchb::_0x0d,
            14 => Cmpchb::_0x0e,
            15 => Cmpchb::_0x0f,
            16 => Cmpchb::_0x10,
            17 => Cmpchb::_0x11,
            18 => Cmpchb::_0x12,
            19 => Cmpchb::_0x13,
            20 => Cmpchb::_0x14,
            21 => Cmpchb::_0x15,
            22 => Cmpchb::_0x16,
            23 => Cmpchb::_0x17,
            24 => Cmpchb::_0x18,
            25 => Cmpchb::_0x19,
            26 => Cmpchb::_0x1a,
            27 => Cmpchb::_0x1b,
            32 => Cmpchb::_0x20,
            33 => Cmpchb::_0x21,
            63 => Cmpchb::_0x3f,
            _ => Cmpchb::Others,
        }
    }
    #[doc = "AN000"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Cmpchb::_0x00
    }
    #[doc = "AN001"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Cmpchb::_0x01
    }
    #[doc = "AN002"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Cmpchb::_0x02
    }
    #[doc = "AN003"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Cmpchb::_0x03
    }
    #[doc = "AN004"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Cmpchb::_0x04
    }
    #[doc = "AN005"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Cmpchb::_0x05
    }
    #[doc = "AN006"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Cmpchb::_0x06
    }
    #[doc = "AN007"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Cmpchb::_0x07
    }
    #[doc = "AN008"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Cmpchb::_0x08
    }
    #[doc = "AN009"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Cmpchb::_0x09
    }
    #[doc = "AN010"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Cmpchb::_0x0a
    }
    #[doc = "AN011"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Cmpchb::_0x0b
    }
    #[doc = "AN012"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Cmpchb::_0x0c
    }
    #[doc = "AN013"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Cmpchb::_0x0d
    }
    #[doc = "AN014"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Cmpchb::_0x0e
    }
    #[doc = "AN015"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Cmpchb::_0x0f
    }
    #[doc = "AN016"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Cmpchb::_0x10
    }
    #[doc = "AN017"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Cmpchb::_0x11
    }
    #[doc = "AN018"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Cmpchb::_0x12
    }
    #[doc = "AN019"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == Cmpchb::_0x13
    }
    #[doc = "AN020"]
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == Cmpchb::_0x14
    }
    #[doc = "AN021"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == Cmpchb::_0x15
    }
    #[doc = "AN022"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == Cmpchb::_0x16
    }
    #[doc = "AN023"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == Cmpchb::_0x17
    }
    #[doc = "AN024"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == Cmpchb::_0x18
    }
    #[doc = "AN025"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == Cmpchb::_0x19
    }
    #[doc = "AN026"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == Cmpchb::_0x1a
    }
    #[doc = "AN027"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == Cmpchb::_0x1b
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == Cmpchb::_0x20
    }
    #[doc = "Internal reference voltage"]
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == Cmpchb::_0x21
    }
    #[doc = "No channel is selected"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == Cmpchb::_0x3f
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmpchb::Others)
    }
}
#[doc = "Field `CMPCHB` writer - Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
pub type CmpchbW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cmpchb, crate::Safe>;
impl<'a, REG> CmpchbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AN000"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x00)
    }
    #[doc = "AN001"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x01)
    }
    #[doc = "AN002"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x02)
    }
    #[doc = "AN003"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x03)
    }
    #[doc = "AN004"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x04)
    }
    #[doc = "AN005"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x05)
    }
    #[doc = "AN006"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x06)
    }
    #[doc = "AN007"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x07)
    }
    #[doc = "AN008"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x08)
    }
    #[doc = "AN009"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x09)
    }
    #[doc = "AN010"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0a)
    }
    #[doc = "AN011"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0b)
    }
    #[doc = "AN012"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0c)
    }
    #[doc = "AN013"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0d)
    }
    #[doc = "AN014"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0e)
    }
    #[doc = "AN015"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x0f)
    }
    #[doc = "AN016"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x10)
    }
    #[doc = "AN017"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x11)
    }
    #[doc = "AN018"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x12)
    }
    #[doc = "AN019"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x13)
    }
    #[doc = "AN020"]
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x14)
    }
    #[doc = "AN021"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x15)
    }
    #[doc = "AN022"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x16)
    }
    #[doc = "AN023"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x17)
    }
    #[doc = "AN024"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x18)
    }
    #[doc = "AN025"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x19)
    }
    #[doc = "AN026"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x1a)
    }
    #[doc = "AN027"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x1b)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x20)
    }
    #[doc = "Internal reference voltage"]
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x21)
    }
    #[doc = "No channel is selected"]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::_0x3f)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpchb::Others)
    }
}
#[doc = "Compare window B Compare condition setting bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplb {
    #[doc = "0: CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    _1 = 1,
}
impl From<Cmplb> for bool {
    #[inline(always)]
    fn from(variant: Cmplb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPLB` reader - Compare window B Compare condition setting bit."]
pub type CmplbR = crate::BitReader<Cmplb>;
impl CmplbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplb {
        match self.bits {
            false => Cmplb::_0,
            true => Cmplb::_1,
        }
    }
    #[doc = "CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplb::_0
    }
    #[doc = "CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplb::_1
    }
}
#[doc = "Field `CMPLB` writer - Compare window B Compare condition setting bit."]
pub type CmplbW<'a, REG> = crate::BitWriter<'a, REG, Cmplb>;
impl<'a, REG> CmplbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplb::_0)
    }
    #[doc = "CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplb::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    pub fn cmpchb(&self) -> CmpchbR {
        CmpchbR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Compare window B Compare condition setting bit."]
    #[inline(always)]
    pub fn cmplb(&self) -> CmplbR {
        CmplbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPBNSR")
            .field("cmplb", &self.cmplb())
            .field("cmpchb", &self.cmpchb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare window B channel selection bit. The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    pub fn cmpchb(&mut self) -> CmpchbW<AdcmpbnsrSpec> {
        CmpchbW::new(self, 0)
    }
    #[doc = "Bit 7 - Compare window B Compare condition setting bit."]
    #[inline(always)]
    pub fn cmplb(&mut self) -> CmplbW<AdcmpbnsrSpec> {
        CmplbW::new(self, 7)
    }
}
#[doc = "A/D Compare Function Window B Channel Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmpbnsrSpec;
impl crate::RegisterSpec for AdcmpbnsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpbnsr::R`](R) reader structure"]
impl crate::Readable for AdcmpbnsrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpbnsr::W`](W) writer structure"]
impl crate::Writable for AdcmpbnsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPBNSR to value 0"]
impl crate::Resettable for AdcmpbnsrSpec {}

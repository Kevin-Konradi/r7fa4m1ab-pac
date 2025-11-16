#[doc = "Register `ADADC` reader"]
pub type R = crate::R<AdadcSpec>;
#[doc = "Register `ADADC` writer"]
pub type W = crate::W<AdadcSpec>;
#[doc = "Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc {
    #[doc = "0: 1-time conversion (no addition; same as normal conversion)"]
    _000 = 0,
    #[doc = "1: 2-time conversion (addition once)"]
    _001 = 1,
    #[doc = "2: 3-time conversion (addition twice)"]
    _010 = 2,
    #[doc = "3: 4-time conversion (addition three times)"]
    _011 = 3,
    #[doc = "5: 16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
    _101 = 5,
    #[doc = "4: Setting prohibited"]
    Others = 4,
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc {
    type Ux = u8;
}
impl crate::IsEnum for Adc {}
#[doc = "Field `ADC` reader - Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
pub type AdcR = crate::FieldReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            0 => Adc::_000,
            1 => Adc::_001,
            2 => Adc::_010,
            3 => Adc::_011,
            5 => Adc::_101,
            _ => Adc::Others,
        }
    }
    #[doc = "1-time conversion (no addition; same as normal conversion)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Adc::_000
    }
    #[doc = "2-time conversion (addition once)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Adc::_001
    }
    #[doc = "3-time conversion (addition twice)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Adc::_010
    }
    #[doc = "4-time conversion (addition three times)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Adc::_011
    }
    #[doc = "16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Adc::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adc::Others)
    }
}
#[doc = "Field `ADC` writer - Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
pub type AdcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc, crate::Safe>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-time conversion (no addition; same as normal conversion)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_000)
    }
    #[doc = "2-time conversion (addition once)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_001)
    }
    #[doc = "3-time conversion (addition twice)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_010)
    }
    #[doc = "4-time conversion (addition three times)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_011)
    }
    #[doc = "16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Others)
    }
}
#[doc = "Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avee {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Avee> for bool {
    #[inline(always)]
    fn from(variant: Avee) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVEE` reader - Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
pub type AveeR = crate::BitReader<Avee>;
impl AveeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avee {
        match self.bits {
            false => Avee::_0,
            true => Avee::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avee::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avee::_1
    }
}
#[doc = "Field `AVEE` writer - Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
pub type AveeW<'a, REG> = crate::BitWriter<'a, REG, Avee>;
impl<'a, REG> AveeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avee::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avee::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
    #[inline(always)]
    pub fn avee(&self) -> AveeR {
        AveeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADADC")
            .field("avee", &self.avee())
            .field("adc", &self.adc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Addition frequency selection bit. NOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<AdadcSpec> {
        AdcW::new(self, 0)
    }
    #[doc = "Bit 7 - Average mode enable bit. Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
    #[inline(always)]
    pub fn avee(&mut self) -> AveeW<AdadcSpec> {
        AveeW::new(self, 7)
    }
}
#[doc = "A/D-Converted Value Addition/Average Count Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adadc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdadcSpec;
impl crate::RegisterSpec for AdadcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adadc::R`](R) reader structure"]
impl crate::Readable for AdadcSpec {}
#[doc = "`write(|w| ..)` method takes [`adadc::W`](W) writer structure"]
impl crate::Writable for AdadcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADADC to value 0"]
impl crate::Resettable for AdadcSpec {}

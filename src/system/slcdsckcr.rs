#[doc = "Register `SLCDSCKCR` reader"]
pub type R = crate::R<SlcdsckcrSpec>;
#[doc = "Register `SLCDSCKCR` writer"]
pub type W = crate::W<SlcdsckcrSpec>;
#[doc = "LCD Source Clock (LCDSRCCLK) Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdscksel {
    #[doc = "0: LOCO"]
    _000 = 0,
    #[doc = "1: SOSC"]
    _001 = 1,
    #[doc = "2: MOSC"]
    _010 = 2,
    #[doc = "4: HOCO"]
    _100 = 4,
    #[doc = "3: Settings other than above are prohibited."]
    Others = 3,
}
impl From<Lcdscksel> for u8 {
    #[inline(always)]
    fn from(variant: Lcdscksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdscksel {
    type Ux = u8;
}
impl crate::IsEnum for Lcdscksel {}
#[doc = "Field `LCDSCKSEL` reader - LCD Source Clock (LCDSRCCLK) Select"]
pub type LcdsckselR = crate::FieldReader<Lcdscksel>;
impl LcdsckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdscksel {
        match self.bits {
            0 => Lcdscksel::_000,
            1 => Lcdscksel::_001,
            2 => Lcdscksel::_010,
            4 => Lcdscksel::_100,
            _ => Lcdscksel::Others,
        }
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Lcdscksel::_000
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Lcdscksel::_001
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Lcdscksel::_010
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Lcdscksel::_100
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lcdscksel::Others)
    }
}
#[doc = "Field `LCDSCKSEL` writer - LCD Source Clock (LCDSRCCLK) Select"]
pub type LcdsckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdscksel, crate::Safe>;
impl<'a, REG> LcdsckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscksel::_000)
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscksel::_001)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscksel::_010)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscksel::_100)
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscksel::Others)
    }
}
#[doc = "LCD Source Clock Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdscken {
    #[doc = "0: LCD source clock out disabled"]
    _0 = 0,
    #[doc = "1: LCD source clock out enabled."]
    _1 = 1,
}
impl From<Lcdscken> for bool {
    #[inline(always)]
    fn from(variant: Lcdscken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSCKEN` reader - LCD Source Clock Out Enable"]
pub type LcdsckenR = crate::BitReader<Lcdscken>;
impl LcdsckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdscken {
        match self.bits {
            false => Lcdscken::_0,
            true => Lcdscken::_1,
        }
    }
    #[doc = "LCD source clock out disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lcdscken::_0
    }
    #[doc = "LCD source clock out enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdscken::_1
    }
}
#[doc = "Field `LCDSCKEN` writer - LCD Source Clock Out Enable"]
pub type LcdsckenW<'a, REG> = crate::BitWriter<'a, REG, Lcdscken>;
impl<'a, REG> LcdsckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD source clock out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscken::_0)
    }
    #[doc = "LCD source clock out enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdscken::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub fn lcdscksel(&self) -> LcdsckselR {
        LcdsckselR::new(self.bits & 7)
    }
    #[doc = "Bit 7 - LCD Source Clock Out Enable"]
    #[inline(always)]
    pub fn lcdscken(&self) -> LcdsckenR {
        LcdsckenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCDSCKCR")
            .field("lcdscken", &self.lcdscken())
            .field("lcdscksel", &self.lcdscksel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub fn lcdscksel(&mut self) -> LcdsckselW<SlcdsckcrSpec> {
        LcdsckselW::new(self, 0)
    }
    #[doc = "Bit 7 - LCD Source Clock Out Enable"]
    #[inline(always)]
    pub fn lcdscken(&mut self) -> LcdsckenW<SlcdsckcrSpec> {
        LcdsckenW::new(self, 7)
    }
}
#[doc = "Segment LCD Source Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slcdsckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slcdsckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlcdsckcrSpec;
impl crate::RegisterSpec for SlcdsckcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`slcdsckcr::R`](R) reader structure"]
impl crate::Readable for SlcdsckcrSpec {}
#[doc = "`write(|w| ..)` method takes [`slcdsckcr::W`](W) writer structure"]
impl crate::Writable for SlcdsckcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLCDSCKCR to value 0"]
impl crate::Resettable for SlcdsckcrSpec {}

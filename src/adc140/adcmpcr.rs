#[doc = "Register `ADCMPCR` reader"]
pub type R = crate::R<AdcmpcrSpec>;
#[doc = "Register `ADCMPCR` writer"]
pub type W = crate::W<AdcmpcrSpec>;
#[doc = "Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpab {
    #[doc = "0: ADC140_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _00 = 0,
    #[doc = "1: S14ADWMELC0 is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _01 = 1,
    #[doc = "2: ADC140_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<Cmpab> for u8 {
    #[inline(always)]
    fn from(variant: Cmpab) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpab {
    type Ux = u8;
}
impl crate::IsEnum for Cmpab {}
#[doc = "Field `CMPAB` reader - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
pub type CmpabR = crate::FieldReader<Cmpab>;
impl CmpabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpab {
        match self.bits {
            0 => Cmpab::_00,
            1 => Cmpab::_01,
            2 => Cmpab::_10,
            3 => Cmpab::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cmpab::_00
    }
    #[doc = "S14ADWMELC0 is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cmpab::_01
    }
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cmpab::_10
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cmpab::_11
    }
}
#[doc = "Field `CMPAB` writer - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
pub type CmpabW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpab, crate::Safe>;
impl<'a, REG> CmpabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_00)
    }
    #[doc = "S14ADWMELC0 is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_01)
    }
    #[doc = "ADC140_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC140_WCMPUM is output in other cases."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_11)
    }
}
#[doc = "Compare Window B Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpbe {
    #[doc = "0: Compare window B operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    _0 = 0,
    #[doc = "1: Compare window B operation is enabled."]
    _1 = 1,
}
impl From<Cmpbe> for bool {
    #[inline(always)]
    fn from(variant: Cmpbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPBE` reader - Compare Window B Operation Enable"]
pub type CmpbeR = crate::BitReader<Cmpbe>;
impl CmpbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpbe {
        match self.bits {
            false => Cmpbe::_0,
            true => Cmpbe::_1,
        }
    }
    #[doc = "Compare window B operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpbe::_0
    }
    #[doc = "Compare window B operation is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpbe::_1
    }
}
#[doc = "Field `CMPBE` writer - Compare Window B Operation Enable"]
pub type CmpbeW<'a, REG> = crate::BitWriter<'a, REG, Cmpbe>;
impl<'a, REG> CmpbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare window B operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbe::_0)
    }
    #[doc = "Compare window B operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbe::_1)
    }
}
#[doc = "Compare Window A Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpae {
    #[doc = "0: Compare window A operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    _0 = 0,
    #[doc = "1: Compare window A operation is enabled."]
    _1 = 1,
}
impl From<Cmpae> for bool {
    #[inline(always)]
    fn from(variant: Cmpae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPAE` reader - Compare Window A Operation Enable"]
pub type CmpaeR = crate::BitReader<Cmpae>;
impl CmpaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpae {
        match self.bits {
            false => Cmpae::_0,
            true => Cmpae::_1,
        }
    }
    #[doc = "Compare window A operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpae::_0
    }
    #[doc = "Compare window A operation is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpae::_1
    }
}
#[doc = "Field `CMPAE` writer - Compare Window A Operation Enable"]
pub type CmpaeW<'a, REG> = crate::BitWriter<'a, REG, Cmpae>;
impl<'a, REG> CmpaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare window A operation is disabled. ADC140_WCMPM and ADC140_WCMPUM outputs are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpae::_0)
    }
    #[doc = "Compare window A operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpae::_1)
    }
}
#[doc = "Compare B Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpbie {
    #[doc = "0: ADC140_CMPAI interrupt is disabled when comparison conditions (window B) are met."]
    _0 = 0,
    #[doc = "1: ADC140_CMPAI interrupt is enabled when comparison conditions (window B) are met."]
    _1 = 1,
}
impl From<Cmpbie> for bool {
    #[inline(always)]
    fn from(variant: Cmpbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPBIE` reader - Compare B Interrupt Enable"]
pub type CmpbieR = crate::BitReader<Cmpbie>;
impl CmpbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpbie {
        match self.bits {
            false => Cmpbie::_0,
            true => Cmpbie::_1,
        }
    }
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpbie::_0
    }
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpbie::_1
    }
}
#[doc = "Field `CMPBIE` writer - Compare B Interrupt Enable"]
pub type CmpbieW<'a, REG> = crate::BitWriter<'a, REG, Cmpbie>;
impl<'a, REG> CmpbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbie::_0)
    }
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window B) are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbie::_1)
    }
}
#[doc = "Window Function Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcmpe {
    #[doc = "0: Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    _0 = 0,
    #[doc = "1: Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    _1 = 1,
}
impl From<Wcmpe> for bool {
    #[inline(always)]
    fn from(variant: Wcmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCMPE` reader - Window Function Setting"]
pub type WcmpeR = crate::BitReader<Wcmpe>;
impl WcmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcmpe {
        match self.bits {
            false => Wcmpe::_0,
            true => Wcmpe::_1,
        }
    }
    #[doc = "Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wcmpe::_0
    }
    #[doc = "Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wcmpe::_1
    }
}
#[doc = "Field `WCMPE` writer - Window Function Setting"]
pub type WcmpeW<'a, REG> = crate::BitWriter<'a, REG, Wcmpe>;
impl<'a, REG> WcmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wcmpe::_0)
    }
    #[doc = "Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wcmpe::_1)
    }
}
#[doc = "Compare A Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpaie {
    #[doc = "0: ADC140_CMPAI interrupt is disabled when comparison conditions (window A) are met."]
    _0 = 0,
    #[doc = "1: ADC140_CMPAI interrupt is enabled when comparison conditions (window A) are met."]
    _1 = 1,
}
impl From<Cmpaie> for bool {
    #[inline(always)]
    fn from(variant: Cmpaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPAIE` reader - Compare A Interrupt Enable"]
pub type CmpaieR = crate::BitReader<Cmpaie>;
impl CmpaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpaie {
        match self.bits {
            false => Cmpaie::_0,
            true => Cmpaie::_1,
        }
    }
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpaie::_0
    }
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpaie::_1
    }
}
#[doc = "Field `CMPAIE` writer - Compare A Interrupt Enable"]
pub type CmpaieW<'a, REG> = crate::BitWriter<'a, REG, Cmpaie>;
impl<'a, REG> CmpaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC140_CMPAI interrupt is disabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpaie::_0)
    }
    #[doc = "ADC140_CMPAI interrupt is enabled when comparison conditions (window A) are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpaie::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    pub fn cmpab(&self) -> CmpabR {
        CmpabR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 9 - Compare Window B Operation Enable"]
    #[inline(always)]
    pub fn cmpbe(&self) -> CmpbeR {
        CmpbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Window A Operation Enable"]
    #[inline(always)]
    pub fn cmpae(&self) -> CmpaeR {
        CmpaeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare B Interrupt Enable"]
    #[inline(always)]
    pub fn cmpbie(&self) -> CmpbieR {
        CmpbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Window Function Setting"]
    #[inline(always)]
    pub fn wcmpe(&self) -> WcmpeR {
        WcmpeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare A Interrupt Enable"]
    #[inline(always)]
    pub fn cmpaie(&self) -> CmpaieR {
        CmpaieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPCR")
            .field("cmpaie", &self.cmpaie())
            .field("wcmpe", &self.wcmpe())
            .field("cmpbie", &self.cmpbie())
            .field("cmpae", &self.cmpae())
            .field("cmpbe", &self.cmpbe())
            .field("cmpab", &self.cmpab())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Window A/B Composite Conditions Setting NOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    pub fn cmpab(&mut self) -> CmpabW<AdcmpcrSpec> {
        CmpabW::new(self, 0)
    }
    #[doc = "Bit 9 - Compare Window B Operation Enable"]
    #[inline(always)]
    pub fn cmpbe(&mut self) -> CmpbeW<AdcmpcrSpec> {
        CmpbeW::new(self, 9)
    }
    #[doc = "Bit 11 - Compare Window A Operation Enable"]
    #[inline(always)]
    pub fn cmpae(&mut self) -> CmpaeW<AdcmpcrSpec> {
        CmpaeW::new(self, 11)
    }
    #[doc = "Bit 13 - Compare B Interrupt Enable"]
    #[inline(always)]
    pub fn cmpbie(&mut self) -> CmpbieW<AdcmpcrSpec> {
        CmpbieW::new(self, 13)
    }
    #[doc = "Bit 14 - Window Function Setting"]
    #[inline(always)]
    pub fn wcmpe(&mut self) -> WcmpeW<AdcmpcrSpec> {
        WcmpeW::new(self, 14)
    }
    #[doc = "Bit 15 - Compare A Interrupt Enable"]
    #[inline(always)]
    pub fn cmpaie(&mut self) -> CmpaieW<AdcmpcrSpec> {
        CmpaieW::new(self, 15)
    }
}
#[doc = "A/D Compare Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmpcrSpec;
impl crate::RegisterSpec for AdcmpcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpcr::R`](R) reader structure"]
impl crate::Readable for AdcmpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpcr::W`](W) writer structure"]
impl crate::Writable for AdcmpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPCR to value 0"]
impl crate::Resettable for AdcmpcrSpec {}

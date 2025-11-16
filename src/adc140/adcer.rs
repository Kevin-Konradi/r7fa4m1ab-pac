#[doc = "Register `ADCER` reader"]
pub type R = crate::R<AdcerSpec>;
#[doc = "Register `ADCER` writer"]
pub type W = crate::W<AdcerSpec>;
#[doc = "A/D Conversion Accuracy Specify\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adprc {
    #[doc = "0: A/D conversion is performed with 12-bit accuracy."]
    _00 = 0,
    #[doc = "3: A/D conversion is performed with 14-bit accuracy."]
    _11 = 3,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Adprc> for u8 {
    #[inline(always)]
    fn from(variant: Adprc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adprc {
    type Ux = u8;
}
impl crate::IsEnum for Adprc {}
#[doc = "Field `ADPRC` reader - A/D Conversion Accuracy Specify"]
pub type AdprcR = crate::FieldReader<Adprc>;
impl AdprcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adprc {
        match self.bits {
            0 => Adprc::_00,
            3 => Adprc::_11,
            _ => Adprc::Others,
        }
    }
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adprc::_00
    }
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adprc::_11
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adprc::Others)
    }
}
#[doc = "Field `ADPRC` writer - A/D Conversion Accuracy Specify"]
pub type AdprcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adprc, crate::Safe>;
impl<'a, REG> AdprcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A/D conversion is performed with 12-bit accuracy."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_00)
    }
    #[doc = "A/D conversion is performed with 14-bit accuracy."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_11)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::Others)
    }
}
#[doc = "A/D Data Register Automatic Clearing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ace {
    #[doc = "0: Disables automatic clearing."]
    _0 = 0,
    #[doc = "1: Enables automatic clearing."]
    _1 = 1,
}
impl From<Ace> for bool {
    #[inline(always)]
    fn from(variant: Ace) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACE` reader - A/D Data Register Automatic Clearing Enable"]
pub type AceR = crate::BitReader<Ace>;
impl AceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ace {
        match self.bits {
            false => Ace::_0,
            true => Ace::_1,
        }
    }
    #[doc = "Disables automatic clearing."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ace::_0
    }
    #[doc = "Enables automatic clearing."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ace::_1
    }
}
#[doc = "Field `ACE` writer - A/D Data Register Automatic Clearing Enable"]
pub type AceW<'a, REG> = crate::BitWriter<'a, REG, Ace>;
impl<'a, REG> AceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables automatic clearing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ace::_0)
    }
    #[doc = "Enables automatic clearing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ace::_1)
    }
}
#[doc = "Self-Diagnosis Conversion Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diagval {
    #[doc = "0: When the self-diagnosis fixation mode is selected, it set prohibits it."]
    _00 = 0,
    #[doc = "1: The self-diagnosis by using the voltage of 0V."]
    _01 = 1,
    #[doc = "2: The self-diagnosis by using the voltage of reference supply x 1/2."]
    _10 = 2,
    #[doc = "3: The self-diagnosis by using the voltage of the reference supply."]
    _11 = 3,
}
impl From<Diagval> for u8 {
    #[inline(always)]
    fn from(variant: Diagval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diagval {
    type Ux = u8;
}
impl crate::IsEnum for Diagval {}
#[doc = "Field `DIAGVAL` reader - Self-Diagnosis Conversion Voltage Select"]
pub type DiagvalR = crate::FieldReader<Diagval>;
impl DiagvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diagval {
        match self.bits {
            0 => Diagval::_00,
            1 => Diagval::_01,
            2 => Diagval::_10,
            3 => Diagval::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Diagval::_00
    }
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Diagval::_01
    }
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Diagval::_10
    }
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Diagval::_11
    }
}
#[doc = "Field `DIAGVAL` writer - Self-Diagnosis Conversion Voltage Select"]
pub type DiagvalW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diagval, crate::Safe>;
impl<'a, REG> DiagvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_00)
    }
    #[doc = "The self-diagnosis by using the voltage of 0V."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_01)
    }
    #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_10)
    }
    #[doc = "The self-diagnosis by using the voltage of the reference supply."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_11)
    }
}
#[doc = "Self-Diagnosis Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagld {
    #[doc = "0: Rotation mode for self-diagnosis voltage"]
    _0 = 0,
    #[doc = "1: Fixed mode for self-diagnosis voltage"]
    _1 = 1,
}
impl From<Diagld> for bool {
    #[inline(always)]
    fn from(variant: Diagld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIAGLD` reader - Self-Diagnosis Mode Select"]
pub type DiagldR = crate::BitReader<Diagld>;
impl DiagldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diagld {
        match self.bits {
            false => Diagld::_0,
            true => Diagld::_1,
        }
    }
    #[doc = "Rotation mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Diagld::_0
    }
    #[doc = "Fixed mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diagld::_1
    }
}
#[doc = "Field `DIAGLD` writer - Self-Diagnosis Mode Select"]
pub type DiagldW<'a, REG> = crate::BitWriter<'a, REG, Diagld>;
impl<'a, REG> DiagldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rotation mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Diagld::_0)
    }
    #[doc = "Fixed mode for self-diagnosis voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diagld::_1)
    }
}
#[doc = "Self-Diagnosis Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagm {
    #[doc = "0: Disables self-diagnosis of A/D converter."]
    _0 = 0,
    #[doc = "1: Enables self-diagnosis of A/D converter."]
    _1 = 1,
}
impl From<Diagm> for bool {
    #[inline(always)]
    fn from(variant: Diagm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIAGM` reader - Self-Diagnosis Enable"]
pub type DiagmR = crate::BitReader<Diagm>;
impl DiagmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diagm {
        match self.bits {
            false => Diagm::_0,
            true => Diagm::_1,
        }
    }
    #[doc = "Disables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Diagm::_0
    }
    #[doc = "Enables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diagm::_1
    }
}
#[doc = "Field `DIAGM` writer - Self-Diagnosis Enable"]
pub type DiagmW<'a, REG> = crate::BitWriter<'a, REG, Diagm>;
impl<'a, REG> DiagmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Diagm::_0)
    }
    #[doc = "Enables self-diagnosis of A/D converter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diagm::_1)
    }
}
#[doc = "A/D Data Register Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrfmt {
    #[doc = "0: Flush-right is selected for the A/D data register format."]
    _0 = 0,
    #[doc = "1: Flush-left is selected for the A/D data register format."]
    _1 = 1,
}
impl From<Adrfmt> for bool {
    #[inline(always)]
    fn from(variant: Adrfmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRFMT` reader - A/D Data Register Format Select"]
pub type AdrfmtR = crate::BitReader<Adrfmt>;
impl AdrfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adrfmt {
        match self.bits {
            false => Adrfmt::_0,
            true => Adrfmt::_1,
        }
    }
    #[doc = "Flush-right is selected for the A/D data register format."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adrfmt::_0
    }
    #[doc = "Flush-left is selected for the A/D data register format."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adrfmt::_1
    }
}
#[doc = "Field `ADRFMT` writer - A/D Data Register Format Select"]
pub type AdrfmtW<'a, REG> = crate::BitWriter<'a, REG, Adrfmt>;
impl<'a, REG> AdrfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush-right is selected for the A/D data register format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrfmt::_0)
    }
    #[doc = "Flush-left is selected for the A/D data register format."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrfmt::_1)
    }
}
impl R {
    #[doc = "Bits 1:2 - A/D Conversion Accuracy Specify"]
    #[inline(always)]
    pub fn adprc(&self) -> AdprcR {
        AdprcR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub fn ace(&self) -> AceR {
        AceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub fn diagval(&self) -> DiagvalR {
        DiagvalR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub fn diagld(&self) -> DiagldR {
        DiagldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn diagm(&self) -> DiagmR {
        DiagmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    pub fn adrfmt(&self) -> AdrfmtR {
        AdrfmtR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCER")
            .field("adrfmt", &self.adrfmt())
            .field("diagm", &self.diagm())
            .field("diagld", &self.diagld())
            .field("diagval", &self.diagval())
            .field("ace", &self.ace())
            .field("adprc", &self.adprc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:2 - A/D Conversion Accuracy Specify"]
    #[inline(always)]
    pub fn adprc(&mut self) -> AdprcW<AdcerSpec> {
        AdprcW::new(self, 1)
    }
    #[doc = "Bit 5 - A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub fn ace(&mut self) -> AceW<AdcerSpec> {
        AceW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub fn diagval(&mut self) -> DiagvalW<AdcerSpec> {
        DiagvalW::new(self, 8)
    }
    #[doc = "Bit 10 - Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub fn diagld(&mut self) -> DiagldW<AdcerSpec> {
        DiagldW::new(self, 10)
    }
    #[doc = "Bit 11 - Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn diagm(&mut self) -> DiagmW<AdcerSpec> {
        DiagmW::new(self, 11)
    }
    #[doc = "Bit 15 - A/D Data Register Format Select"]
    #[inline(always)]
    pub fn adrfmt(&mut self) -> AdrfmtW<AdcerSpec> {
        AdrfmtW::new(self, 15)
    }
}
#[doc = "A/D Control Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcerSpec;
impl crate::RegisterSpec for AdcerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcer::R`](R) reader structure"]
impl crate::Readable for AdcerSpec {}
#[doc = "`write(|w| ..)` method takes [`adcer::W`](W) writer structure"]
impl crate::Writable for AdcerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for AdcerSpec {}

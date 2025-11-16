#[doc = "Register `AGTIOC` reader"]
pub type R = crate::R<AgtiocSpec>;
#[doc = "Register `AGTIOC` writer"]
pub type W = crate::W<AgtiocSpec>;
#[doc = "Field `TEDGSEL` reader - I/O polarity switch Function varies depending on the operating mode."]
pub type TedgselR = crate::BitReader;
#[doc = "Field `TEDGSEL` writer - I/O polarity switch Function varies depending on the operating mode."]
pub type TedgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AGTOn output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toe {
    #[doc = "0: AGTOn output disabled"]
    _0 = 0,
    #[doc = "1: AGTOn output enabled."]
    _1 = 1,
}
impl From<Toe> for bool {
    #[inline(always)]
    fn from(variant: Toe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOE` reader - AGTOn output enable"]
pub type ToeR = crate::BitReader<Toe>;
impl ToeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toe {
        match self.bits {
            false => Toe::_0,
            true => Toe::_1,
        }
    }
    #[doc = "AGTOn output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toe::_0
    }
    #[doc = "AGTOn output enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toe::_1
    }
}
#[doc = "Field `TOE` writer - AGTOn output enable"]
pub type ToeW<'a, REG> = crate::BitWriter<'a, REG, Toe>;
impl<'a, REG> ToeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGTOn output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_0)
    }
    #[doc = "AGTOn output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_1)
    }
}
#[doc = "Input filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tipf {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Filter sampled at PCLKB"]
    _01 = 1,
    #[doc = "2: Filter sampled at PCLKB/8"]
    _10 = 2,
    #[doc = "3: Filter sampled at PCLKB/32"]
    _11 = 3,
}
impl From<Tipf> for u8 {
    #[inline(always)]
    fn from(variant: Tipf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tipf {
    type Ux = u8;
}
impl crate::IsEnum for Tipf {}
#[doc = "Field `TIPF` reader - Input filter"]
pub type TipfR = crate::FieldReader<Tipf>;
impl TipfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tipf {
        match self.bits {
            0 => Tipf::_00,
            1 => Tipf::_01,
            2 => Tipf::_10,
            3 => Tipf::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tipf::_00
    }
    #[doc = "Filter sampled at PCLKB"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tipf::_01
    }
    #[doc = "Filter sampled at PCLKB/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tipf::_10
    }
    #[doc = "Filter sampled at PCLKB/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tipf::_11
    }
}
#[doc = "Field `TIPF` writer - Input filter"]
pub type TipfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tipf, crate::Safe>;
impl<'a, REG> TipfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tipf::_00)
    }
    #[doc = "Filter sampled at PCLKB"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tipf::_01)
    }
    #[doc = "Filter sampled at PCLKB/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tipf::_10)
    }
    #[doc = "Filter sampled at PCLKB/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tipf::_11)
    }
}
#[doc = "Count control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tiogt {
    #[doc = "0: Event is always counted"]
    _00 = 0,
    #[doc = "1: Event is counted during polarity period specified for AGTEEn."]
    _01 = 1,
    #[doc = "2: settings are prohibited."]
    Others = 2,
}
impl From<Tiogt> for u8 {
    #[inline(always)]
    fn from(variant: Tiogt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tiogt {
    type Ux = u8;
}
impl crate::IsEnum for Tiogt {}
#[doc = "Field `TIOGT` reader - Count control"]
pub type TiogtR = crate::FieldReader<Tiogt>;
impl TiogtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tiogt {
        match self.bits {
            0 => Tiogt::_00,
            1 => Tiogt::_01,
            _ => Tiogt::Others,
        }
    }
    #[doc = "Event is always counted"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tiogt::_00
    }
    #[doc = "Event is counted during polarity period specified for AGTEEn."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tiogt::_01
    }
    #[doc = "settings are prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tiogt::Others)
    }
}
#[doc = "Field `TIOGT` writer - Count control"]
pub type TiogtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tiogt, crate::Safe>;
impl<'a, REG> TiogtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event is always counted"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tiogt::_00)
    }
    #[doc = "Event is counted during polarity period specified for AGTEEn."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tiogt::_01)
    }
    #[doc = "settings are prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tiogt::Others)
    }
}
impl R {
    #[doc = "Bit 0 - I/O polarity switch Function varies depending on the operating mode."]
    #[inline(always)]
    pub fn tedgsel(&self) -> TedgselR {
        TedgselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AGTOn output enable"]
    #[inline(always)]
    pub fn toe(&self) -> ToeR {
        ToeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Input filter"]
    #[inline(always)]
    pub fn tipf(&self) -> TipfR {
        TipfR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Count control"]
    #[inline(always)]
    pub fn tiogt(&self) -> TiogtR {
        TiogtR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTIOC")
            .field("tiogt", &self.tiogt())
            .field("tipf", &self.tipf())
            .field("toe", &self.toe())
            .field("tedgsel", &self.tedgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I/O polarity switch Function varies depending on the operating mode."]
    #[inline(always)]
    pub fn tedgsel(&mut self) -> TedgselW<AgtiocSpec> {
        TedgselW::new(self, 0)
    }
    #[doc = "Bit 2 - AGTOn output enable"]
    #[inline(always)]
    pub fn toe(&mut self) -> ToeW<AgtiocSpec> {
        ToeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Input filter"]
    #[inline(always)]
    pub fn tipf(&mut self) -> TipfW<AgtiocSpec> {
        TipfW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Count control"]
    #[inline(always)]
    pub fn tiogt(&mut self) -> TiogtW<AgtiocSpec> {
        TiogtW::new(self, 6)
    }
}
#[doc = "AGT I/O Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtioc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtioc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtiocSpec;
impl crate::RegisterSpec for AgtiocSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtioc::R`](R) reader structure"]
impl crate::Readable for AgtiocSpec {}
#[doc = "`write(|w| ..)` method takes [`agtioc::W`](W) writer structure"]
impl crate::Writable for AgtiocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTIOC to value 0"]
impl crate::Resettable for AgtiocSpec {}

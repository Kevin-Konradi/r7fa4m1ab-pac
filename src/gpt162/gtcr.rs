#[doc = "Register `GTCR` reader"]
pub type R = crate::R<GtcrSpec>;
#[doc = "Register `GTCR` writer"]
pub type W = crate::W<GtcrSpec>;
#[doc = "Count Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst {
    #[doc = "0: Count operation is stopped"]
    _0 = 0,
    #[doc = "1: Count operation is performed"]
    _1 = 1,
}
impl From<Cst> for bool {
    #[inline(always)]
    fn from(variant: Cst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST` reader - Count Start"]
pub type CstR = crate::BitReader<Cst>;
impl CstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cst {
        match self.bits {
            false => Cst::_0,
            true => Cst::_1,
        }
    }
    #[doc = "Count operation is stopped"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cst::_0
    }
    #[doc = "Count operation is performed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cst::_1
    }
}
#[doc = "Field `CST` writer - Count Start"]
pub type CstW<'a, REG> = crate::BitWriter<'a, REG, Cst>;
impl<'a, REG> CstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count operation is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cst::_0)
    }
    #[doc = "Count operation is performed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst::_1)
    }
}
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md {
    #[doc = "0: Saw-wave PWM mode (single buffer or double buffer possible)"]
    _000 = 0,
    #[doc = "1: Saw-wave one-shot pulse mode (fixed buffer operation)"]
    _001 = 1,
    #[doc = "2: Setting prohibited"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
    #[doc = "4: Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    _100 = 4,
    #[doc = "5: Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    _101 = 5,
    #[doc = "6: Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(variant: Md) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md {
    type Ux = u8;
}
impl crate::IsEnum for Md {}
#[doc = "Field `MD` reader - Mode Select"]
pub type MdR = crate::FieldReader<Md>;
impl MdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md {
        match self.bits {
            0 => Md::_000,
            1 => Md::_001,
            2 => Md::_010,
            3 => Md::_011,
            4 => Md::_100,
            5 => Md::_101,
            6 => Md::_110,
            7 => Md::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Md::_000
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Md::_001
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Md::_010
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Md::_011
    }
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Md::_100
    }
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Md::_101
    }
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Md::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Md::_111
    }
}
#[doc = "Field `MD` writer - Mode Select"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Md, crate::Safe>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_000)
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_011)
    }
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_100)
    }
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_101)
    }
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_111)
    }
}
#[doc = "Timer Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpcs {
    #[doc = "0: PCLK/1"]
    _000 = 0,
    #[doc = "1: PCLK/4"]
    _001 = 1,
    #[doc = "2: PCLK/16"]
    _010 = 2,
    #[doc = "3: PCLK/64"]
    _011 = 3,
    #[doc = "4: PCLK/256"]
    _100 = 4,
    #[doc = "5: PCLK/1024"]
    _101 = 5,
    #[doc = "6: Setting prohibied"]
    Others = 6,
}
impl From<Tpcs> for u8 {
    #[inline(always)]
    fn from(variant: Tpcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpcs {
    type Ux = u8;
}
impl crate::IsEnum for Tpcs {}
#[doc = "Field `TPCS` reader - Timer Prescaler Select"]
pub type TpcsR = crate::FieldReader<Tpcs>;
impl TpcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpcs {
        match self.bits {
            0 => Tpcs::_000,
            1 => Tpcs::_001,
            2 => Tpcs::_010,
            3 => Tpcs::_011,
            4 => Tpcs::_100,
            5 => Tpcs::_101,
            _ => Tpcs::Others,
        }
    }
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tpcs::_000
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tpcs::_001
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Tpcs::_010
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tpcs::_011
    }
    #[doc = "PCLK/256"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tpcs::_100
    }
    #[doc = "PCLK/1024"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Tpcs::_101
    }
    #[doc = "Setting prohibied"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tpcs::Others)
    }
}
#[doc = "Field `TPCS` writer - Timer Prescaler Select"]
pub type TpcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tpcs, crate::Safe>;
impl<'a, REG> TpcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_000)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_001)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_010)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_011)
    }
    #[doc = "PCLK/256"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_100)
    }
    #[doc = "PCLK/1024"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_101)
    }
    #[doc = "Setting prohibied"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    pub fn cst(&self) -> CstR {
        CstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(&self) -> TpcsR {
        TpcsR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCR")
            .field("tpcs", &self.tpcs())
            .field("md", &self.md())
            .field("cst", &self.cst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    pub fn cst(&mut self) -> CstW<GtcrSpec> {
        CstW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<GtcrSpec> {
        MdW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(&mut self) -> TpcsW<GtcrSpec> {
        TpcsW::new(self, 24)
    }
}
#[doc = "General PWM Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcrSpec;
impl crate::RegisterSpec for GtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcr::R`](R) reader structure"]
impl crate::Readable for GtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtcr::W`](W) writer structure"]
impl crate::Writable for GtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTCR to value 0"]
impl crate::Resettable for GtcrSpec {}

#[doc = "Register `TFCR` reader"]
pub type R = crate::R<TfcrSpec>;
#[doc = "Register `TFCR` writer"]
pub type W = crate::W<TfcrSpec>;
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "0: Transmit FIFO disabled"]
    _0 = 0,
    #[doc = "1: Transmit FIFO enabled"]
    _1 = 1,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Enable"]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            false => Tfe::_0,
            true => Tfe::_1,
        }
    }
    #[doc = "Transmit FIFO disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfe::_0
    }
    #[doc = "Transmit FIFO enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfe::_1
    }
}
#[doc = "Field `TFE` writer - Transmit FIFO Enable"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG, Tfe>;
impl<'a, REG> TfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::_0)
    }
    #[doc = "Transmit FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::_1)
    }
}
#[doc = "Transmit FIFO Unsent Message Number Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfust {
    #[doc = "0: No unsent message"]
    _000 = 0,
    #[doc = "1: 1 unsent message"]
    _001 = 1,
    #[doc = "2: 2 unsent messages"]
    _010 = 2,
    #[doc = "3: 3 unsent messages"]
    _011 = 3,
    #[doc = "4: 4 unsent messages"]
    _100 = 4,
    #[doc = "5: Setting prohibited"]
    Others = 5,
}
impl From<Tfust> for u8 {
    #[inline(always)]
    fn from(variant: Tfust) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfust {
    type Ux = u8;
}
impl crate::IsEnum for Tfust {}
#[doc = "Field `TFUST` reader - Transmit FIFO Unsent Message Number Status"]
pub type TfustR = crate::FieldReader<Tfust>;
impl TfustR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfust {
        match self.bits {
            0 => Tfust::_000,
            1 => Tfust::_001,
            2 => Tfust::_010,
            3 => Tfust::_011,
            4 => Tfust::_100,
            _ => Tfust::Others,
        }
    }
    #[doc = "No unsent message"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tfust::_000
    }
    #[doc = "1 unsent message"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tfust::_001
    }
    #[doc = "2 unsent messages"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Tfust::_010
    }
    #[doc = "3 unsent messages"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tfust::_011
    }
    #[doc = "4 unsent messages"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tfust::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tfust::Others)
    }
}
#[doc = "Transmit FIFO Full Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tffst {
    #[doc = "0: Transmit FIFO is not full"]
    _0 = 0,
    #[doc = "1: Transmit FIFO is full (4 unsent messages)"]
    _1 = 1,
}
impl From<Tffst> for bool {
    #[inline(always)]
    fn from(variant: Tffst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFST` reader - Transmit FIFO Full Status"]
pub type TffstR = crate::BitReader<Tffst>;
impl TffstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tffst {
        match self.bits {
            false => Tffst::_0,
            true => Tffst::_1,
        }
    }
    #[doc = "Transmit FIFO is not full"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tffst::_0
    }
    #[doc = "Transmit FIFO is full (4 unsent messages)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tffst::_1
    }
}
#[doc = "Transmit FIFO Empty Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfest {
    #[doc = "0: Unsent message in transmit FIFO"]
    _0 = 0,
    #[doc = "1: No unsent message in transmit FIFO"]
    _1 = 1,
}
impl From<Tfest> for bool {
    #[inline(always)]
    fn from(variant: Tfest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFEST` reader - Transmit FIFO Empty Status"]
pub type TfestR = crate::BitReader<Tfest>;
impl TfestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfest {
        match self.bits {
            false => Tfest::_0,
            true => Tfest::_1,
        }
    }
    #[doc = "Unsent message in transmit FIFO"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfest::_0
    }
    #[doc = "No unsent message in transmit FIFO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfest::_1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Transmit FIFO Unsent Message Number Status"]
    #[inline(always)]
    pub fn tfust(&self) -> TfustR {
        TfustR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 6 - Transmit FIFO Full Status"]
    #[inline(always)]
    pub fn tffst(&self) -> TffstR {
        TffstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Status"]
    #[inline(always)]
    pub fn tfest(&self) -> TfestR {
        TfestR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TFCR")
            .field("tfest", &self.tfest())
            .field("tffst", &self.tffst())
            .field("tfust", &self.tfust())
            .field("tfe", &self.tfe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TfeW<TfcrSpec> {
        TfeW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfcrSpec;
impl crate::RegisterSpec for TfcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tfcr::R`](R) reader structure"]
impl crate::Readable for TfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tfcr::W`](W) writer structure"]
impl crate::Writable for TfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFCR to value 0x80"]
impl crate::Resettable for TfcrSpec {
    const RESET_VALUE: u8 = 0x80;
}

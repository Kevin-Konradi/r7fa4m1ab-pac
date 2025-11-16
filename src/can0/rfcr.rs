#[doc = "Register `RFCR` reader"]
pub type R = crate::R<RfcrSpec>;
#[doc = "Register `RFCR` writer"]
pub type W = crate::W<RfcrSpec>;
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "0: Receive FIFO disabled"]
    _0 = 0,
    #[doc = "1: Receive FIFO enabled"]
    _1 = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Enable"]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::_0,
            true => Rfe::_1,
        }
    }
    #[doc = "Receive FIFO disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfe::_0
    }
    #[doc = "Receive FIFO enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfe::_1
    }
}
#[doc = "Field `RFE` writer - Receive FIFO Enable"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG, Rfe>;
impl<'a, REG> RfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::_0)
    }
    #[doc = "Receive FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::_1)
    }
}
#[doc = "Receive FIFO Unread Message Number Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfust {
    #[doc = "0: No unread message"]
    _000 = 0,
    #[doc = "1: 1 unread message"]
    _001 = 1,
    #[doc = "2: 2 unread messages"]
    _010 = 2,
    #[doc = "3: 3 unread messages"]
    _011 = 3,
    #[doc = "4: 4 unread messages"]
    _100 = 4,
    #[doc = "5: Setting prohibited"]
    Others = 5,
}
impl From<Rfust> for u8 {
    #[inline(always)]
    fn from(variant: Rfust) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfust {
    type Ux = u8;
}
impl crate::IsEnum for Rfust {}
#[doc = "Field `RFUST` reader - Receive FIFO Unread Message Number Status"]
pub type RfustR = crate::FieldReader<Rfust>;
impl RfustR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfust {
        match self.bits {
            0 => Rfust::_000,
            1 => Rfust::_001,
            2 => Rfust::_010,
            3 => Rfust::_011,
            4 => Rfust::_100,
            _ => Rfust::Others,
        }
    }
    #[doc = "No unread message"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rfust::_000
    }
    #[doc = "1 unread message"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rfust::_001
    }
    #[doc = "2 unread messages"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rfust::_010
    }
    #[doc = "3 unread messages"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rfust::_011
    }
    #[doc = "4 unread messages"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rfust::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rfust::Others)
    }
}
#[doc = "Receive FIFO Message Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfmlf {
    #[doc = "0: No receive FIFO message lost has occurred"]
    _0 = 0,
    #[doc = "1: Receive FIFO message lost has occurred"]
    _1 = 1,
}
impl From<Rfmlf> for bool {
    #[inline(always)]
    fn from(variant: Rfmlf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFMLF` reader - Receive FIFO Message Lost Flag"]
pub type RfmlfR = crate::BitReader<Rfmlf>;
impl RfmlfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfmlf {
        match self.bits {
            false => Rfmlf::_0,
            true => Rfmlf::_1,
        }
    }
    #[doc = "No receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfmlf::_0
    }
    #[doc = "Receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfmlf::_1
    }
}
#[doc = "Field `RFMLF` writer - Receive FIFO Message Lost Flag"]
pub type RfmlfW<'a, REG> = crate::BitWriter<'a, REG, Rfmlf>;
impl<'a, REG> RfmlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfmlf::_0)
    }
    #[doc = "Receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfmlf::_1)
    }
}
#[doc = "Receive FIFO Full Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffst {
    #[doc = "0: Receive FIFO is not full"]
    _0 = 0,
    #[doc = "1: Receive FIFO is full (4 unread messages)"]
    _1 = 1,
}
impl From<Rffst> for bool {
    #[inline(always)]
    fn from(variant: Rffst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFST` reader - Receive FIFO Full Status Flag"]
pub type RffstR = crate::BitReader<Rffst>;
impl RffstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffst {
        match self.bits {
            false => Rffst::_0,
            true => Rffst::_1,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rffst::_0
    }
    #[doc = "Receive FIFO is full (4 unread messages)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rffst::_1
    }
}
#[doc = "Receive FIFO Buffer Warning Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfwst {
    #[doc = "0: Receive FIFO is not buffer warning"]
    _0 = 0,
    #[doc = "1: Receive FIFO is buffer warning (3 unread messages)"]
    _1 = 1,
}
impl From<Rfwst> for bool {
    #[inline(always)]
    fn from(variant: Rfwst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFWST` reader - Receive FIFO Buffer Warning Status Flag"]
pub type RfwstR = crate::BitReader<Rfwst>;
impl RfwstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfwst {
        match self.bits {
            false => Rfwst::_0,
            true => Rfwst::_1,
        }
    }
    #[doc = "Receive FIFO is not buffer warning"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfwst::_0
    }
    #[doc = "Receive FIFO is buffer warning (3 unread messages)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfwst::_1
    }
}
#[doc = "Receive FIFO Empty Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfest {
    #[doc = "0: Unread message in receive FIFO"]
    _0 = 0,
    #[doc = "1: No unread message in receive FIFO"]
    _1 = 1,
}
impl From<Rfest> for bool {
    #[inline(always)]
    fn from(variant: Rfest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEST` reader - Receive FIFO Empty Status Flag"]
pub type RfestR = crate::BitReader<Rfest>;
impl RfestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfest {
        match self.bits {
            false => Rfest::_0,
            true => Rfest::_1,
        }
    }
    #[doc = "Unread message in receive FIFO"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfest::_0
    }
    #[doc = "No unread message in receive FIFO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfest::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Receive FIFO Unread Message Number Status"]
    #[inline(always)]
    pub fn rfust(&self) -> RfustR {
        RfustR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub fn rfmlf(&self) -> RfmlfR {
        RfmlfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Status Flag"]
    #[inline(always)]
    pub fn rffst(&self) -> RffstR {
        RffstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Warning Status Flag"]
    #[inline(always)]
    pub fn rfwst(&self) -> RfwstR {
        RfwstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Empty Status Flag"]
    #[inline(always)]
    pub fn rfest(&self) -> RfestR {
        RfestR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCR")
            .field("rfest", &self.rfest())
            .field("rfwst", &self.rfwst())
            .field("rffst", &self.rffst())
            .field("rfmlf", &self.rfmlf())
            .field("rfust", &self.rfust())
            .field("rfe", &self.rfe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RfeW<RfcrSpec> {
        RfeW::new(self, 0)
    }
    #[doc = "Bit 4 - Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub fn rfmlf(&mut self) -> RfmlfW<RfcrSpec> {
        RfmlfW::new(self, 4)
    }
}
#[doc = "Receive FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcrSpec;
impl crate::RegisterSpec for RfcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rfcr::R`](R) reader structure"]
impl crate::Readable for RfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcr::W`](W) writer structure"]
impl crate::Writable for RfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFCR to value 0x80"]
impl crate::Resettable for RfcrSpec {
    const RESET_VALUE: u8 = 0x80;
}

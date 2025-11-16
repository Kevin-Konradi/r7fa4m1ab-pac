#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "NEWDATA Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ndst {
    #[doc = "0: No mailbox with NEWDATA bit = 1"]
    _0 = 0,
    #[doc = "1: Mailbox(es) with NEWDATA bit = 1"]
    _1 = 1,
}
impl From<Ndst> for bool {
    #[inline(always)]
    fn from(variant: Ndst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDST` reader - NEWDATA Status Flag"]
pub type NdstR = crate::BitReader<Ndst>;
impl NdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ndst {
        match self.bits {
            false => Ndst::_0,
            true => Ndst::_1,
        }
    }
    #[doc = "No mailbox with NEWDATA bit = 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ndst::_0
    }
    #[doc = "Mailbox(es) with NEWDATA bit = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ndst::_1
    }
}
#[doc = "SENTDATA Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdst {
    #[doc = "0: No mailbox with SENTDATA bit = 1"]
    _0 = 0,
    #[doc = "1: Mailbox(es) with SENTDATA bit = 1"]
    _1 = 1,
}
impl From<Sdst> for bool {
    #[inline(always)]
    fn from(variant: Sdst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDST` reader - SENTDATA Status Flag"]
pub type SdstR = crate::BitReader<Sdst>;
impl SdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdst {
        match self.bits {
            false => Sdst::_0,
            true => Sdst::_1,
        }
    }
    #[doc = "No mailbox with SENTDATA bit = 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdst::_0
    }
    #[doc = "Mailbox(es) with SENTDATA bit = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdst::_1
    }
}
#[doc = "Receive FIFO Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfst {
    #[doc = "0: No message in receive FIFO (empty)"]
    _0 = 0,
    #[doc = "1: Message in receive FIFO"]
    _1 = 1,
}
impl From<Rfst> for bool {
    #[inline(always)]
    fn from(variant: Rfst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFST` reader - Receive FIFO Status Flag"]
pub type RfstR = crate::BitReader<Rfst>;
impl RfstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfst {
        match self.bits {
            false => Rfst::_0,
            true => Rfst::_1,
        }
    }
    #[doc = "No message in receive FIFO (empty)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfst::_0
    }
    #[doc = "Message in receive FIFO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfst::_1
    }
}
#[doc = "Transmit FIFO Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfst {
    #[doc = "0: Transmit FIFO is full"]
    _0 = 0,
    #[doc = "1: Transmit FIFO is not full"]
    _1 = 1,
}
impl From<Tfst> for bool {
    #[inline(always)]
    fn from(variant: Tfst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFST` reader - Transmit FIFO Status Flag"]
pub type TfstR = crate::BitReader<Tfst>;
impl TfstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfst {
        match self.bits {
            false => Tfst::_0,
            true => Tfst::_1,
        }
    }
    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfst::_0
    }
    #[doc = "Transmit FIFO is not full"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfst::_1
    }
}
#[doc = "Normal Mailbox Message Lost Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmlst {
    #[doc = "0: No mailbox with MSGLOST bit = 1"]
    _0 = 0,
    #[doc = "1: Mailbox(es) with MSGLOST bit = 1"]
    _1 = 1,
}
impl From<Nmlst> for bool {
    #[inline(always)]
    fn from(variant: Nmlst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMLST` reader - Normal Mailbox Message Lost Status Flag"]
pub type NmlstR = crate::BitReader<Nmlst>;
impl NmlstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmlst {
        match self.bits {
            false => Nmlst::_0,
            true => Nmlst::_1,
        }
    }
    #[doc = "No mailbox with MSGLOST bit = 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmlst::_0
    }
    #[doc = "Mailbox(es) with MSGLOST bit = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmlst::_1
    }
}
#[doc = "FIFO Mailbox Message Lost Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmlst {
    #[doc = "0: RFMLF bit = 0"]
    _0 = 0,
    #[doc = "1: RFMLF bit = 1"]
    _1 = 1,
}
impl From<Fmlst> for bool {
    #[inline(always)]
    fn from(variant: Fmlst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMLST` reader - FIFO Mailbox Message Lost Status Flag"]
pub type FmlstR = crate::BitReader<Fmlst>;
impl FmlstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmlst {
        match self.bits {
            false => Fmlst::_0,
            true => Fmlst::_1,
        }
    }
    #[doc = "RFMLF bit = 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fmlst::_0
    }
    #[doc = "RFMLF bit = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fmlst::_1
    }
}
#[doc = "Transmission Abort Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tabst {
    #[doc = "0: No mailbox with TRMABT bit = 1"]
    _0 = 0,
    #[doc = "1: Mailbox(es) with TRMABT bit = 1"]
    _1 = 1,
}
impl From<Tabst> for bool {
    #[inline(always)]
    fn from(variant: Tabst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TABST` reader - Transmission Abort Status Flag"]
pub type TabstR = crate::BitReader<Tabst>;
impl TabstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tabst {
        match self.bits {
            false => Tabst::_0,
            true => Tabst::_1,
        }
    }
    #[doc = "No mailbox with TRMABT bit = 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tabst::_0
    }
    #[doc = "Mailbox(es) with TRMABT bit = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tabst::_1
    }
}
#[doc = "Error Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Est {
    #[doc = "0: No error occurred"]
    _0 = 0,
    #[doc = "1: Error occurred"]
    _1 = 1,
}
impl From<Est> for bool {
    #[inline(always)]
    fn from(variant: Est) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EST` reader - Error Status Flag"]
pub type EstR = crate::BitReader<Est>;
impl EstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Est {
        match self.bits {
            false => Est::_0,
            true => Est::_1,
        }
    }
    #[doc = "No error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Est::_0
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Est::_1
    }
}
#[doc = "CAN Reset Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstst {
    #[doc = "0: Not in CAN reset mode"]
    _0 = 0,
    #[doc = "1: In CAN reset mode"]
    _1 = 1,
}
impl From<Rstst> for bool {
    #[inline(always)]
    fn from(variant: Rstst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTST` reader - CAN Reset Status Flag"]
pub type RststR = crate::BitReader<Rstst>;
impl RststR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstst {
        match self.bits {
            false => Rstst::_0,
            true => Rstst::_1,
        }
    }
    #[doc = "Not in CAN reset mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rstst::_0
    }
    #[doc = "In CAN reset mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rstst::_1
    }
}
#[doc = "CAN Halt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hltst {
    #[doc = "0: Not in CAN halt mode"]
    _0 = 0,
    #[doc = "1: In CAN halt mode"]
    _1 = 1,
}
impl From<Hltst> for bool {
    #[inline(always)]
    fn from(variant: Hltst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HLTST` reader - CAN Halt Status Flag"]
pub type HltstR = crate::BitReader<Hltst>;
impl HltstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hltst {
        match self.bits {
            false => Hltst::_0,
            true => Hltst::_1,
        }
    }
    #[doc = "Not in CAN halt mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hltst::_0
    }
    #[doc = "In CAN halt mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hltst::_1
    }
}
#[doc = "CAN Sleep Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpst {
    #[doc = "0: Not in CAN sleep mode"]
    _0 = 0,
    #[doc = "1: In CAN sleep mode"]
    _1 = 1,
}
impl From<Slpst> for bool {
    #[inline(always)]
    fn from(variant: Slpst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPST` reader - CAN Sleep Status Flag"]
pub type SlpstR = crate::BitReader<Slpst>;
impl SlpstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpst {
        match self.bits {
            false => Slpst::_0,
            true => Slpst::_1,
        }
    }
    #[doc = "Not in CAN sleep mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slpst::_0
    }
    #[doc = "In CAN sleep mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slpst::_1
    }
}
#[doc = "Error-Passive Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epst {
    #[doc = "0: Not in error-passive state"]
    _0 = 0,
    #[doc = "1: In error-passive state"]
    _1 = 1,
}
impl From<Epst> for bool {
    #[inline(always)]
    fn from(variant: Epst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPST` reader - Error-Passive Status Flag"]
pub type EpstR = crate::BitReader<Epst>;
impl EpstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epst {
        match self.bits {
            false => Epst::_0,
            true => Epst::_1,
        }
    }
    #[doc = "Not in error-passive state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epst::_0
    }
    #[doc = "In error-passive state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epst::_1
    }
}
#[doc = "Bus-Off Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bost {
    #[doc = "0: Not in bus-off state"]
    _0 = 0,
    #[doc = "1: In bus-off state"]
    _1 = 1,
}
impl From<Bost> for bool {
    #[inline(always)]
    fn from(variant: Bost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOST` reader - Bus-Off Status Flag"]
pub type BostR = crate::BitReader<Bost>;
impl BostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bost {
        match self.bits {
            false => Bost::_0,
            true => Bost::_1,
        }
    }
    #[doc = "Not in bus-off state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bost::_0
    }
    #[doc = "In bus-off state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bost::_1
    }
}
#[doc = "Transmit Status Flag (transmitter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trmst {
    #[doc = "0: Bus idle or reception in progress"]
    _0 = 0,
    #[doc = "1: Transmission in progress or in bus-off state"]
    _1 = 1,
}
impl From<Trmst> for bool {
    #[inline(always)]
    fn from(variant: Trmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRMST` reader - Transmit Status Flag (transmitter)"]
pub type TrmstR = crate::BitReader<Trmst>;
impl TrmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trmst {
        match self.bits {
            false => Trmst::_0,
            true => Trmst::_1,
        }
    }
    #[doc = "Bus idle or reception in progress"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trmst::_0
    }
    #[doc = "Transmission in progress or in bus-off state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trmst::_1
    }
}
#[doc = "Receive Status Flag (receiver)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recst {
    #[doc = "0: Bus idle or transmission in progress"]
    _0 = 0,
    #[doc = "1: Reception in progress"]
    _1 = 1,
}
impl From<Recst> for bool {
    #[inline(always)]
    fn from(variant: Recst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECST` reader - Receive Status Flag (receiver)"]
pub type RecstR = crate::BitReader<Recst>;
impl RecstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Recst {
        match self.bits {
            false => Recst::_0,
            true => Recst::_1,
        }
    }
    #[doc = "Bus idle or transmission in progress"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Recst::_0
    }
    #[doc = "Reception in progress"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Recst::_1
    }
}
impl R {
    #[doc = "Bit 0 - NEWDATA Status Flag"]
    #[inline(always)]
    pub fn ndst(&self) -> NdstR {
        NdstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SENTDATA Status Flag"]
    #[inline(always)]
    pub fn sdst(&self) -> SdstR {
        SdstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Status Flag"]
    #[inline(always)]
    pub fn rfst(&self) -> RfstR {
        RfstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO Status Flag"]
    #[inline(always)]
    pub fn tfst(&self) -> TfstR {
        TfstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn nmlst(&self) -> NmlstR {
        NmlstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn fmlst(&self) -> FmlstR {
        FmlstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Abort Status Flag"]
    #[inline(always)]
    pub fn tabst(&self) -> TabstR {
        TabstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Status Flag"]
    #[inline(always)]
    pub fn est(&self) -> EstR {
        EstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN Reset Status Flag"]
    #[inline(always)]
    pub fn rstst(&self) -> RststR {
        RststR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAN Halt Status Flag"]
    #[inline(always)]
    pub fn hltst(&self) -> HltstR {
        HltstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CAN Sleep Status Flag"]
    #[inline(always)]
    pub fn slpst(&self) -> SlpstR {
        SlpstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error-Passive Status Flag"]
    #[inline(always)]
    pub fn epst(&self) -> EpstR {
        EpstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bus-Off Status Flag"]
    #[inline(always)]
    pub fn bost(&self) -> BostR {
        BostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Flag (transmitter)"]
    #[inline(always)]
    pub fn trmst(&self) -> TrmstR {
        TrmstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Flag (receiver)"]
    #[inline(always)]
    pub fn recst(&self) -> RecstR {
        RecstR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR")
            .field("recst", &self.recst())
            .field("trmst", &self.trmst())
            .field("bost", &self.bost())
            .field("epst", &self.epst())
            .field("slpst", &self.slpst())
            .field("hltst", &self.hltst())
            .field("rstst", &self.rstst())
            .field("est", &self.est())
            .field("tabst", &self.tabst())
            .field("fmlst", &self.fmlst())
            .field("nmlst", &self.nmlst())
            .field("tfst", &self.tfst())
            .field("rfst", &self.rfst())
            .field("sdst", &self.sdst())
            .field("ndst", &self.ndst())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`reset()` method sets STR to value 0x0500"]
impl crate::Resettable for StrSpec {
    const RESET_VALUE: u16 = 0x0500;
}

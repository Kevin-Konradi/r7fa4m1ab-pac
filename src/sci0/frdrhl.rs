#[doc = "Register `FRDRHL` reader"]
pub type R = crate::R<FrdrhlSpec>;
#[doc = "Field `RDAT` reader - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type RdatR = crate::FieldReader<u16>;
#[doc = "Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpb {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<Mpb> for bool {
    #[inline(always)]
    fn from(variant: Mpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPB` reader - Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
pub type MpbR = crate::BitReader<Mpb>;
impl MpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpb {
        match self.bits {
            false => Mpb::_0,
            true => Mpb::_1,
        }
    }
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpb::_0
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpb::_1
    }
}
#[doc = "Receive data ready flag (It is same as SSR.DR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    #[doc = "0: Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
    _0 = 0,
    #[doc = "1: Next receive data has not been received for a period after normal completed receiving."]
    _1 = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DR` reader - Receive data ready flag (It is same as SSR.DR)"]
pub type DrR = crate::BitReader<Dr>;
impl DrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dr {
        match self.bits {
            false => Dr::_0,
            true => Dr::_1,
        }
    }
    #[doc = "Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dr::_0
    }
    #[doc = "Next receive data has not been received for a period after normal completed receiving."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dr::_1
    }
}
#[doc = "Parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: No parity error occurred at the first data of FRDRH and FRDRL."]
    _0 = 0,
    #[doc = "1: A parity error has occurred at the first data of FRDRH and FRDRL."]
    _1 = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Parity error flag"]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            false => Per::_0,
            true => Per::_1,
        }
    }
    #[doc = "No parity error occurred at the first data of FRDRH and FRDRL."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Per::_0
    }
    #[doc = "A parity error has occurred at the first data of FRDRH and FRDRL."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Per::_1
    }
}
#[doc = "Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer {
    #[doc = "0: No framing error occurred at the first data of FRDRH and FRDRL."]
    _0 = 0,
    #[doc = "1: A framing error has occurred at the first data of FRDRH and FRDRL."]
    _1 = 1,
}
impl From<Fer> for bool {
    #[inline(always)]
    fn from(variant: Fer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER` reader - Framing error flag"]
pub type FerR = crate::BitReader<Fer>;
impl FerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fer {
        match self.bits {
            false => Fer::_0,
            true => Fer::_1,
        }
    }
    #[doc = "No framing error occurred at the first data of FRDRH and FRDRL."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fer::_0
    }
    #[doc = "A framing error has occurred at the first data of FRDRH and FRDRL."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fer::_1
    }
}
#[doc = "Overrun error flag (It is same as SSR.ORER)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orer {
    #[doc = "0: No overrun error occurred."]
    _0 = 0,
    #[doc = "1: An overrun error has occurred."]
    _1 = 1,
}
impl From<Orer> for bool {
    #[inline(always)]
    fn from(variant: Orer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORER` reader - Overrun error flag (It is same as SSR.ORER)"]
pub type OrerR = crate::BitReader<Orer>;
impl OrerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orer {
        match self.bits {
            false => Orer::_0,
            true => Orer::_1,
        }
    }
    #[doc = "No overrun error occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orer::_0
    }
    #[doc = "An overrun error has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orer::_1
    }
}
#[doc = "Receive FIFO data full flag (It is same as SSR.RDF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdf {
    #[doc = "0: The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
    _0 = 0,
    #[doc = "1: The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
    _1 = 1,
}
impl From<Rdf> for bool {
    #[inline(always)]
    fn from(variant: Rdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDF` reader - Receive FIFO data full flag (It is same as SSR.RDF)"]
pub type RdfR = crate::BitReader<Rdf>;
impl RdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdf {
        match self.bits {
            false => Rdf::_0,
            true => Rdf::_1,
        }
    }
    #[doc = "The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdf::_0
    }
    #[doc = "The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdf::_1
    }
}
impl R {
    #[doc = "Bits 0:8 - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn rdat(&self) -> RdatR {
        RdatR::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 9 - Multi-processor bit flag (Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive data ready flag (It is same as SSR.DR)"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Parity error flag"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Framing error flag"]
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun error flag (It is same as SSR.ORER)"]
    #[inline(always)]
    pub fn orer(&self) -> OrerR {
        OrerR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO data full flag (It is same as SSR.RDF)"]
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDRHL")
            .field("rdf", &self.rdf())
            .field("orer", &self.orer())
            .field("fer", &self.fer())
            .field("per", &self.per())
            .field("dr", &self.dr())
            .field("mpb", &self.mpb())
            .field("rdat", &self.rdat())
            .finish()
    }
}
#[doc = "Receive FIFO Data Register HL\n\nYou can [`read`](crate::Reg::read) this register and get [`frdrhl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrdrhlSpec;
impl crate::RegisterSpec for FrdrhlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frdrhl::R`](R) reader structure"]
impl crate::Readable for FrdrhlSpec {}
#[doc = "`reset()` method sets FRDRHL to value 0"]
impl crate::Resettable for FrdrhlSpec {}

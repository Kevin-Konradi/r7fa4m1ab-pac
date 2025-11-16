#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fm {
    #[doc = "0: Non-FIFO mode(Selects o TDR/RDR for communication)"]
    _0 = 0,
    #[doc = "1: FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    _1 = 1,
}
impl From<Fm> for bool {
    #[inline(always)]
    fn from(variant: Fm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FM` reader - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FmR = crate::BitReader<Fm>;
impl FmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fm {
        match self.bits {
            false => Fm::_0,
            true => Fm::_1,
        }
    }
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fm::_0
    }
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fm::_1
    }
}
#[doc = "Field `FM` writer - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FmW<'a, REG> = crate::BitWriter<'a, REG, Fm>;
impl<'a, REG> FmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fm::_0)
    }
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fm::_1)
    }
}
#[doc = "Receive FIFO Data Register Reset (Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfrst {
    #[doc = "0: The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FRDRH and FRDRL register are made 0"]
    _1 = 1,
}
impl From<Rfrst> for bool {
    #[inline(always)]
    fn from(variant: Rfrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRST` reader - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type RfrstR = crate::BitReader<Rfrst>;
impl RfrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfrst {
        match self.bits {
            false => Rfrst::_0,
            true => Rfrst::_1,
        }
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfrst::_0
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfrst::_1
    }
}
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type RfrstW<'a, REG> = crate::BitWriter<'a, REG, Rfrst>;
impl<'a, REG> RfrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_0)
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_1)
    }
}
#[doc = "Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfrst {
    #[doc = "0: The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FTDRH and FTDRL register are made 0"]
    _1 = 1,
}
impl From<Tfrst> for bool {
    #[inline(always)]
    fn from(variant: Tfrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRST` reader - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type TfrstR = crate::BitReader<Tfrst>;
impl TfrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfrst {
        match self.bits {
            false => Tfrst::_0,
            true => Tfrst::_1,
        }
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfrst::_0
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfrst::_1
    }
}
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type TfrstW<'a, REG> = crate::BitWriter<'a, REG, Tfrst>;
impl<'a, REG> TfrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_0)
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_1)
    }
}
#[doc = "Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dres {
    #[doc = "0: reception data full interrupt (RXI)"]
    _0 = 0,
    #[doc = "1: receive error interrupt (ERI)"]
    _1 = 1,
}
impl From<Dres> for bool {
    #[inline(always)]
    fn from(variant: Dres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRES` reader - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
pub type DresR = crate::BitReader<Dres>;
impl DresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dres {
        match self.bits {
            false => Dres::_0,
            true => Dres::_1,
        }
    }
    #[doc = "reception data full interrupt (RXI)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dres::_0
    }
    #[doc = "receive error interrupt (ERI)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dres::_1
    }
}
#[doc = "Field `DRES` writer - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
pub type DresW<'a, REG> = crate::BitWriter<'a, REG, Dres>;
impl<'a, REG> DresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reception data full interrupt (RXI)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::_0)
    }
    #[doc = "receive error interrupt (ERI)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::_1)
    }
}
#[doc = "Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ttrg {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
    #[doc = "1: Triger number n (n= 0-15)"]
    Others = 1,
}
impl From<Ttrg> for u8 {
    #[inline(always)]
    fn from(variant: Ttrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ttrg {
    type Ux = u8;
}
impl crate::IsEnum for Ttrg {}
#[doc = "Field `TTRG` reader - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TtrgR = crate::FieldReader<Ttrg>;
impl TtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttrg {
        match self.bits {
            0 => Ttrg::_0000,
            _ => Ttrg::Others,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Ttrg::_0000
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ttrg::Others)
    }
}
#[doc = "Field `TTRG` writer - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TtrgW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ttrg, crate::Safe>;
impl<'a, REG> TtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ttrg::_0000)
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ttrg::Others)
    }
}
#[doc = "Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtrg {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
    #[doc = "1: Triger number n (n= 0-15)"]
    Others = 1,
}
impl From<Rtrg> for u8 {
    #[inline(always)]
    fn from(variant: Rtrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtrg {
    type Ux = u8;
}
impl crate::IsEnum for Rtrg {}
#[doc = "Field `RTRG` reader - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RtrgR = crate::FieldReader<Rtrg>;
impl RtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtrg {
        match self.bits {
            0 => Rtrg::_0000,
            _ => Rtrg::Others,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Rtrg::_0000
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rtrg::Others)
    }
}
#[doc = "Field `RTRG` writer - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RtrgW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rtrg, crate::Safe>;
impl<'a, REG> RtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Rtrg::_0000)
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rtrg::Others)
    }
}
#[doc = "RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstrg {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
    #[doc = "1: Triger number n (n= 0-15)"]
    Others = 1,
}
impl From<Rstrg> for u8 {
    #[inline(always)]
    fn from(variant: Rstrg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstrg {
    type Ux = u8;
}
impl crate::IsEnum for Rstrg {}
#[doc = "Field `RSTRG` reader - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RstrgR = crate::FieldReader<Rstrg>;
impl RstrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstrg {
        match self.bits {
            0 => Rstrg::_0000,
            _ => Rstrg::Others,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Rstrg::_0000
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rstrg::Others)
    }
}
#[doc = "Field `RSTRG` writer - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RstrgW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rstrg, crate::Safe>;
impl<'a, REG> RstrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Rstrg::_0000)
    }
    #[doc = "Triger number n (n= 0-15)"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rstrg::Others)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn fm(&self) -> FmR {
        FmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn rfrst(&self) -> RfrstR {
        RfrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn tfrst(&self) -> TfrstR {
        TfrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn ttrg(&self) -> TtrgR {
        TtrgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rtrg(&self) -> RtrgR {
        RtrgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rstrg(&self) -> RstrgR {
        RstrgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("rstrg", &self.rstrg())
            .field("rtrg", &self.rtrg())
            .field("ttrg", &self.ttrg())
            .field("dres", &self.dres())
            .field("tfrst", &self.tfrst())
            .field("rfrst", &self.rfrst())
            .field("fm", &self.fm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn fm(&mut self) -> FmW<FcrSpec> {
        FmW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn rfrst(&mut self) -> RfrstW<FcrSpec> {
        RfrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn tfrst(&mut self) -> TfrstW<FcrSpec> {
        TfrstW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub fn dres(&mut self) -> DresW<FcrSpec> {
        DresW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn ttrg(&mut self) -> TtrgW<FcrSpec> {
        TtrgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rtrg(&mut self) -> RtrgW<FcrSpec> {
        RtrgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rstrg(&mut self) -> RstrgW<FcrSpec> {
        RstrgW::new(self, 12)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0xf800"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u16 = 0xf800;
}

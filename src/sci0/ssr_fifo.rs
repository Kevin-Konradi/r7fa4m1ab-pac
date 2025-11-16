#[doc = "Register `SSR_FIFO` reader"]
pub type R = crate::R<SsrFifoSpec>;
#[doc = "Register `SSR_FIFO` writer"]
pub type W = crate::W<SsrFifoSpec>;
#[doc = "Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    #[doc = "0: Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    _0 = 0,
    #[doc = "1: Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    _1 = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DR` reader - Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dr::_0
    }
    #[doc = "Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dr::_1
    }
}
#[doc = "Field `DR` writer - Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
pub type DrW<'a, REG> = crate::BitWriter0C<'a, REG, Dr>;
impl<'a, REG> DrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dr::_0)
    }
    #[doc = "Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dr::_1)
    }
}
#[doc = "Transmit End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tend {
    #[doc = "0: A character is being transmitted or standing by for transmission."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed."]
    _1 = 1,
}
impl From<Tend> for bool {
    #[inline(always)]
    fn from(variant: Tend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TendR = crate::BitReader<Tend>;
impl TendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tend {
        match self.bits {
            false => Tend::_0,
            true => Tend::_1,
        }
    }
    #[doc = "A character is being transmitted or standing by for transmission."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tend::_0
    }
    #[doc = "Character transfer has been completed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tend::_1
    }
}
#[doc = "Field `TEND` writer - Transmit End Flag"]
pub type TendW<'a, REG> = crate::BitWriter0C<'a, REG, Tend>;
impl<'a, REG> TendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A character is being transmitted or standing by for transmission."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_0)
    }
    #[doc = "Character transfer has been completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_1)
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "0: No parity error occurred."]
    _0 = 0,
    #[doc = "1: A parity error has occurred."]
    _1 = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Parity Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "No parity error occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Per::_0
    }
    #[doc = "A parity error has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Per::_1
    }
}
#[doc = "Field `PER` writer - Parity Error Flag"]
pub type PerW<'a, REG> = crate::BitWriter0C<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_0)
    }
    #[doc = "A parity error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_1)
    }
}
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer {
    #[doc = "0: No framing error occurred."]
    _0 = 0,
    #[doc = "1: A framing error has occurred."]
    _1 = 1,
}
impl From<Fer> for bool {
    #[inline(always)]
    fn from(variant: Fer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FER` reader - Framing Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "No framing error occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fer::_0
    }
    #[doc = "A framing error has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fer::_1
    }
}
#[doc = "Field `FER` writer - Framing Error Flag"]
pub type FerW<'a, REG> = crate::BitWriter0C<'a, REG, Fer>;
impl<'a, REG> FerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No framing error occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fer::_0)
    }
    #[doc = "A framing error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fer::_1)
    }
}
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orer {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: An overrun error has occurred"]
    _1 = 1,
}
impl From<Orer> for bool {
    #[inline(always)]
    fn from(variant: Orer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orer::_0
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orer::_1
    }
}
#[doc = "Field `ORER` writer - Overrun Error Flag"]
pub type OrerW<'a, REG> = crate::BitWriter0C<'a, REG, Orer>;
impl<'a, REG> OrerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_0)
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_1)
    }
}
#[doc = "Receive FIFO data full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdf {
    #[doc = "0: The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    _0 = 0,
    #[doc = "1: The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    _1 = 1,
}
impl From<Rdf> for bool {
    #[inline(always)]
    fn from(variant: Rdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDF` reader - Receive FIFO data full flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
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
    #[doc = "The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdf::_0
    }
    #[doc = "The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdf::_1
    }
}
#[doc = "Field `RDF` writer - Receive FIFO data full flag"]
pub type RdfW<'a, REG> = crate::BitWriter0C<'a, REG, Rdf>;
impl<'a, REG> RdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The quantity of receive data written in FRDR falls below the specified receive triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_0)
    }
    #[doc = "The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_1)
    }
}
#[doc = "Transmit FIFO data empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdfe {
    #[doc = "0: The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    _0 = 0,
    #[doc = "1: The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    _1 = 1,
}
impl From<Tdfe> for bool {
    #[inline(always)]
    fn from(variant: Tdfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDFE` reader - Transmit FIFO data empty flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TdfeR = crate::BitReader<Tdfe>;
impl TdfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdfe {
        match self.bits {
            false => Tdfe::_0,
            true => Tdfe::_1,
        }
    }
    #[doc = "The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdfe::_0
    }
    #[doc = "The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdfe::_1
    }
}
#[doc = "Field `TDFE` writer - Transmit FIFO data empty flag"]
pub type TdfeW<'a, REG> = crate::BitWriter0C<'a, REG, Tdfe>;
impl<'a, REG> TdfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdfe::_0)
    }
    #[doc = "The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdfe::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TendR {
        TendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> OrerR {
        OrerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO data full flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO data empty flag"]
    #[inline(always)]
    pub fn tdfe(&self) -> TdfeR {
        TdfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR_FIFO").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Ready flag (Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<SsrFifoSpec> {
        DrW::new(self, 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&mut self) -> TendW<SsrFifoSpec> {
        TendW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrFifoSpec> {
        PerW::new(self, 3)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn fer(&mut self) -> FerW<SsrFifoSpec> {
        FerW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrFifoSpec> {
        OrerW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO data full flag"]
    #[inline(always)]
    pub fn rdf(&mut self) -> RdfW<SsrFifoSpec> {
        RdfW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit FIFO data empty flag"]
    #[inline(always)]
    pub fn tdfe(&mut self) -> TdfeW<SsrFifoSpec> {
        TdfeW::new(self, 7)
    }
}
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrFifoSpec;
impl crate::RegisterSpec for SsrFifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ssr_fifo::R`](R) reader structure"]
impl crate::Readable for SsrFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ssr_fifo::W`](W) writer structure"]
impl crate::Writable for SsrFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xfd;
}
#[doc = "`reset()` method sets SSR_FIFO to value 0x80"]
impl crate::Resettable for SsrFifoSpec {
    const RESET_VALUE: u8 = 0x80;
}

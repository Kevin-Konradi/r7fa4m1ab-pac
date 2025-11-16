#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cke {
    #[doc = "0: The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _00 = 0,
    #[doc = "1: The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _01 = 1,
    #[doc = "2: The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)"]
    Others = 2,
}
impl From<Cke> for u8 {
    #[inline(always)]
    fn from(variant: Cke) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cke {
    type Ux = u8;
}
impl crate::IsEnum for Cke {}
#[doc = "Field `CKE` reader - Clock Enable"]
pub type CkeR = crate::FieldReader<Cke>;
impl CkeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cke {
        match self.bits {
            0 => Cke::_00,
            1 => Cke::_01,
            _ => Cke::Others,
        }
    }
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cke::_00
    }
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cke::_01
    }
    #[doc = "The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cke::Others)
    }
}
#[doc = "Field `CKE` writer - Clock Enable"]
pub type CkeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cke, crate::Safe>;
impl<'a, REG> CkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_00)
    }
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::_01)
    }
    #[doc = "The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cke::Others)
    }
}
#[doc = "Transmit End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teie {
    #[doc = "0: SCI_TEI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCI_TEI interrupt request is enabled"]
    _1 = 1,
}
impl From<Teie> for bool {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Enable"]
pub type TeieR = crate::BitReader<Teie>;
impl TeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teie {
        match self.bits {
            false => Teie::_0,
            true => Teie::_1,
        }
    }
    #[doc = "SCI_TEI interrupt request is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teie::_0
    }
    #[doc = "SCI_TEI interrupt request is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teie::_1
    }
}
#[doc = "Field `TEIE` writer - Transmit End Interrupt Enable"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_TEI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_0)
    }
    #[doc = "SCI_TEI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_1)
    }
}
#[doc = "Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpie {
    #[doc = "0: Normal reception"]
    _0 = 0,
    #[doc = "1: When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    _1 = 1,
}
impl From<Mpie> for bool {
    #[inline(always)]
    fn from(variant: Mpie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPIE` reader - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
pub type MpieR = crate::BitReader<Mpie>;
impl MpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpie {
        match self.bits {
            false => Mpie::_0,
            true => Mpie::_1,
        }
    }
    #[doc = "Normal reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpie::_0
    }
    #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpie::_1
    }
}
#[doc = "Field `MPIE` writer - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
pub type MpieW<'a, REG> = crate::BitWriter<'a, REG, Mpie>;
impl<'a, REG> MpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpie::_0)
    }
    #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpie::_1)
    }
}
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Serial reception is disabled"]
    _0 = 0,
    #[doc = "1: Serial reception is enabled"]
    _1 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receive Enable"]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::_0,
            true => Re::_1,
        }
    }
    #[doc = "Serial reception is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Re::_0
    }
    #[doc = "Serial reception is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Re::_1
    }
}
#[doc = "Field `RE` writer - Receive Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial reception is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_0)
    }
    #[doc = "Serial reception is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_1)
    }
}
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Serial transmission is disabled"]
    _0 = 0,
    #[doc = "1: Serial transmission is enabled"]
    _1 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmit Enable"]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::_0,
            true => Te::_1,
        }
    }
    #[doc = "Serial transmission is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Te::_0
    }
    #[doc = "Serial transmission is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Te::_1
    }
}
#[doc = "Field `TE` writer - Transmit Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial transmission is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_0)
    }
    #[doc = "Serial transmission is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_1)
    }
}
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: SCI_RXI and SCI_ERI interrupt requests are disabled"]
    _0 = 0,
    #[doc = "1: SCI_RXI and SCI_ERI interrupt requests are enabled"]
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: SCI_TXI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCI_TXI interrupt request is enabled"]
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::_0,
            true => Tie::_1,
        }
    }
    #[doc = "SCI_TXI interrupt request is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    #[doc = "SCI_TXI interrupt request is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_TXI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    #[doc = "SCI_TXI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&self) -> CkeR {
        CkeR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    pub fn mpie(&self) -> MpieR {
        MpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("tie", &self.tie())
            .field("rie", &self.rie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("mpie", &self.mpie())
            .field("teie", &self.teie())
            .field("cke", &self.cke())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&mut self) -> CkeW<ScrSpec> {
        CkeW::new(self, 0)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<ScrSpec> {
        TeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    pub fn mpie(&mut self) -> MpieW<ScrSpec> {
        MpieW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<ScrSpec> {
        ReW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<ScrSpec> {
        TeW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<ScrSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<ScrSpec> {
        TieW::new(self, 7)
    }
}
#[doc = "Serial Control Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}

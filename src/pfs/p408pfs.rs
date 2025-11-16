#[doc = "Register `P408PFS` reader"]
pub type R = crate::R<P408pfsSpec>;
#[doc = "Register `P408PFS` writer"]
pub type W = crate::W<P408pfsSpec>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Podr> for bool {
    #[inline(always)]
    fn from(variant: Podr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PodrR = crate::BitReader<Podr>;
impl PodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Podr {
        match self.bits {
            false => Podr::_0,
            true => Podr::_1,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PodrW<'a, REG> = crate::BitWriter<'a, REG, Podr>;
impl<'a, REG> PodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_1)
    }
}
#[doc = "Port Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<Pidr> for bool {
    #[inline(always)]
    fn from(variant: Pidr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR` reader - Port Input Data"]
pub type PidrR = crate::BitReader<Pidr>;
impl PidrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr {
        match self.bits {
            false => Pidr::_0,
            true => Pidr::_1,
        }
    }
    #[doc = "Low input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr::_0
    }
    #[doc = "High input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr::_1
    }
}
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr {
    #[doc = "0: Input (Functions as an input pin.)"]
    _0 = 0,
    #[doc = "1: Output (Functions as an output pin.)"]
    _1 = 1,
}
impl From<Pdr> for bool {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PdrR = crate::BitReader<Pdr>;
impl PdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr {
        match self.bits {
            false => Pdr::_0,
            true => Pdr::_1,
        }
    }
    #[doc = "Input (Functions as an input pin.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr::_0
    }
    #[doc = "Output (Functions as an output pin.)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PdrW<'a, REG> = crate::BitWriter<'a, REG, Pdr>;
impl<'a, REG> PdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (Functions as an input pin.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_0)
    }
    #[doc = "Output (Functions as an output pin.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_1)
    }
}
#[doc = "Pull-up Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: Disables an input pull-up."]
    _0 = 0,
    #[doc = "1: Enables an input pull-up."]
    _1 = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::_0,
            true => Pcr::_1,
        }
    }
    #[doc = "Disables an input pull-up."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcr::_0
    }
    #[doc = "Enables an input pull-up."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcr::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables an input pull-up."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_0)
    }
    #[doc = "Enables an input pull-up."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_1)
    }
}
#[doc = "N-Channel Open Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncodr {
    #[doc = "0: CMOS output"]
    _0 = 0,
    #[doc = "1: NMOS open-drain output"]
    _1 = 1,
}
impl From<Ncodr> for bool {
    #[inline(always)]
    fn from(variant: Ncodr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open Drain Control"]
pub type NcodrR = crate::BitReader<Ncodr>;
impl NcodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncodr {
        match self.bits {
            false => Ncodr::_0,
            true => Ncodr::_1,
        }
    }
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ncodr::_0
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ncodr::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open Drain Control"]
pub type NcodrW<'a, REG> = crate::BitWriter<'a, REG, Ncodr>;
impl<'a, REG> NcodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_0)
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_1)
    }
}
#[doc = "Drive Strength Control Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscr {
    #[doc = "0: Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
    _0 = 0,
    #[doc = "1: Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
    _1 = 1,
}
impl From<Dscr> for bool {
    #[inline(always)]
    fn from(variant: Dscr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCR` reader - Drive Strength Control Register"]
pub type DscrR = crate::BitReader<Dscr>;
impl DscrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscr {
        match self.bits {
            false => Dscr::_0,
            true => Dscr::_1,
        }
    }
    #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscr::_0
    }
    #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscr::_1
    }
}
#[doc = "Field `DSCR` writer - Drive Strength Control Register"]
pub type DscrW<'a, REG> = crate::BitWriter<'a, REG, Dscr>;
impl<'a, REG> DscrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr::_0)
    }
    #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr::_1)
    }
}
#[doc = "Drive Strength Control Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscr1 {
    #[doc = "0: Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
    _0 = 0,
    #[doc = "1: Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
    _1 = 1,
}
impl From<Dscr1> for bool {
    #[inline(always)]
    fn from(variant: Dscr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSCR1` reader - Drive Strength Control Register"]
pub type Dscr1R = crate::BitReader<Dscr1>;
impl Dscr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dscr1 {
        match self.bits {
            false => Dscr1::_0,
            true => Dscr1::_1,
        }
    }
    #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscr1::_0
    }
    #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscr1::_1
    }
}
#[doc = "Field `DSCR1` writer - Drive Strength Control Register"]
pub type Dscr1W<'a, REG> = crate::BitWriter<'a, REG, Dscr1>;
impl<'a, REG> Dscr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr1::_0)
    }
    #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscr1::_1)
    }
}
#[doc = "Event on Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eor {
    #[doc = "0: Do not care"]
    _0 = 0,
    #[doc = "1: Detect rising edge"]
    _1 = 1,
}
impl From<Eor> for bool {
    #[inline(always)]
    fn from(variant: Eor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOR` reader - Event on Rising"]
pub type EorR = crate::BitReader<Eor>;
impl EorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eor {
        match self.bits {
            false => Eor::_0,
            true => Eor::_1,
        }
    }
    #[doc = "Do not care"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eor::_0
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eor::_1
    }
}
#[doc = "Field `EOR` writer - Event on Rising"]
pub type EorW<'a, REG> = crate::BitWriter<'a, REG, Eor>;
impl<'a, REG> EorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not care"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eor::_0)
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eor::_1)
    }
}
#[doc = "Event on Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eof {
    #[doc = "0: Do not care"]
    _0 = 0,
    #[doc = "1: Detect falling edge"]
    _1 = 1,
}
impl From<Eof> for bool {
    #[inline(always)]
    fn from(variant: Eof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` reader - Event on Falling"]
pub type EofR = crate::BitReader<Eof>;
impl EofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eof {
        match self.bits {
            false => Eof::_0,
            true => Eof::_1,
        }
    }
    #[doc = "Do not care"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eof::_0
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eof::_1
    }
}
#[doc = "Field `EOF` writer - Event on Falling"]
pub type EofW<'a, REG> = crate::BitWriter<'a, REG, Eof>;
impl<'a, REG> EofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not care"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::_0)
    }
    #[doc = "Detect falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::_1)
    }
}
#[doc = "IRQ input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    #[doc = "0: Not used as IRQn input pin"]
    _0 = 0,
    #[doc = "1: Used as IRQn input pin"]
    _1 = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - IRQ input enable"]
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::_0,
            true => Isel::_1,
        }
    }
    #[doc = "Not used as IRQn input pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isel::_0
    }
    #[doc = "Used as IRQn input pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isel::_1
    }
}
#[doc = "Field `ISEL` writer - IRQ input enable"]
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used as IRQn input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_0)
    }
    #[doc = "Used as IRQn input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_1)
    }
}
#[doc = "Analog Input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asel {
    #[doc = "0: Used other than as analog pin"]
    _0 = 0,
    #[doc = "1: Used as analog pin"]
    _1 = 1,
}
impl From<Asel> for bool {
    #[inline(always)]
    fn from(variant: Asel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASEL` reader - Analog Input enable"]
pub type AselR = crate::BitReader<Asel>;
impl AselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asel {
        match self.bits {
            false => Asel::_0,
            true => Asel::_1,
        }
    }
    #[doc = "Used other than as analog pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asel::_0
    }
    #[doc = "Used as analog pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asel::_1
    }
}
#[doc = "Field `ASEL` writer - Analog Input enable"]
pub type AselW<'a, REG> = crate::BitWriter<'a, REG, Asel>;
impl<'a, REG> AselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Used other than as analog pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_0)
    }
    #[doc = "Used as analog pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_1)
    }
}
#[doc = "Port Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmr {
    #[doc = "0: Uses the pin as a general I/O pin."]
    _0 = 0,
    #[doc = "1: Uses the pin as an I/O port for peripheral functions."]
    _1 = 1,
}
impl From<Pmr> for bool {
    #[inline(always)]
    fn from(variant: Pmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMR` reader - Port Mode Control"]
pub type PmrR = crate::BitReader<Pmr>;
impl PmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmr {
        match self.bits {
            false => Pmr::_0,
            true => Pmr::_1,
        }
    }
    #[doc = "Uses the pin as a general I/O pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pmr::_0
    }
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pmr::_1
    }
}
#[doc = "Field `PMR` writer - Port Mode Control"]
pub type PmrW<'a, REG> = crate::BitWriter<'a, REG, Pmr>;
impl<'a, REG> PmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Uses the pin as a general I/O pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_0)
    }
    #[doc = "Uses the pin as an I/O port for peripheral functions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_1)
    }
}
#[doc = "Field `PSEL` reader - Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
pub type PselR = crate::FieldReader;
#[doc = "Field `PSEL` writer - Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Input Data"]
    #[inline(always)]
    pub fn pidr(&self) -> PidrR {
        PidrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NcodrR {
        NcodrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(&self) -> DscrR {
        DscrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr1(&self) -> Dscr1R {
        Dscr1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event on Rising"]
    #[inline(always)]
    pub fn eor(&self) -> EorR {
        EorR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event on Falling"]
    #[inline(always)]
    pub fn eof(&self) -> EofR {
        EofR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ input enable"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Input enable"]
    #[inline(always)]
    pub fn asel(&self) -> AselR {
        AselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    pub fn pmr(&self) -> PmrR {
        PmrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P408PFS")
            .field("psel", &self.psel())
            .field("pmr", &self.pmr())
            .field("asel", &self.asel())
            .field("isel", &self.isel())
            .field("eof", &self.eof())
            .field("eor", &self.eor())
            .field("dscr1", &self.dscr1())
            .field("dscr", &self.dscr())
            .field("ncodr", &self.ncodr())
            .field("pcr", &self.pcr())
            .field("pdr", &self.pdr())
            .field("pidr", &self.pidr())
            .field("podr", &self.podr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<P408pfsSpec> {
        PodrW::new(self, 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<P408pfsSpec> {
        PdrW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<P408pfsSpec> {
        PcrW::new(self, 4)
    }
    #[doc = "Bit 6 - N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(&mut self) -> NcodrW<P408pfsSpec> {
        NcodrW::new(self, 6)
    }
    #[doc = "Bit 10 - Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DscrW<P408pfsSpec> {
        DscrW::new(self, 10)
    }
    #[doc = "Bit 11 - Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr1(&mut self) -> Dscr1W<P408pfsSpec> {
        Dscr1W::new(self, 11)
    }
    #[doc = "Bit 12 - Event on Rising"]
    #[inline(always)]
    pub fn eor(&mut self) -> EorW<P408pfsSpec> {
        EorW::new(self, 12)
    }
    #[doc = "Bit 13 - Event on Falling"]
    #[inline(always)]
    pub fn eof(&mut self) -> EofW<P408pfsSpec> {
        EofW::new(self, 13)
    }
    #[doc = "Bit 14 - IRQ input enable"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<P408pfsSpec> {
        IselW::new(self, 14)
    }
    #[doc = "Bit 15 - Analog Input enable"]
    #[inline(always)]
    pub fn asel(&mut self) -> AselW<P408pfsSpec> {
        AselW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Mode Control"]
    #[inline(always)]
    pub fn pmr(&mut self) -> PmrW<P408pfsSpec> {
        PmrW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Port Function Select These bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<P408pfsSpec> {
        PselW::new(self, 24)
    }
}
#[doc = "P408 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p408pfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p408pfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P408pfsSpec;
impl crate::RegisterSpec for P408pfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p408pfs::R`](R) reader structure"]
impl crate::Readable for P408pfsSpec {}
#[doc = "`write(|w| ..)` method takes [`p408pfs::W`](W) writer structure"]
impl crate::Writable for P408pfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P408PFS to value 0"]
impl crate::Resettable for P408pfsSpec {}

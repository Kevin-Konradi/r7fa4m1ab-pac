#[doc = "Register `SMR` reader"]
pub type R = crate::R<SmrSpec>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SmrSpec>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: PCLK clock"]
    _00 = 0,
    #[doc = "1: PCLK/4 clock"]
    _01 = 1,
    #[doc = "2: PCLK/16 clock"]
    _10 = 2,
    #[doc = "3: PCLK/64 clock"]
    _11 = 3,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Clock Select"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_00,
            1 => Cks::_01,
            2 => Cks::_10,
            3 => Cks::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK clock"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cks::_00
    }
    #[doc = "PCLK/4 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cks::_01
    }
    #[doc = "PCLK/16 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cks::_10
    }
    #[doc = "PCLK/64 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cks::_11
    }
}
#[doc = "Field `CKS` writer - Clock Select"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_00)
    }
    #[doc = "PCLK/4 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_01)
    }
    #[doc = "PCLK/16 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_10)
    }
    #[doc = "PCLK/64 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_11)
    }
}
#[doc = "Multi-Processor Mode (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mp {
    #[doc = "0: Multi-processor communications function is disabled"]
    _0 = 0,
    #[doc = "1: Multi-processor communications function is enabled"]
    _1 = 1,
}
impl From<Mp> for bool {
    #[inline(always)]
    fn from(variant: Mp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MP` reader - Multi-Processor Mode (Valid only in asynchronous mode)"]
pub type MpR = crate::BitReader<Mp>;
impl MpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mp {
        match self.bits {
            false => Mp::_0,
            true => Mp::_1,
        }
    }
    #[doc = "Multi-processor communications function is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mp::_0
    }
    #[doc = "Multi-processor communications function is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mp::_1
    }
}
#[doc = "Field `MP` writer - Multi-Processor Mode (Valid only in asynchronous mode)"]
pub type MpW<'a, REG> = crate::BitWriter<'a, REG, Mp>;
impl<'a, REG> MpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-processor communications function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mp::_0)
    }
    #[doc = "Multi-processor communications function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mp::_1)
    }
}
#[doc = "Stop Bit Length (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: 1 stop bit"]
    _0 = 0,
    #[doc = "1: 2 stop bits"]
    _1 = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop Bit Length (Valid only in asynchronous mode)"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::_0,
            true => Stop::_1,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stop::_0
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stop::_1
    }
}
#[doc = "Field `STOP` writer - Stop Bit Length (Valid only in asynchronous mode)"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_0)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_1)
    }
}
#[doc = "Parity Mode (Valid only when the PE bit is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: Selects even parity"]
    _0 = 0,
    #[doc = "1: Selects odd parity"]
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Parity Mode (Valid only when the PE bit is 1)"]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    #[doc = "Selects even parity"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    #[doc = "Selects odd parity"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
#[doc = "Field `PM` writer - Parity Mode (Valid only when the PE bit is 1)"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects even parity"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    #[doc = "Selects odd parity"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
#[doc = "Parity Enable (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    _0 = 0,
    #[doc = "1: The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
    _1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Enable (Valid only in asynchronous mode)"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::_0,
            true => Pe::_1,
        }
    }
    #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pe::_0
    }
    #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pe::_1
    }
}
#[doc = "Field `PE` writer - Parity Enable (Valid only in asynchronous mode)"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_0)
    }
    #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_1)
    }
}
#[doc = "Character Length (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chr {
    #[doc = "0: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    _0 = 0,
    #[doc = "1: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    _1 = 1,
}
impl From<Chr> for bool {
    #[inline(always)]
    fn from(variant: Chr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHR` reader - Character Length (Valid only in asynchronous mode)"]
pub type ChrR = crate::BitReader<Chr>;
impl ChrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chr {
        match self.bits {
            false => Chr::_0,
            true => Chr::_1,
        }
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chr::_0
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chr::_1
    }
}
#[doc = "Field `CHR` writer - Character Length (Valid only in asynchronous mode)"]
pub type ChrW<'a, REG> = crate::BitWriter<'a, REG, Chr>;
impl<'a, REG> ChrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chr::_0)
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chr::_1)
    }
}
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    #[doc = "0: Asynchronous mode or simple I2C mode"]
    _0 = 0,
    #[doc = "1: Clock synchronous mode"]
    _1 = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Communication Mode"]
pub type CmR = crate::BitReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            false => Cm::_0,
            true => Cm::_1,
        }
    }
    #[doc = "Asynchronous mode or simple I2C mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cm::_0
    }
    #[doc = "Clock synchronous mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cm::_1
    }
}
#[doc = "Field `CM` writer - Communication Mode"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode or simple I2C mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::_0)
    }
    #[doc = "Clock synchronous mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Bit Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Character Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn chr(&self) -> ChrR {
        ChrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Communication Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMR")
            .field("cm", &self.cm())
            .field("chr", &self.chr())
            .field("pe", &self.pe())
            .field("pm", &self.pm())
            .field("stop", &self.stop())
            .field("mp", &self.mp())
            .field("cks", &self.cks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SmrSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bit 2 - Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn mp(&mut self) -> MpW<SmrSpec> {
        MpW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop Bit Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<SmrSpec> {
        StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<SmrSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<SmrSpec> {
        PeW::new(self, 5)
    }
    #[doc = "Bit 6 - Character Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn chr(&mut self) -> ChrW<SmrSpec> {
        ChrW::new(self, 6)
    }
    #[doc = "Bit 7 - Communication Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<SmrSpec> {
        CmW::new(self, 7)
    }
}
#[doc = "Serial Mode Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmrSpec;
impl crate::RegisterSpec for SmrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SmrSpec {}

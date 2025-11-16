#[doc = "Register `PIPE%sCTR` reader"]
pub type R = crate::R<PipectrSpec>;
#[doc = "Register `PIPE%sCTR` writer"]
pub type W = crate::W<PipectrSpec>;
#[doc = "Response PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    #[doc = "0: NAK response"]
    _00 = 0,
    #[doc = "1: BUF response (depending on the buffer state)"]
    _01 = 1,
    #[doc = "2: STALL response"]
    _10 = 2,
    #[doc = "3: STALL response"]
    _11 = 3,
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid {
    type Ux = u8;
}
impl crate::IsEnum for Pid {}
#[doc = "Field `PID` reader - Response PID"]
pub type PidR = crate::FieldReader<Pid>;
impl PidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            0 => Pid::_00,
            1 => Pid::_01,
            2 => Pid::_10,
            3 => Pid::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "NAK response"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pid::_00
    }
    #[doc = "BUF response (depending on the buffer state)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pid::_01
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Pid::_10
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Pid::_11
    }
}
#[doc = "Field `PID` writer - Response PID"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pid, crate::Safe>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NAK response"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_00)
    }
    #[doc = "BUF response (depending on the buffer state)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_01)
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_10)
    }
    #[doc = "STALL response"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_11)
    }
}
#[doc = "Pipe Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbusy {
    #[doc = "0: Pipe n not in use for the transaction"]
    _0 = 0,
    #[doc = "1: Pipe n in use for the transaction."]
    _1 = 1,
}
impl From<Pbusy> for bool {
    #[inline(always)]
    fn from(variant: Pbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBUSY` reader - Pipe Busy"]
pub type PbusyR = crate::BitReader<Pbusy>;
impl PbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbusy {
        match self.bits {
            false => Pbusy::_0,
            true => Pbusy::_1,
        }
    }
    #[doc = "Pipe n not in use for the transaction"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pbusy::_0
    }
    #[doc = "Pipe n in use for the transaction."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pbusy::_1
    }
}
#[doc = "Sequence Toggle Bit Confirmation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqmon {
    #[doc = "0: DATA0"]
    _0 = 0,
    #[doc = "1: DATA1"]
    _1 = 1,
}
impl From<Sqmon> for bool {
    #[inline(always)]
    fn from(variant: Sqmon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQMON` reader - Sequence Toggle Bit Confirmation"]
pub type SqmonR = crate::BitReader<Sqmon>;
impl SqmonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqmon {
        match self.bits {
            false => Sqmon::_0,
            true => Sqmon::_1,
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqmon::_0
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqmon::_1
    }
}
#[doc = "Sequence Toggle Bit Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqset {
    #[doc = "0: Write disabled"]
    _0 = 0,
    #[doc = "1: Specifies DATA1."]
    _1 = 1,
}
impl From<Sqset> for bool {
    #[inline(always)]
    fn from(variant: Sqset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQSET` reader - Sequence Toggle Bit Set"]
pub type SqsetR = crate::BitReader<Sqset>;
impl SqsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqset {
        match self.bits {
            false => Sqset::_0,
            true => Sqset::_1,
        }
    }
    #[doc = "Write disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqset::_0
    }
    #[doc = "Specifies DATA1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqset::_1
    }
}
#[doc = "Field `SQSET` writer - Sequence Toggle Bit Set"]
pub type SqsetW<'a, REG> = crate::BitWriter<'a, REG, Sqset>;
impl<'a, REG> SqsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sqset::_0)
    }
    #[doc = "Specifies DATA1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sqset::_1)
    }
}
#[doc = "Sequence Toggle Bit Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqclr {
    #[doc = "0: Write disabled"]
    _0 = 0,
    #[doc = "1: Specifies DATA0."]
    _1 = 1,
}
impl From<Sqclr> for bool {
    #[inline(always)]
    fn from(variant: Sqclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQCLR` reader - Sequence Toggle Bit Clear"]
pub type SqclrR = crate::BitReader<Sqclr>;
impl SqclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqclr {
        match self.bits {
            false => Sqclr::_0,
            true => Sqclr::_1,
        }
    }
    #[doc = "Write disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqclr::_0
    }
    #[doc = "Specifies DATA0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqclr::_1
    }
}
#[doc = "Field `SQCLR` writer - Sequence Toggle Bit Clear"]
pub type SqclrW<'a, REG> = crate::BitWriter<'a, REG, Sqclr>;
impl<'a, REG> SqclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sqclr::_0)
    }
    #[doc = "Specifies DATA0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sqclr::_1)
    }
}
#[doc = "Auto Buffer Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aclrm {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled (all buffers are initialized)"]
    _1 = 1,
}
impl From<Aclrm> for bool {
    #[inline(always)]
    fn from(variant: Aclrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLRM` reader - Auto Buffer Clear Mode"]
pub type AclrmR = crate::BitReader<Aclrm>;
impl AclrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aclrm {
        match self.bits {
            false => Aclrm::_0,
            true => Aclrm::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aclrm::_0
    }
    #[doc = "Enabled (all buffers are initialized)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aclrm::_1
    }
}
#[doc = "Field `ACLRM` writer - Auto Buffer Clear Mode"]
pub type AclrmW<'a, REG> = crate::BitWriter<'a, REG, Aclrm>;
impl<'a, REG> AclrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aclrm::_0)
    }
    #[doc = "Enabled (all buffers are initialized)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aclrm::_1)
    }
}
#[doc = "Auto Response Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atrepm {
    #[doc = "0: Auto response disabled."]
    _0 = 0,
    #[doc = "1: Auto response enabled."]
    _1 = 1,
}
impl From<Atrepm> for bool {
    #[inline(always)]
    fn from(variant: Atrepm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATREPM` reader - Auto Response Mode"]
pub type AtrepmR = crate::BitReader<Atrepm>;
impl AtrepmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atrepm {
        match self.bits {
            false => Atrepm::_0,
            true => Atrepm::_1,
        }
    }
    #[doc = "Auto response disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Atrepm::_0
    }
    #[doc = "Auto response enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Atrepm::_1
    }
}
#[doc = "Field `ATREPM` writer - Auto Response Mode"]
pub type AtrepmW<'a, REG> = crate::BitWriter<'a, REG, Atrepm>;
impl<'a, REG> AtrepmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto response disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Atrepm::_0)
    }
    #[doc = "Auto response enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Atrepm::_1)
    }
}
#[doc = "Transmit Buffer Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inbufm {
    #[doc = "0: No data to be transmitted is in the FIFO buffer"]
    _0 = 0,
    #[doc = "1: Data to be transmitted is in the FIFO buffer"]
    _1 = 1,
}
impl From<Inbufm> for bool {
    #[inline(always)]
    fn from(variant: Inbufm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INBUFM` reader - Transmit Buffer Monitor"]
pub type InbufmR = crate::BitReader<Inbufm>;
impl InbufmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inbufm {
        match self.bits {
            false => Inbufm::_0,
            true => Inbufm::_1,
        }
    }
    #[doc = "No data to be transmitted is in the FIFO buffer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inbufm::_0
    }
    #[doc = "Data to be transmitted is in the FIFO buffer"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inbufm::_1
    }
}
#[doc = "Buffer Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsts {
    #[doc = "0: Buffer access by the CPU is disabled."]
    _0 = 0,
    #[doc = "1: Buffer access by the CPU is enabled."]
    _1 = 1,
}
impl From<Bsts> for bool {
    #[inline(always)]
    fn from(variant: Bsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSTS` reader - Buffer Status"]
pub type BstsR = crate::BitReader<Bsts>;
impl BstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsts {
        match self.bits {
            false => Bsts::_0,
            true => Bsts::_1,
        }
    }
    #[doc = "Buffer access by the CPU is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsts::_0
    }
    #[doc = "Buffer access by the CPU is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsts::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Response PID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Pipe Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequence Toggle Bit Confirmation"]
    #[inline(always)]
    pub fn sqmon(&self) -> SqmonR {
        SqmonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(&self) -> SqsetR {
        SqsetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(&self) -> SqclrR {
        SqclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto Buffer Clear Mode"]
    #[inline(always)]
    pub fn aclrm(&self) -> AclrmR {
        AclrmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Auto Response Mode"]
    #[inline(always)]
    pub fn atrepm(&self) -> AtrepmR {
        AtrepmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Buffer Monitor"]
    #[inline(always)]
    pub fn inbufm(&self) -> InbufmR {
        InbufmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer Status"]
    #[inline(always)]
    pub fn bsts(&self) -> BstsR {
        BstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPECTR")
            .field("bsts", &self.bsts())
            .field("inbufm", &self.inbufm())
            .field("atrepm", &self.atrepm())
            .field("aclrm", &self.aclrm())
            .field("sqclr", &self.sqclr())
            .field("sqset", &self.sqset())
            .field("sqmon", &self.sqmon())
            .field("pbusy", &self.pbusy())
            .field("pid", &self.pid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Response PID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<PipectrSpec> {
        PidW::new(self, 0)
    }
    #[doc = "Bit 7 - Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(&mut self) -> SqsetW<PipectrSpec> {
        SqsetW::new(self, 7)
    }
    #[doc = "Bit 8 - Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(&mut self) -> SqclrW<PipectrSpec> {
        SqclrW::new(self, 8)
    }
    #[doc = "Bit 9 - Auto Buffer Clear Mode"]
    #[inline(always)]
    pub fn aclrm(&mut self) -> AclrmW<PipectrSpec> {
        AclrmW::new(self, 9)
    }
    #[doc = "Bit 10 - Auto Response Mode"]
    #[inline(always)]
    pub fn atrepm(&mut self) -> AtrepmW<PipectrSpec> {
        AtrepmW::new(self, 10)
    }
}
#[doc = "Pipe %s Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipectrSpec;
impl crate::RegisterSpec for PipectrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipectr::R`](R) reader structure"]
impl crate::Readable for PipectrSpec {}
#[doc = "`write(|w| ..)` method takes [`pipectr::W`](W) writer structure"]
impl crate::Writable for PipectrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPE%sCTR to value 0"]
impl crate::Resettable for PipectrSpec {}

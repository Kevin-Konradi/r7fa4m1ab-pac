#[doc = "Register `DCPCTR` reader"]
pub type R = crate::R<DcpctrSpec>;
#[doc = "Register `DCPCTR` writer"]
pub type W = crate::W<DcpctrSpec>;
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
#[doc = "Control Transfer End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpl {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Completion of control transfer is enabled."]
    _1 = 1,
}
impl From<Ccpl> for bool {
    #[inline(always)]
    fn from(variant: Ccpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPL` reader - Control Transfer End Enable"]
pub type CcplR = crate::BitReader<Ccpl>;
impl CcplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpl {
        match self.bits {
            false => Ccpl::_0,
            true => Ccpl::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccpl::_0
    }
    #[doc = "Completion of control transfer is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccpl::_1
    }
}
#[doc = "Field `CCPL` writer - Control Transfer End Enable"]
pub type CcplW<'a, REG> = crate::BitWriter<'a, REG, Ccpl>;
impl<'a, REG> CcplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpl::_0)
    }
    #[doc = "Completion of control transfer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpl::_1)
    }
}
#[doc = "Pipe Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbusy {
    #[doc = "0: DCP is not used for the transaction."]
    _0 = 0,
    #[doc = "1: DCP is used for the transaction."]
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
    #[doc = "DCP is not used for the transaction."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pbusy::_0
    }
    #[doc = "DCP is used for the transaction."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pbusy::_1
    }
}
#[doc = "Sequence Toggle Bit Monitor\n\nValue on reset: 1"]
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
#[doc = "Field `SQMON` reader - Sequence Toggle Bit Monitor"]
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
    #[doc = "0: Invalid"]
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
#[doc = "Field `SQSET` writer - Sequence Toggle Bit Set"]
pub type SqsetW<'a, REG> = crate::BitWriter<'a, REG, Sqset>;
impl<'a, REG> SqsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
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
    #[doc = "0: Invalid"]
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
#[doc = "Field `SQCLR` writer - Sequence Toggle Bit Clear"]
pub type SqclrW<'a, REG> = crate::BitWriter<'a, REG, Sqclr>;
impl<'a, REG> SqclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
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
#[doc = "SUREQ Bit Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sureqclr {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Clears the SUREQ bit to 0."]
    _1 = 1,
}
impl From<Sureqclr> for bool {
    #[inline(always)]
    fn from(variant: Sureqclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUREQCLR` reader - SUREQ Bit Clear"]
pub type SureqclrR = crate::BitReader<Sureqclr>;
impl SureqclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sureqclr {
        match self.bits {
            false => Sureqclr::_0,
            true => Sureqclr::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sureqclr::_0
    }
    #[doc = "Clears the SUREQ bit to 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sureqclr::_1
    }
}
#[doc = "Field `SUREQCLR` writer - SUREQ Bit Clear"]
pub type SureqclrW<'a, REG> = crate::BitWriter<'a, REG, Sureqclr>;
impl<'a, REG> SureqclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sureqclr::_0)
    }
    #[doc = "Clears the SUREQ bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sureqclr::_1)
    }
}
#[doc = "Setup Token Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sureq {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: Transmits the setup packet."]
    _1 = 1,
}
impl From<Sureq> for bool {
    #[inline(always)]
    fn from(variant: Sureq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUREQ` reader - Setup Token Transmission"]
pub type SureqR = crate::BitReader<Sureq>;
impl SureqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sureq {
        match self.bits {
            false => Sureq::_0,
            true => Sureq::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sureq::_0
    }
    #[doc = "Transmits the setup packet."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sureq::_1
    }
}
#[doc = "Field `SUREQ` writer - Setup Token Transmission"]
pub type SureqW<'a, REG> = crate::BitWriter<'a, REG, Sureq>;
impl<'a, REG> SureqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sureq::_0)
    }
    #[doc = "Transmits the setup packet."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sureq::_1)
    }
}
#[doc = "Buffer Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsts {
    #[doc = "0: Buffer access is disabled."]
    _0 = 0,
    #[doc = "1: Buffer access is enabled."]
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
    #[doc = "Buffer access is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsts::_0
    }
    #[doc = "Buffer access is enabled."]
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
    #[doc = "Bit 2 - Control Transfer End Enable"]
    #[inline(always)]
    pub fn ccpl(&self) -> CcplR {
        CcplR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequence Toggle Bit Monitor"]
    #[inline(always)]
    pub fn sqmon(&self) -> SqmonR {
        SqmonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - SUREQ Bit Clear"]
    #[inline(always)]
    pub fn sureqclr(&self) -> SureqclrR {
        SureqclrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Setup Token Transmission"]
    #[inline(always)]
    pub fn sureq(&self) -> SureqR {
        SureqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer Status"]
    #[inline(always)]
    pub fn bsts(&self) -> BstsR {
        BstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPCTR")
            .field("bsts", &self.bsts())
            .field("sureq", &self.sureq())
            .field("sureqclr", &self.sureqclr())
            .field("sqmon", &self.sqmon())
            .field("pbusy", &self.pbusy())
            .field("ccpl", &self.ccpl())
            .field("pid", &self.pid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Response PID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<DcpctrSpec> {
        PidW::new(self, 0)
    }
    #[doc = "Bit 2 - Control Transfer End Enable"]
    #[inline(always)]
    pub fn ccpl(&mut self) -> CcplW<DcpctrSpec> {
        CcplW::new(self, 2)
    }
    #[doc = "Bit 7 - Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(&mut self) -> SqsetW<DcpctrSpec> {
        SqsetW::new(self, 7)
    }
    #[doc = "Bit 8 - Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(&mut self) -> SqclrW<DcpctrSpec> {
        SqclrW::new(self, 8)
    }
    #[doc = "Bit 11 - SUREQ Bit Clear"]
    #[inline(always)]
    pub fn sureqclr(&mut self) -> SureqclrW<DcpctrSpec> {
        SureqclrW::new(self, 11)
    }
    #[doc = "Bit 14 - Setup Token Transmission"]
    #[inline(always)]
    pub fn sureq(&mut self) -> SureqW<DcpctrSpec> {
        SureqW::new(self, 14)
    }
}
#[doc = "DCP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcpctrSpec;
impl crate::RegisterSpec for DcpctrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dcpctr::R`](R) reader structure"]
impl crate::Readable for DcpctrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcpctr::W`](W) writer structure"]
impl crate::Writable for DcpctrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCPCTR to value 0x40"]
impl crate::Resettable for DcpctrSpec {
    const RESET_VALUE: u16 = 0x40;
}

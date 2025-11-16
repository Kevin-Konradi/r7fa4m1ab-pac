#[doc = "Register `ICMR3` reader"]
pub type R = crate::R<Icmr3Spec>;
#[doc = "Register `ICMR3` writer"]
pub type W = crate::W<Icmr3Spec>;
#[doc = "Noise Filter Stage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nf {
    #[doc = "0: Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
    _00 = 0,
    #[doc = "1: Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
    _01 = 1,
    #[doc = "2: Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
    _10 = 2,
    #[doc = "3: Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
    _11 = 3,
}
impl From<Nf> for u8 {
    #[inline(always)]
    fn from(variant: Nf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nf {
    type Ux = u8;
}
impl crate::IsEnum for Nf {}
#[doc = "Field `NF` reader - Noise Filter Stage Selection"]
pub type NfR = crate::FieldReader<Nf>;
impl NfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nf {
        match self.bits {
            0 => Nf::_00,
            1 => Nf::_01,
            2 => Nf::_10,
            3 => Nf::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nf::_00
    }
    #[doc = "Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nf::_01
    }
    #[doc = "Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nf::_10
    }
    #[doc = "Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nf::_11
    }
}
#[doc = "Field `NF` writer - Noise Filter Stage Selection"]
pub type NfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nf, crate::Safe>;
impl<'a, REG> NfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_00)
    }
    #[doc = "Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_01)
    }
    #[doc = "Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_10)
    }
    #[doc = "Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_11)
    }
}
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackbr {
    #[doc = "0: A 0 is received as the acknowledge bit (ACK reception)."]
    _0 = 0,
    #[doc = "1: A 1 is received as the acknowledge bit (NACK reception)."]
    _1 = 1,
}
impl From<Ackbr> for bool {
    #[inline(always)]
    fn from(variant: Ackbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKBR` reader - Receive Acknowledge"]
pub type AckbrR = crate::BitReader<Ackbr>;
impl AckbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackbr {
        match self.bits {
            false => Ackbr::_0,
            true => Ackbr::_1,
        }
    }
    #[doc = "A 0 is received as the acknowledge bit (ACK reception)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackbr::_0
    }
    #[doc = "A 1 is received as the acknowledge bit (NACK reception)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackbr::_1
    }
}
#[doc = "Transmit Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackbt {
    #[doc = "0: A 0 is sent as the acknowledge bit (ACK transmission)."]
    _0 = 0,
    #[doc = "1: A 1 is sent as the acknowledge bit (NACK transmission)."]
    _1 = 1,
}
impl From<Ackbt> for bool {
    #[inline(always)]
    fn from(variant: Ackbt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKBT` reader - Transmit Acknowledge"]
pub type AckbtR = crate::BitReader<Ackbt>;
impl AckbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackbt {
        match self.bits {
            false => Ackbt::_0,
            true => Ackbt::_1,
        }
    }
    #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackbt::_0
    }
    #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackbt::_1
    }
}
#[doc = "Field `ACKBT` writer - Transmit Acknowledge"]
pub type AckbtW<'a, REG> = crate::BitWriter<'a, REG, Ackbt>;
impl<'a, REG> AckbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackbt::_0)
    }
    #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackbt::_1)
    }
}
#[doc = "ACKBT Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackwp {
    #[doc = "0: Modification of the ACKBT bit is disabled."]
    _0 = 0,
    #[doc = "1: Modification of the ACKBT bit is enabled."]
    _1 = 1,
}
impl From<Ackwp> for bool {
    #[inline(always)]
    fn from(variant: Ackwp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKWP` reader - ACKBT Write Protect"]
pub type AckwpR = crate::BitReader<Ackwp>;
impl AckwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackwp {
        match self.bits {
            false => Ackwp::_0,
            true => Ackwp::_1,
        }
    }
    #[doc = "Modification of the ACKBT bit is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackwp::_0
    }
    #[doc = "Modification of the ACKBT bit is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackwp::_1
    }
}
#[doc = "Field `ACKWP` writer - ACKBT Write Protect"]
pub type AckwpW<'a, REG> = crate::BitWriter<'a, REG, Ackwp>;
impl<'a, REG> AckwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Modification of the ACKBT bit is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackwp::_0)
    }
    #[doc = "Modification of the ACKBT bit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackwp::_1)
    }
}
#[doc = "RDRF Flag Set Timing Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrfs {
    #[doc = "0: The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    _0 = 0,
    #[doc = "1: The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)"]
    _1 = 1,
}
impl From<Rdrfs> for bool {
    #[inline(always)]
    fn from(variant: Rdrfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRFS` reader - RDRF Flag Set Timing Selection"]
pub type RdrfsR = crate::BitReader<Rdrfs>;
impl RdrfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdrfs {
        match self.bits {
            false => Rdrfs::_0,
            true => Rdrfs::_1,
        }
    }
    #[doc = "The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrfs::_0
    }
    #[doc = "The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdrfs::_1
    }
}
#[doc = "Field `RDRFS` writer - RDRF Flag Set Timing Selection"]
pub type RdrfsW<'a, REG> = crate::BitWriter<'a, REG, Rdrfs>;
impl<'a, REG> RdrfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrfs::_0)
    }
    #[doc = "The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrfs::_1)
    }
}
#[doc = "WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wait {
    #[doc = "0: No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    _0 = 0,
    #[doc = "1: WAIT (The period between ninth clock cycle and first clock cycle is held low.)"]
    _1 = 1,
}
impl From<Wait> for bool {
    #[inline(always)]
    fn from(variant: Wait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT` reader - WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
pub type WaitR = crate::BitReader<Wait>;
impl WaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wait {
        match self.bits {
            false => Wait::_0,
            true => Wait::_1,
        }
    }
    #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wait::_0
    }
    #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wait::_1
    }
}
#[doc = "Field `WAIT` writer - WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
pub type WaitW<'a, REG> = crate::BitWriter<'a, REG, Wait>;
impl<'a, REG> WaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_0)
    }
    #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::_1)
    }
}
#[doc = "SMBus/I2C Bus Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbs {
    #[doc = "0: The I2C bus is selected."]
    _0 = 0,
    #[doc = "1: The SMBus is selected."]
    _1 = 1,
}
impl From<Smbs> for bool {
    #[inline(always)]
    fn from(variant: Smbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBS` reader - SMBus/I2C Bus Selection"]
pub type SmbsR = crate::BitReader<Smbs>;
impl SmbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbs {
        match self.bits {
            false => Smbs::_0,
            true => Smbs::_1,
        }
    }
    #[doc = "The I2C bus is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smbs::_0
    }
    #[doc = "The SMBus is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smbs::_1
    }
}
#[doc = "Field `SMBS` writer - SMBus/I2C Bus Selection"]
pub type SmbsW<'a, REG> = crate::BitWriter<'a, REG, Smbs>;
impl<'a, REG> SmbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The I2C bus is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smbs::_0)
    }
    #[doc = "The SMBus is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smbs::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Noise Filter Stage Selection"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Receive Acknowledge"]
    #[inline(always)]
    pub fn ackbr(&self) -> AckbrR {
        AckbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge"]
    #[inline(always)]
    pub fn ackbt(&self) -> AckbtR {
        AckbtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACKBT Write Protect"]
    #[inline(always)]
    pub fn ackwp(&self) -> AckwpR {
        AckwpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDRF Flag Set Timing Selection"]
    #[inline(always)]
    pub fn rdrfs(&self) -> RdrfsR {
        RdrfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub fn smbs(&self) -> SmbsR {
        SmbsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICMR3")
            .field("smbs", &self.smbs())
            .field("wait", &self.wait())
            .field("rdrfs", &self.rdrfs())
            .field("ackwp", &self.ackwp())
            .field("ackbt", &self.ackbt())
            .field("ackbr", &self.ackbr())
            .field("nf", &self.nf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Noise Filter Stage Selection"]
    #[inline(always)]
    pub fn nf(&mut self) -> NfW<Icmr3Spec> {
        NfW::new(self, 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge"]
    #[inline(always)]
    pub fn ackbt(&mut self) -> AckbtW<Icmr3Spec> {
        AckbtW::new(self, 3)
    }
    #[doc = "Bit 4 - ACKBT Write Protect"]
    #[inline(always)]
    pub fn ackwp(&mut self) -> AckwpW<Icmr3Spec> {
        AckwpW::new(self, 4)
    }
    #[doc = "Bit 5 - RDRF Flag Set Timing Selection"]
    #[inline(always)]
    pub fn rdrfs(&mut self) -> RdrfsW<Icmr3Spec> {
        RdrfsW::new(self, 5)
    }
    #[doc = "Bit 6 - WAIT Note: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<Icmr3Spec> {
        WaitW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub fn smbs(&mut self) -> SmbsW<Icmr3Spec> {
        SmbsW::new(self, 7)
    }
}
#[doc = "I2C Bus Mode Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icmr3Spec;
impl crate::RegisterSpec for Icmr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icmr3::R`](R) reader structure"]
impl crate::Readable for Icmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`icmr3::W`](W) writer structure"]
impl crate::Writable for Icmr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICMR3 to value 0"]
impl crate::Resettable for Icmr3Spec {}

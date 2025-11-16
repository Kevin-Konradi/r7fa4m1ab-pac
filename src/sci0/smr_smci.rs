#[doc = "Register `SMR_SMCI` reader"]
pub type R = crate::R<SmrSmciSpec>;
#[doc = "Register `SMR_SMCI` writer"]
pub type W = crate::W<SmrSmciSpec>;
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
#[doc = "Base Clock Pulse (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcp {
    #[doc = "0: 93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    _00 = 0,
    #[doc = "1: 128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    _01 = 1,
    #[doc = "2: 186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    _10 = 2,
    #[doc = "3: 512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    _11 = 3,
}
impl From<Bcp> for u8 {
    #[inline(always)]
    fn from(variant: Bcp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcp {
    type Ux = u8;
}
impl crate::IsEnum for Bcp {}
#[doc = "Field `BCP` reader - Base Clock Pulse (Valid only in asynchronous mode)"]
pub type BcpR = crate::FieldReader<Bcp>;
impl BcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcp {
        match self.bits {
            0 => Bcp::_00,
            1 => Bcp::_01,
            2 => Bcp::_10,
            3 => Bcp::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bcp::_00
    }
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Bcp::_01
    }
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bcp::_10
    }
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Bcp::_11
    }
}
#[doc = "Field `BCP` writer - Base Clock Pulse (Valid only in asynchronous mode)"]
pub type BcpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcp, crate::Safe>;
impl<'a, REG> BcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::_00)
    }
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::_01)
    }
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::_10)
    }
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::_11)
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
    #[doc = "0: Setting Prohibited"]
    _0 = 0,
    #[doc = "1: Set this bit to 1 in smart card interface mode."]
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
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pe::_0
    }
    #[doc = "Set this bit to 1 in smart card interface mode."]
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
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_0)
    }
    #[doc = "Set this bit to 1 in smart card interface mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_1)
    }
}
#[doc = "Block Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blk {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: Block transfer mode operation"]
    _1 = 1,
}
impl From<Blk> for bool {
    #[inline(always)]
    fn from(variant: Blk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLK` reader - Block Transfer Mode"]
pub type BlkR = crate::BitReader<Blk>;
impl BlkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blk {
        match self.bits {
            false => Blk::_0,
            true => Blk::_1,
        }
    }
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blk::_0
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blk::_1
    }
}
#[doc = "Field `BLK` writer - Block Transfer Mode"]
pub type BlkW<'a, REG> = crate::BitWriter<'a, REG, Blk>;
impl<'a, REG> BlkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blk::_0)
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blk::_1)
    }
}
#[doc = "GSM Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gm {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: GSM mode operation"]
    _1 = 1,
}
impl From<Gm> for bool {
    #[inline(always)]
    fn from(variant: Gm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GM` reader - GSM Mode"]
pub type GmR = crate::BitReader<Gm>;
impl GmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gm {
        match self.bits {
            false => Gm::_0,
            true => Gm::_1,
        }
    }
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gm::_0
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gm::_1
    }
}
#[doc = "Field `GM` writer - GSM Mode"]
pub type GmW<'a, REG> = crate::BitWriter<'a, REG, Gm>;
impl<'a, REG> GmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gm::_0)
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gm::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn bcp(&self) -> BcpR {
        BcpR::new((self.bits >> 2) & 3)
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
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&self) -> BlkR {
        BlkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&self) -> GmR {
        GmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMR_SMCI")
            .field("gm", &self.gm())
            .field("blk", &self.blk())
            .field("pe", &self.pe())
            .field("pm", &self.pm())
            .field("bcp", &self.bcp())
            .field("cks", &self.cks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SmrSmciSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn bcp(&mut self) -> BcpW<SmrSmciSpec> {
        BcpW::new(self, 2)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<SmrSmciSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<SmrSmciSpec> {
        PeW::new(self, 5)
    }
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&mut self) -> BlkW<SmrSmciSpec> {
        BlkW::new(self, 6)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&mut self) -> GmW<SmrSmciSpec> {
        GmW::new(self, 7)
    }
}
#[doc = "Serial mode register (SCMR.SMIF = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`smr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmrSmciSpec;
impl crate::RegisterSpec for SmrSmciSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smr_smci::R`](R) reader structure"]
impl crate::Readable for SmrSmciSpec {}
#[doc = "`write(|w| ..)` method takes [`smr_smci::W`](W) writer structure"]
impl crate::Writable for SmrSmciSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMR_SMCI to value 0"]
impl crate::Resettable for SmrSmciSpec {}

#[doc = "Register `SPSR` reader"]
pub type R = crate::R<SpsrSpec>;
#[doc = "Register `SPSR` writer"]
pub type W = crate::W<SpsrSpec>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrf {
    #[doc = "0: No overrun error occurs"]
    _0 = 0,
    #[doc = "1: An overrun error occurs"]
    _1 = 1,
}
impl From<Ovrf> for bool {
    #[inline(always)]
    fn from(variant: Ovrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRF` reader - Overrun Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OvrfR = crate::BitReader<Ovrf>;
impl OvrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrf {
        match self.bits {
            false => Ovrf::_0,
            true => Ovrf::_1,
        }
    }
    #[doc = "No overrun error occurs"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrf::_0
    }
    #[doc = "An overrun error occurs"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrf::_1
    }
}
#[doc = "Field `OVRF` writer - Overrun Error Flag"]
pub type OvrfW<'a, REG> = crate::BitWriter0C<'a, REG, Ovrf>;
impl<'a, REG> OvrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun error occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_0)
    }
    #[doc = "An overrun error occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_1)
    }
}
#[doc = "SPI Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idlnf {
    #[doc = "0: SPI is in the idle state"]
    _0 = 0,
    #[doc = "1: SPI is in the transfer state"]
    _1 = 1,
}
impl From<Idlnf> for bool {
    #[inline(always)]
    fn from(variant: Idlnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLNF` reader - SPI Idle Flag"]
pub type IdlnfR = crate::BitReader<Idlnf>;
impl IdlnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idlnf {
        match self.bits {
            false => Idlnf::_0,
            true => Idlnf::_1,
        }
    }
    #[doc = "SPI is in the idle state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idlnf::_0
    }
    #[doc = "SPI is in the transfer state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idlnf::_1
    }
}
#[doc = "Mode Fault Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modf {
    #[doc = "0: Neither mode fault error nor underrun error occurs"]
    _0 = 0,
    #[doc = "1: A mode fault error or an underrun error occurs."]
    _1 = 1,
}
impl From<Modf> for bool {
    #[inline(always)]
    fn from(variant: Modf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ModfR = crate::BitReader<Modf>;
impl ModfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modf {
        match self.bits {
            false => Modf::_0,
            true => Modf::_1,
        }
    }
    #[doc = "Neither mode fault error nor underrun error occurs"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Modf::_0
    }
    #[doc = "A mode fault error or an underrun error occurs."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Modf::_1
    }
}
#[doc = "Field `MODF` writer - Mode Fault Error Flag"]
pub type ModfW<'a, REG> = crate::BitWriter0C<'a, REG, Modf>;
impl<'a, REG> ModfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Neither mode fault error nor underrun error occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Modf::_0)
    }
    #[doc = "A mode fault error or an underrun error occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Modf::_1)
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perf {
    #[doc = "0: No parity error occurs"]
    _0 = 0,
    #[doc = "1: A parity error occurs"]
    _1 = 1,
}
impl From<Perf> for bool {
    #[inline(always)]
    fn from(variant: Perf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERF` reader - Parity Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PerfR = crate::BitReader<Perf>;
impl PerfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perf {
        match self.bits {
            false => Perf::_0,
            true => Perf::_1,
        }
    }
    #[doc = "No parity error occurs"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Perf::_0
    }
    #[doc = "A parity error occurs"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Perf::_1
    }
}
#[doc = "Field `PERF` writer - Parity Error Flag"]
pub type PerfW<'a, REG> = crate::BitWriter0C<'a, REG, Perf>;
impl<'a, REG> PerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Perf::_0)
    }
    #[doc = "A parity error occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Perf::_1)
    }
}
#[doc = "Underrun Error Flag (When MODF is 0, This bit is invalid.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udrf {
    #[doc = "0: A mode fault error occurs (MODF=1)"]
    _0 = 0,
    #[doc = "1: An underrun error occurs (MODF=1)"]
    _1 = 1,
}
impl From<Udrf> for bool {
    #[inline(always)]
    fn from(variant: Udrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDRF` reader - Underrun Error Flag (When MODF is 0, This bit is invalid.)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type UdrfR = crate::BitReader<Udrf>;
impl UdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udrf {
        match self.bits {
            false => Udrf::_0,
            true => Udrf::_1,
        }
    }
    #[doc = "A mode fault error occurs (MODF=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Udrf::_0
    }
    #[doc = "An underrun error occurs (MODF=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Udrf::_1
    }
}
#[doc = "Field `UDRF` writer - Underrun Error Flag (When MODF is 0, This bit is invalid.)"]
pub type UdrfW<'a, REG> = crate::BitWriter0C<'a, REG, Udrf>;
impl<'a, REG> UdrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A mode fault error occurs (MODF=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Udrf::_0)
    }
    #[doc = "An underrun error occurs (MODF=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Udrf::_1)
    }
}
#[doc = "SPI Transmit Buffer Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sptef {
    #[doc = "0: Data found in the transmit buffer"]
    _0 = 0,
    #[doc = "1: No data in the transmit buffer"]
    _1 = 1,
}
impl From<Sptef> for bool {
    #[inline(always)]
    fn from(variant: Sptef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTEF` reader - SPI Transmit Buffer Empty Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SptefR = crate::BitReader<Sptef>;
impl SptefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sptef {
        match self.bits {
            false => Sptef::_0,
            true => Sptef::_1,
        }
    }
    #[doc = "Data found in the transmit buffer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sptef::_0
    }
    #[doc = "No data in the transmit buffer"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sptef::_1
    }
}
#[doc = "Field `SPTEF` writer - SPI Transmit Buffer Empty Flag"]
pub type SptefW<'a, REG> = crate::BitWriter0C<'a, REG, Sptef>;
impl<'a, REG> SptefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data found in the transmit buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sptef::_0)
    }
    #[doc = "No data in the transmit buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sptef::_1)
    }
}
#[doc = "SPI Receive Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprf {
    #[doc = "0: No valid data in SPDR"]
    _0 = 0,
    #[doc = "1: Valid data found in SPDR"]
    _1 = 1,
}
impl From<Sprf> for bool {
    #[inline(always)]
    fn from(variant: Sprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRF` reader - SPI Receive Buffer Full Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SprfR = crate::BitReader<Sprf>;
impl SprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sprf {
        match self.bits {
            false => Sprf::_0,
            true => Sprf::_1,
        }
    }
    #[doc = "No valid data in SPDR"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sprf::_0
    }
    #[doc = "Valid data found in SPDR"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sprf::_1
    }
}
#[doc = "Field `SPRF` writer - SPI Receive Buffer Full Flag"]
pub type SprfW<'a, REG> = crate::BitWriter0C<'a, REG, Sprf>;
impl<'a, REG> SprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No valid data in SPDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sprf::_0)
    }
    #[doc = "Valid data found in SPDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sprf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun Error Flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OvrfR {
        OvrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Idle Flag"]
    #[inline(always)]
    pub fn idlnf(&self) -> IdlnfR {
        IdlnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Flag"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn perf(&self) -> PerfR {
        PerfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Underrun Error Flag (When MODF is 0, This bit is invalid.)"]
    #[inline(always)]
    pub fn udrf(&self) -> UdrfR {
        UdrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SptefR {
        SptefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SprfR {
        SprfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPSR")
            .field("idlnf", &self.idlnf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overrun Error Flag"]
    #[inline(always)]
    pub fn ovrf(&mut self) -> OvrfW<SpsrSpec> {
        OvrfW::new(self, 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Flag"]
    #[inline(always)]
    pub fn modf(&mut self) -> ModfW<SpsrSpec> {
        ModfW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn perf(&mut self) -> PerfW<SpsrSpec> {
        PerfW::new(self, 3)
    }
    #[doc = "Bit 4 - Underrun Error Flag (When MODF is 0, This bit is invalid.)"]
    #[inline(always)]
    pub fn udrf(&mut self) -> UdrfW<SpsrSpec> {
        UdrfW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(&mut self) -> SptefW<SpsrSpec> {
        SptefW::new(self, 5)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(&mut self) -> SprfW<SpsrSpec> {
        SprfW::new(self, 7)
    }
}
#[doc = "SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpsrSpec;
impl crate::RegisterSpec for SpsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spsr::R`](R) reader structure"]
impl crate::Readable for SpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`spsr::W`](W) writer structure"]
impl crate::Writable for SpsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xbd;
}
#[doc = "`reset()` method sets SPSR to value 0x20"]
impl crate::Resettable for SpsrSpec {
    const RESET_VALUE: u8 = 0x20;
}

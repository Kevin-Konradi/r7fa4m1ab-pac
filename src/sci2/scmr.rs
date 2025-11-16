#[doc = "Register `SCMR` reader"]
pub type R = crate::R<ScmrSpec>;
#[doc = "Register `SCMR` writer"]
pub type W = crate::W<ScmrSpec>;
#[doc = "Smart Card Interface Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smif {
    #[doc = "0: Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    _0 = 0,
    #[doc = "1: Smart card interface mode"]
    _1 = 1,
}
impl From<Smif> for bool {
    #[inline(always)]
    fn from(variant: Smif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIF` reader - Smart Card Interface Mode Select"]
pub type SmifR = crate::BitReader<Smif>;
impl SmifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smif {
        match self.bits {
            false => Smif::_0,
            true => Smif::_1,
        }
    }
    #[doc = "Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smif::_0
    }
    #[doc = "Smart card interface mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smif::_1
    }
}
#[doc = "Field `SMIF` writer - Smart Card Interface Mode Select"]
pub type SmifW<'a, REG> = crate::BitWriter<'a, REG, Smif>;
impl<'a, REG> SmifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smif::_0)
    }
    #[doc = "Smart card interface mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smif::_1)
    }
}
#[doc = "Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sinv {
    #[doc = "0: TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    _0 = 0,
    #[doc = "1: TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    _1 = 1,
}
impl From<Sinv> for bool {
    #[inline(always)]
    fn from(variant: Sinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINV` reader - Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
pub type SinvR = crate::BitReader<Sinv>;
impl SinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sinv {
        match self.bits {
            false => Sinv::_0,
            true => Sinv::_1,
        }
    }
    #[doc = "TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sinv::_0
    }
    #[doc = "TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sinv::_1
    }
}
#[doc = "Field `SINV` writer - Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
pub type SinvW<'a, REG> = crate::BitWriter<'a, REG, Sinv>;
impl<'a, REG> SinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_0)
    }
    #[doc = "TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_1)
    }
}
#[doc = "Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdir {
    #[doc = "0: Transfer with LSB first"]
    _0 = 0,
    #[doc = "1: Transfer with MSB first"]
    _1 = 1,
}
impl From<Sdir> for bool {
    #[inline(always)]
    fn from(variant: Sdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIR` reader - Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
pub type SdirR = crate::BitReader<Sdir>;
impl SdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdir {
        match self.bits {
            false => Sdir::_0,
            true => Sdir::_1,
        }
    }
    #[doc = "Transfer with LSB first"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdir::_0
    }
    #[doc = "Transfer with MSB first"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdir::_1
    }
}
#[doc = "Field `SDIR` writer - Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
pub type SdirW<'a, REG> = crate::BitWriter<'a, REG, Sdir>;
impl<'a, REG> SdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer with LSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::_0)
    }
    #[doc = "Transfer with MSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::_1)
    }
}
#[doc = "Character Length 1 (Only valid in asynchronous mode)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chr1 {
    #[doc = "0: Transmit/receive in 9-bit data length"]
    _0 = 0,
    #[doc = "1: Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    _1 = 1,
}
impl From<Chr1> for bool {
    #[inline(always)]
    fn from(variant: Chr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHR1` reader - Character Length 1 (Only valid in asynchronous mode)"]
pub type Chr1R = crate::BitReader<Chr1>;
impl Chr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chr1 {
        match self.bits {
            false => Chr1::_0,
            true => Chr1::_1,
        }
    }
    #[doc = "Transmit/receive in 9-bit data length"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chr1::_0
    }
    #[doc = "Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chr1::_1
    }
}
#[doc = "Field `CHR1` writer - Character Length 1 (Only valid in asynchronous mode)"]
pub type Chr1W<'a, REG> = crate::BitWriter<'a, REG, Chr1>;
impl<'a, REG> Chr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit/receive in 9-bit data length"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chr1::_0)
    }
    #[doc = "Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chr1::_1)
    }
}
#[doc = "Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcp2 {
    #[doc = "0: S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    _0 = 0,
    #[doc = "1: S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    _1 = 1,
}
impl From<Bcp2> for bool {
    #[inline(always)]
    fn from(variant: Bcp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCP2` reader - Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
pub type Bcp2R = crate::BitReader<Bcp2>;
impl Bcp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcp2 {
        match self.bits {
            false => Bcp2::_0,
            true => Bcp2::_1,
        }
    }
    #[doc = "S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bcp2::_0
    }
    #[doc = "S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bcp2::_1
    }
}
#[doc = "Field `BCP2` writer - Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
pub type Bcp2W<'a, REG> = crate::BitWriter<'a, REG, Bcp2>;
impl<'a, REG> Bcp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp2::_0)
    }
    #[doc = "S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp2::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Mode Select"]
    #[inline(always)]
    pub fn smif(&self) -> SmifR {
        SmifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sinv(&self) -> SinvR {
        SinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sdir(&self) -> SdirR {
        SdirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Character Length 1 (Only valid in asynchronous mode)"]
    #[inline(always)]
    pub fn chr1(&self) -> Chr1R {
        Chr1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
    #[inline(always)]
    pub fn bcp2(&self) -> Bcp2R {
        Bcp2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMR")
            .field("bcp2", &self.bcp2())
            .field("chr1", &self.chr1())
            .field("sdir", &self.sdir())
            .field("sinv", &self.sinv())
            .field("smif", &self.smif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Mode Select"]
    #[inline(always)]
    pub fn smif(&mut self) -> SmifW<ScmrSpec> {
        SmifW::new(self, 0)
    }
    #[doc = "Bit 2 - Transmitted/Received Data Invert Set this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sinv(&mut self) -> SinvW<ScmrSpec> {
        SinvW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitted/Received Data Transfer Direction NOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode. Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sdir(&mut self) -> SdirW<ScmrSpec> {
        SdirW::new(self, 3)
    }
    #[doc = "Bit 4 - Character Length 1 (Only valid in asynchronous mode)"]
    #[inline(always)]
    pub fn chr1(&mut self) -> Chr1W<ScmrSpec> {
        Chr1W::new(self, 4)
    }
    #[doc = "Bit 7 - Base Clock Pulse 2 Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
    #[inline(always)]
    pub fn bcp2(&mut self) -> Bcp2W<ScmrSpec> {
        Bcp2W::new(self, 7)
    }
}
#[doc = "Smart Card Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmrSpec;
impl crate::RegisterSpec for ScmrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scmr::R`](R) reader structure"]
impl crate::Readable for ScmrSpec {}
#[doc = "`write(|w| ..)` method takes [`scmr::W`](W) writer structure"]
impl crate::Writable for ScmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMR to value 0xf2"]
impl crate::Resettable for ScmrSpec {
    const RESET_VALUE: u8 = 0xf2;
}

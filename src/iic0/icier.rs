#[doc = "Register `ICIER` reader"]
pub type R = crate::R<IcierSpec>;
#[doc = "Register `ICIER` writer"]
pub type W = crate::W<IcierSpec>;
#[doc = "Timeout Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoie {
    #[doc = "0: Timeout interrupt request (TMOI) is disabled."]
    _0 = 0,
    #[doc = "1: Timeout interrupt request (TMOI) is enabled."]
    _1 = 1,
}
impl From<Tmoie> for bool {
    #[inline(always)]
    fn from(variant: Tmoie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOIE` reader - Timeout Interrupt Request Enable"]
pub type TmoieR = crate::BitReader<Tmoie>;
impl TmoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmoie {
        match self.bits {
            false => Tmoie::_0,
            true => Tmoie::_1,
        }
    }
    #[doc = "Timeout interrupt request (TMOI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmoie::_0
    }
    #[doc = "Timeout interrupt request (TMOI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmoie::_1
    }
}
#[doc = "Field `TMOIE` writer - Timeout Interrupt Request Enable"]
pub type TmoieW<'a, REG> = crate::BitWriter<'a, REG, Tmoie>;
impl<'a, REG> TmoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout interrupt request (TMOI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoie::_0)
    }
    #[doc = "Timeout interrupt request (TMOI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoie::_1)
    }
}
#[doc = "Arbitration-Lost Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alie {
    #[doc = "0: Arbitration-lost interrupt request (ALI) is disabled."]
    _0 = 0,
    #[doc = "1: Arbitration-lost interrupt request (ALI) is enabled."]
    _1 = 1,
}
impl From<Alie> for bool {
    #[inline(always)]
    fn from(variant: Alie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIE` reader - Arbitration-Lost Interrupt Request Enable"]
pub type AlieR = crate::BitReader<Alie>;
impl AlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alie {
        match self.bits {
            false => Alie::_0,
            true => Alie::_1,
        }
    }
    #[doc = "Arbitration-lost interrupt request (ALI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alie::_0
    }
    #[doc = "Arbitration-lost interrupt request (ALI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alie::_1
    }
}
#[doc = "Field `ALIE` writer - Arbitration-Lost Interrupt Request Enable"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG, Alie>;
impl<'a, REG> AlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Arbitration-lost interrupt request (ALI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_0)
    }
    #[doc = "Arbitration-lost interrupt request (ALI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_1)
    }
}
#[doc = "Start Condition Detection Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stie {
    #[doc = "0: Start condition detection interrupt request (STI) is disabled."]
    _0 = 0,
    #[doc = "1: Start condition detection interrupt request (STI) is enabled."]
    _1 = 1,
}
impl From<Stie> for bool {
    #[inline(always)]
    fn from(variant: Stie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIE` reader - Start Condition Detection Interrupt Request Enable"]
pub type StieR = crate::BitReader<Stie>;
impl StieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stie {
        match self.bits {
            false => Stie::_0,
            true => Stie::_1,
        }
    }
    #[doc = "Start condition detection interrupt request (STI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stie::_0
    }
    #[doc = "Start condition detection interrupt request (STI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stie::_1
    }
}
#[doc = "Field `STIE` writer - Start Condition Detection Interrupt Request Enable"]
pub type StieW<'a, REG> = crate::BitWriter<'a, REG, Stie>;
impl<'a, REG> StieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start condition detection interrupt request (STI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stie::_0)
    }
    #[doc = "Start condition detection interrupt request (STI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stie::_1)
    }
}
#[doc = "Stop Condition Detection Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spie {
    #[doc = "0: Stop condition detection interrupt request (SPI) is disabled."]
    _0 = 0,
    #[doc = "1: Stop condition detection interrupt request (SPI) is enabled."]
    _1 = 1,
}
impl From<Spie> for bool {
    #[inline(always)]
    fn from(variant: Spie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - Stop Condition Detection Interrupt Request Enable"]
pub type SpieR = crate::BitReader<Spie>;
impl SpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spie {
        match self.bits {
            false => Spie::_0,
            true => Spie::_1,
        }
    }
    #[doc = "Stop condition detection interrupt request (SPI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spie::_0
    }
    #[doc = "Stop condition detection interrupt request (SPI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spie::_1
    }
}
#[doc = "Field `SPIE` writer - Stop Condition Detection Interrupt Request Enable"]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, Spie>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop condition detection interrupt request (SPI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_0)
    }
    #[doc = "Stop condition detection interrupt request (SPI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_1)
    }
}
#[doc = "NACK Reception Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nakie {
    #[doc = "0: NACK reception interrupt request (NAKI) is disabled."]
    _0 = 0,
    #[doc = "1: NACK reception interrupt request (NAKI) is enabled."]
    _1 = 1,
}
impl From<Nakie> for bool {
    #[inline(always)]
    fn from(variant: Nakie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAKIE` reader - NACK Reception Interrupt Request Enable"]
pub type NakieR = crate::BitReader<Nakie>;
impl NakieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nakie {
        match self.bits {
            false => Nakie::_0,
            true => Nakie::_1,
        }
    }
    #[doc = "NACK reception interrupt request (NAKI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nakie::_0
    }
    #[doc = "NACK reception interrupt request (NAKI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nakie::_1
    }
}
#[doc = "Field `NAKIE` writer - NACK Reception Interrupt Request Enable"]
pub type NakieW<'a, REG> = crate::BitWriter<'a, REG, Nakie>;
impl<'a, REG> NakieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK reception interrupt request (NAKI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nakie::_0)
    }
    #[doc = "NACK reception interrupt request (NAKI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nakie::_1)
    }
}
#[doc = "Receive Data Full Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: Receive data full interrupt request (IIC_RXI) is disabled."]
    _0 = 0,
    #[doc = "1: Receive data full interrupt request (IIC_RXI) is enabled."]
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receive Data Full Interrupt Request Enable"]
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
    #[doc = "Receive data full interrupt request (IIC_RXI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    #[doc = "Receive data full interrupt request (IIC_RXI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
#[doc = "Field `RIE` writer - Receive Data Full Interrupt Request Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data full interrupt request (IIC_RXI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    #[doc = "Receive data full interrupt request (IIC_RXI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
#[doc = "Transmit End Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teie {
    #[doc = "0: Transmit end interrupt request (IIC_TEI) is disabled."]
    _0 = 0,
    #[doc = "1: Transmit end interrupt request (IIC_TEI) is enabled."]
    _1 = 1,
}
impl From<Teie> for bool {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Request Enable"]
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
    #[doc = "Transmit end interrupt request (IIC_TEI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teie::_0
    }
    #[doc = "Transmit end interrupt request (IIC_TEI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teie::_1
    }
}
#[doc = "Field `TEIE` writer - Transmit End Interrupt Request Enable"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit end interrupt request (IIC_TEI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_0)
    }
    #[doc = "Transmit end interrupt request (IIC_TEI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_1)
    }
}
#[doc = "Transmit Data Empty Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Transmit data empty interrupt request (IIC_TXI) is disabled."]
    _0 = 0,
    #[doc = "1: Transmit data empty interrupt request (IIC_TXI) is enabled."]
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmit Data Empty Interrupt Request Enable"]
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
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Data Empty Interrupt Request Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    #[doc = "Transmit data empty interrupt request (IIC_TXI) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Interrupt Request Enable"]
    #[inline(always)]
    pub fn tmoie(&self) -> TmoieR {
        TmoieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn stie(&self) -> StieR {
        StieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    pub fn nakie(&self) -> NakieR {
        NakieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit End Interrupt Request Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICIER")
            .field("tie", &self.tie())
            .field("teie", &self.teie())
            .field("rie", &self.rie())
            .field("nakie", &self.nakie())
            .field("spie", &self.spie())
            .field("stie", &self.stie())
            .field("alie", &self.alie())
            .field("tmoie", &self.tmoie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Interrupt Request Enable"]
    #[inline(always)]
    pub fn tmoie(&mut self) -> TmoieW<IcierSpec> {
        TmoieW::new(self, 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<IcierSpec> {
        AlieW::new(self, 1)
    }
    #[doc = "Bit 2 - Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> StieW<IcierSpec> {
        StieW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn spie(&mut self) -> SpieW<IcierSpec> {
        SpieW::new(self, 3)
    }
    #[doc = "Bit 4 - NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    pub fn nakie(&mut self) -> NakieW<IcierSpec> {
        NakieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<IcierSpec> {
        RieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit End Interrupt Request Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<IcierSpec> {
        TeieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<IcierSpec> {
        TieW::new(self, 7)
    }
}
#[doc = "I2C Bus Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcierSpec;
impl crate::RegisterSpec for IcierSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icier::R`](R) reader structure"]
impl crate::Readable for IcierSpec {}
#[doc = "`write(|w| ..)` method takes [`icier::W`](W) writer structure"]
impl crate::Writable for IcierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICIER to value 0"]
impl crate::Resettable for IcierSpec {}

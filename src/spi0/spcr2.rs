#[doc = "Register `SPCR2` reader"]
pub type R = crate::R<Spcr2Spec>;
#[doc = "Register `SPCR2` writer"]
pub type W = crate::W<Spcr2Spec>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sppe {
    #[doc = "0: Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    _0 = 0,
    #[doc = "1: Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    _1 = 1,
}
impl From<Sppe> for bool {
    #[inline(always)]
    fn from(variant: Sppe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPPE` reader - Parity Enable"]
pub type SppeR = crate::BitReader<Sppe>;
impl SppeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sppe {
        match self.bits {
            false => Sppe::_0,
            true => Sppe::_1,
        }
    }
    #[doc = "Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sppe::_0
    }
    #[doc = "Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sppe::_1
    }
}
#[doc = "Field `SPPE` writer - Parity Enable"]
pub type SppeW<'a, REG> = crate::BitWriter<'a, REG, Sppe>;
impl<'a, REG> SppeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sppe::_0)
    }
    #[doc = "Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sppe::_1)
    }
}
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spoe {
    #[doc = "0: Selects even parity for use in transmission and reception"]
    _0 = 0,
    #[doc = "1: Selects odd parity for use in transmission and reception"]
    _1 = 1,
}
impl From<Spoe> for bool {
    #[inline(always)]
    fn from(variant: Spoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOE` reader - Parity Mode"]
pub type SpoeR = crate::BitReader<Spoe>;
impl SpoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spoe {
        match self.bits {
            false => Spoe::_0,
            true => Spoe::_1,
        }
    }
    #[doc = "Selects even parity for use in transmission and reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spoe::_0
    }
    #[doc = "Selects odd parity for use in transmission and reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spoe::_1
    }
}
#[doc = "Field `SPOE` writer - Parity Mode"]
pub type SpoeW<'a, REG> = crate::BitWriter<'a, REG, Spoe>;
impl<'a, REG> SpoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects even parity for use in transmission and reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spoe::_0)
    }
    #[doc = "Selects odd parity for use in transmission and reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spoe::_1)
    }
}
#[doc = "SPI Idle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiie {
    #[doc = "0: Disables the generation of idle interrupt requests"]
    _0 = 0,
    #[doc = "1: Enables the generation of idle interrupt requests"]
    _1 = 1,
}
impl From<Spiie> for bool {
    #[inline(always)]
    fn from(variant: Spiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIIE` reader - SPI Idle Interrupt Enable"]
pub type SpiieR = crate::BitReader<Spiie>;
impl SpiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiie {
        match self.bits {
            false => Spiie::_0,
            true => Spiie::_1,
        }
    }
    #[doc = "Disables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spiie::_0
    }
    #[doc = "Enables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spiie::_1
    }
}
#[doc = "Field `SPIIE` writer - SPI Idle Interrupt Enable"]
pub type SpiieW<'a, REG> = crate::BitWriter<'a, REG, Spiie>;
impl<'a, REG> SpiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spiie::_0)
    }
    #[doc = "Enables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spiie::_1)
    }
}
#[doc = "Parity Self-Testing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pte {
    #[doc = "0: Disables the self-diagnosis function of the parity circuit"]
    _0 = 0,
    #[doc = "1: Enables the self-diagnosis function of the parity circuit"]
    _1 = 1,
}
impl From<Pte> for bool {
    #[inline(always)]
    fn from(variant: Pte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTE` reader - Parity Self-Testing"]
pub type PteR = crate::BitReader<Pte>;
impl PteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pte {
        match self.bits {
            false => Pte::_0,
            true => Pte::_1,
        }
    }
    #[doc = "Disables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pte::_0
    }
    #[doc = "Enables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pte::_1
    }
}
#[doc = "Field `PTE` writer - Parity Self-Testing"]
pub type PteW<'a, REG> = crate::BitWriter<'a, REG, Pte>;
impl<'a, REG> PteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pte::_0)
    }
    #[doc = "Enables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pte::_1)
    }
}
#[doc = "RSPCK Auto-Stop Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckase {
    #[doc = "0: Disables the RSPCK auto-stop function"]
    _0 = 0,
    #[doc = "1: Enables the RSPCK auto-stop function"]
    _1 = 1,
}
impl From<Sckase> for bool {
    #[inline(always)]
    fn from(variant: Sckase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKASE` reader - RSPCK Auto-Stop Function Enable"]
pub type SckaseR = crate::BitReader<Sckase>;
impl SckaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckase {
        match self.bits {
            false => Sckase::_0,
            true => Sckase::_1,
        }
    }
    #[doc = "Disables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sckase::_0
    }
    #[doc = "Enables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sckase::_1
    }
}
#[doc = "Field `SCKASE` writer - RSPCK Auto-Stop Function Enable"]
pub type SckaseW<'a, REG> = crate::BitWriter<'a, REG, Sckase>;
impl<'a, REG> SckaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sckase::_0)
    }
    #[doc = "Enables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sckase::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&self) -> SppeR {
        SppeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&self) -> SpoeR {
        SpoeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&self) -> SpiieR {
        SpiieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(&self) -> PteR {
        PteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&self) -> SckaseR {
        SckaseR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCR2")
            .field("sckase", &self.sckase())
            .field("pte", &self.pte())
            .field("spiie", &self.spiie())
            .field("spoe", &self.spoe())
            .field("sppe", &self.sppe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&mut self) -> SppeW<Spcr2Spec> {
        SppeW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&mut self) -> SpoeW<Spcr2Spec> {
        SpoeW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&mut self) -> SpiieW<Spcr2Spec> {
        SpiieW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(&mut self) -> PteW<Spcr2Spec> {
        PteW::new(self, 3)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&mut self) -> SckaseW<Spcr2Spec> {
        SckaseW::new(self, 4)
    }
}
#[doc = "SPI Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spcr2Spec;
impl crate::RegisterSpec for Spcr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spcr2::R`](R) reader structure"]
impl crate::Readable for Spcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`spcr2::W`](W) writer structure"]
impl crate::Writable for Spcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPCR2 to value 0"]
impl crate::Resettable for Spcr2Spec {}

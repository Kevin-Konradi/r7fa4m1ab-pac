#[doc = "Register `EIER` reader"]
pub type R = crate::R<EierSpec>;
#[doc = "Register `EIER` writer"]
pub type W = crate::W<EierSpec>;
#[doc = "Bus Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beie {
    #[doc = "0: Bus error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus error interrupt enabled"]
    _1 = 1,
}
impl From<Beie> for bool {
    #[inline(always)]
    fn from(variant: Beie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable"]
pub type BeieR = crate::BitReader<Beie>;
impl BeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beie {
        match self.bits {
            false => Beie::_0,
            true => Beie::_1,
        }
    }
    #[doc = "Bus error interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Beie::_0
    }
    #[doc = "Bus error interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Beie::_1
    }
}
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable"]
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG, Beie>;
impl<'a, REG> BeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Beie::_0)
    }
    #[doc = "Bus error interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Beie::_1)
    }
}
#[doc = "Error-Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewie {
    #[doc = "0: Error-warning interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error-warning interrupt enabled"]
    _1 = 1,
}
impl From<Ewie> for bool {
    #[inline(always)]
    fn from(variant: Ewie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - Error-Warning Interrupt Enable"]
pub type EwieR = crate::BitReader<Ewie>;
impl EwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewie {
        match self.bits {
            false => Ewie::_0,
            true => Ewie::_1,
        }
    }
    #[doc = "Error-warning interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewie::_0
    }
    #[doc = "Error-warning interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewie::_1
    }
}
#[doc = "Field `EWIE` writer - Error-Warning Interrupt Enable"]
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG, Ewie>;
impl<'a, REG> EwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error-warning interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::_0)
    }
    #[doc = "Error-warning interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::_1)
    }
}
#[doc = "Error-Passive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epie {
    #[doc = "0: Error-passive interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error-passive interrupt enabled"]
    _1 = 1,
}
impl From<Epie> for bool {
    #[inline(always)]
    fn from(variant: Epie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIE` reader - Error-Passive Interrupt Enable"]
pub type EpieR = crate::BitReader<Epie>;
impl EpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epie {
        match self.bits {
            false => Epie::_0,
            true => Epie::_1,
        }
    }
    #[doc = "Error-passive interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epie::_0
    }
    #[doc = "Error-passive interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epie::_1
    }
}
#[doc = "Field `EPIE` writer - Error-Passive Interrupt Enable"]
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG, Epie>;
impl<'a, REG> EpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error-passive interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Epie::_0)
    }
    #[doc = "Error-passive interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Epie::_1)
    }
}
#[doc = "Bus-Off Entry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boeie {
    #[doc = "0: Bus-off entry interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off entry interrupt enabled"]
    _1 = 1,
}
impl From<Boeie> for bool {
    #[inline(always)]
    fn from(variant: Boeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOEIE` reader - Bus-Off Entry Interrupt Enable"]
pub type BoeieR = crate::BitReader<Boeie>;
impl BoeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boeie {
        match self.bits {
            false => Boeie::_0,
            true => Boeie::_1,
        }
    }
    #[doc = "Bus-off entry interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boeie::_0
    }
    #[doc = "Bus-off entry interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boeie::_1
    }
}
#[doc = "Field `BOEIE` writer - Bus-Off Entry Interrupt Enable"]
pub type BoeieW<'a, REG> = crate::BitWriter<'a, REG, Boeie>;
impl<'a, REG> BoeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus-off entry interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boeie::_0)
    }
    #[doc = "Bus-off entry interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boeie::_1)
    }
}
#[doc = "Bus-Off Recovery Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borie {
    #[doc = "0: Bus-off recovery interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus-off recovery interrupt enabled"]
    _1 = 1,
}
impl From<Borie> for bool {
    #[inline(always)]
    fn from(variant: Borie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORIE` reader - Bus-Off Recovery Interrupt Enable"]
pub type BorieR = crate::BitReader<Borie>;
impl BorieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borie {
        match self.bits {
            false => Borie::_0,
            true => Borie::_1,
        }
    }
    #[doc = "Bus-off recovery interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Borie::_0
    }
    #[doc = "Bus-off recovery interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Borie::_1
    }
}
#[doc = "Field `BORIE` writer - Bus-Off Recovery Interrupt Enable"]
pub type BorieW<'a, REG> = crate::BitWriter<'a, REG, Borie>;
impl<'a, REG> BorieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus-off recovery interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Borie::_0)
    }
    #[doc = "Bus-off recovery interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Borie::_1)
    }
}
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orie {
    #[doc = "0: Receive overrun interrupt disabled"]
    _0 = 0,
    #[doc = "1: Receive overrun interrupt enabled"]
    _1 = 1,
}
impl From<Orie> for bool {
    #[inline(always)]
    fn from(variant: Orie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIE` reader - Overrun Interrupt Enable"]
pub type OrieR = crate::BitReader<Orie>;
impl OrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orie {
        match self.bits {
            false => Orie::_0,
            true => Orie::_1,
        }
    }
    #[doc = "Receive overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orie::_0
    }
    #[doc = "Receive overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orie::_1
    }
}
#[doc = "Field `ORIE` writer - Overrun Interrupt Enable"]
pub type OrieW<'a, REG> = crate::BitWriter<'a, REG, Orie>;
impl<'a, REG> OrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive overrun interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::_0)
    }
    #[doc = "Receive overrun interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::_1)
    }
}
#[doc = "Overload Frame Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Olie {
    #[doc = "0: Overload frame transmit interrupt disabled"]
    _0 = 0,
    #[doc = "1: Overload frame transmit interrupt enabled"]
    _1 = 1,
}
impl From<Olie> for bool {
    #[inline(always)]
    fn from(variant: Olie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OLIE` reader - Overload Frame Transmit Interrupt Enable"]
pub type OlieR = crate::BitReader<Olie>;
impl OlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Olie {
        match self.bits {
            false => Olie::_0,
            true => Olie::_1,
        }
    }
    #[doc = "Overload frame transmit interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Olie::_0
    }
    #[doc = "Overload frame transmit interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Olie::_1
    }
}
#[doc = "Field `OLIE` writer - Overload Frame Transmit Interrupt Enable"]
pub type OlieW<'a, REG> = crate::BitWriter<'a, REG, Olie>;
impl<'a, REG> OlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overload frame transmit interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Olie::_0)
    }
    #[doc = "Overload frame transmit interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Olie::_1)
    }
}
#[doc = "Bus Lock Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blie {
    #[doc = "0: Bus lock interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus lock interrupt enabled"]
    _1 = 1,
}
impl From<Blie> for bool {
    #[inline(always)]
    fn from(variant: Blie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLIE` reader - Bus Lock Interrupt Enable"]
pub type BlieR = crate::BitReader<Blie>;
impl BlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blie {
        match self.bits {
            false => Blie::_0,
            true => Blie::_1,
        }
    }
    #[doc = "Bus lock interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blie::_0
    }
    #[doc = "Bus lock interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blie::_1
    }
}
#[doc = "Field `BLIE` writer - Bus Lock Interrupt Enable"]
pub type BlieW<'a, REG> = crate::BitWriter<'a, REG, Blie>;
impl<'a, REG> BlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus lock interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blie::_0)
    }
    #[doc = "Bus lock interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blie::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(&self) -> BoeieR {
        BoeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(&self) -> BorieR {
        BorieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> OrieR {
        OrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn olie(&self) -> OlieR {
        OlieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(&self) -> BlieR {
        BlieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIER")
            .field("blie", &self.blie())
            .field("olie", &self.olie())
            .field("orie", &self.orie())
            .field("borie", &self.borie())
            .field("boeie", &self.boeie())
            .field("epie", &self.epie())
            .field("ewie", &self.ewie())
            .field("beie", &self.beie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<EierSpec> {
        BeieW::new(self, 0)
    }
    #[doc = "Bit 1 - Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(&mut self) -> EwieW<EierSpec> {
        EwieW::new(self, 1)
    }
    #[doc = "Bit 2 - Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<EierSpec> {
        EpieW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(&mut self) -> BoeieW<EierSpec> {
        BoeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(&mut self) -> BorieW<EierSpec> {
        BorieW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&mut self) -> OrieW<EierSpec> {
        OrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn olie(&mut self) -> OlieW<EierSpec> {
        OlieW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(&mut self) -> BlieW<EierSpec> {
        BlieW::new(self, 7)
    }
}
#[doc = "Error Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EierSpec;
impl crate::RegisterSpec for EierSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eier::R`](R) reader structure"]
impl crate::Readable for EierSpec {}
#[doc = "`write(|w| ..)` method takes [`eier::W`](W) writer structure"]
impl crate::Writable for EierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIER to value 0"]
impl crate::Resettable for EierSpec {}

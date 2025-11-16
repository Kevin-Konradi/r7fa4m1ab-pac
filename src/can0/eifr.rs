#[doc = "Register `EIFR` reader"]
pub type R = crate::R<EifrSpec>;
#[doc = "Register `EIFR` writer"]
pub type W = crate::W<EifrSpec>;
#[doc = "Bus Error Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beif {
    #[doc = "0: No bus error detected"]
    _0 = 0,
    #[doc = "1: Bus error detected"]
    _1 = 1,
}
impl From<Beif> for bool {
    #[inline(always)]
    fn from(variant: Beif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIF` reader - Bus Error Detect Flag"]
pub type BeifR = crate::BitReader<Beif>;
impl BeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beif {
        match self.bits {
            false => Beif::_0,
            true => Beif::_1,
        }
    }
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Beif::_0
    }
    #[doc = "Bus error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Beif::_1
    }
}
#[doc = "Field `BEIF` writer - Bus Error Detect Flag"]
pub type BeifW<'a, REG> = crate::BitWriter<'a, REG, Beif>;
impl<'a, REG> BeifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Beif::_0)
    }
    #[doc = "Bus error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Beif::_1)
    }
}
#[doc = "Error-Warning Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewif {
    #[doc = "0: No error-warning detected"]
    _0 = 0,
    #[doc = "1: Error-warning detected"]
    _1 = 1,
}
impl From<Ewif> for bool {
    #[inline(always)]
    fn from(variant: Ewif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` reader - Error-Warning Detect Flag"]
pub type EwifR = crate::BitReader<Ewif>;
impl EwifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewif {
        match self.bits {
            false => Ewif::_0,
            true => Ewif::_1,
        }
    }
    #[doc = "No error-warning detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewif::_0
    }
    #[doc = "Error-warning detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewif::_1
    }
}
#[doc = "Field `EWIF` writer - Error-Warning Detect Flag"]
pub type EwifW<'a, REG> = crate::BitWriter<'a, REG, Ewif>;
impl<'a, REG> EwifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error-warning detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewif::_0)
    }
    #[doc = "Error-warning detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewif::_1)
    }
}
#[doc = "Error-Passive Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epif {
    #[doc = "0: No error-passive detected"]
    _0 = 0,
    #[doc = "1: Error-passive detected"]
    _1 = 1,
}
impl From<Epif> for bool {
    #[inline(always)]
    fn from(variant: Epif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIF` reader - Error-Passive Detect Flag"]
pub type EpifR = crate::BitReader<Epif>;
impl EpifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epif {
        match self.bits {
            false => Epif::_0,
            true => Epif::_1,
        }
    }
    #[doc = "No error-passive detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epif::_0
    }
    #[doc = "Error-passive detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epif::_1
    }
}
#[doc = "Field `EPIF` writer - Error-Passive Detect Flag"]
pub type EpifW<'a, REG> = crate::BitWriter<'a, REG, Epif>;
impl<'a, REG> EpifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error-passive detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Epif::_0)
    }
    #[doc = "Error-passive detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Epif::_1)
    }
}
#[doc = "Bus-Off Entry Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boeif {
    #[doc = "0: No bus-off entry detected"]
    _0 = 0,
    #[doc = "1: Bus-off entry detected"]
    _1 = 1,
}
impl From<Boeif> for bool {
    #[inline(always)]
    fn from(variant: Boeif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOEIF` reader - Bus-Off Entry Detect Flag"]
pub type BoeifR = crate::BitReader<Boeif>;
impl BoeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boeif {
        match self.bits {
            false => Boeif::_0,
            true => Boeif::_1,
        }
    }
    #[doc = "No bus-off entry detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boeif::_0
    }
    #[doc = "Bus-off entry detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boeif::_1
    }
}
#[doc = "Field `BOEIF` writer - Bus-Off Entry Detect Flag"]
pub type BoeifW<'a, REG> = crate::BitWriter<'a, REG, Boeif>;
impl<'a, REG> BoeifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus-off entry detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boeif::_0)
    }
    #[doc = "Bus-off entry detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boeif::_1)
    }
}
#[doc = "Bus-Off Recovery Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borif {
    #[doc = "0: No bus-off recovery detected"]
    _0 = 0,
    #[doc = "1: Bus-off recovery detected"]
    _1 = 1,
}
impl From<Borif> for bool {
    #[inline(always)]
    fn from(variant: Borif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORIF` reader - Bus-Off Recovery Detect Flag"]
pub type BorifR = crate::BitReader<Borif>;
impl BorifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borif {
        match self.bits {
            false => Borif::_0,
            true => Borif::_1,
        }
    }
    #[doc = "No bus-off recovery detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Borif::_0
    }
    #[doc = "Bus-off recovery detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Borif::_1
    }
}
#[doc = "Field `BORIF` writer - Bus-Off Recovery Detect Flag"]
pub type BorifW<'a, REG> = crate::BitWriter<'a, REG, Borif>;
impl<'a, REG> BorifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus-off recovery detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Borif::_0)
    }
    #[doc = "Bus-off recovery detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Borif::_1)
    }
}
#[doc = "Receive Overrun Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orif {
    #[doc = "0: No receive overrun detected"]
    _0 = 0,
    #[doc = "1: Receive overrun detected"]
    _1 = 1,
}
impl From<Orif> for bool {
    #[inline(always)]
    fn from(variant: Orif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIF` reader - Receive Overrun Detect Flag"]
pub type OrifR = crate::BitReader<Orif>;
impl OrifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orif {
        match self.bits {
            false => Orif::_0,
            true => Orif::_1,
        }
    }
    #[doc = "No receive overrun detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orif::_0
    }
    #[doc = "Receive overrun detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orif::_1
    }
}
#[doc = "Field `ORIF` writer - Receive Overrun Detect Flag"]
pub type OrifW<'a, REG> = crate::BitWriter<'a, REG, Orif>;
impl<'a, REG> OrifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive overrun detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Orif::_0)
    }
    #[doc = "Receive overrun detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orif::_1)
    }
}
#[doc = "Overload Frame Transmission Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Olif {
    #[doc = "0: No overload frame transmission detected"]
    _0 = 0,
    #[doc = "1: Overload frame transmission detected"]
    _1 = 1,
}
impl From<Olif> for bool {
    #[inline(always)]
    fn from(variant: Olif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OLIF` reader - Overload Frame Transmission Detect Flag"]
pub type OlifR = crate::BitReader<Olif>;
impl OlifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Olif {
        match self.bits {
            false => Olif::_0,
            true => Olif::_1,
        }
    }
    #[doc = "No overload frame transmission detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Olif::_0
    }
    #[doc = "Overload frame transmission detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Olif::_1
    }
}
#[doc = "Field `OLIF` writer - Overload Frame Transmission Detect Flag"]
pub type OlifW<'a, REG> = crate::BitWriter<'a, REG, Olif>;
impl<'a, REG> OlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overload frame transmission detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Olif::_0)
    }
    #[doc = "Overload frame transmission detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Olif::_1)
    }
}
#[doc = "Bus Lock Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blif {
    #[doc = "0: No bus lock detected"]
    _0 = 0,
    #[doc = "1: Bus lock detected"]
    _1 = 1,
}
impl From<Blif> for bool {
    #[inline(always)]
    fn from(variant: Blif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLIF` reader - Bus Lock Detect Flag"]
pub type BlifR = crate::BitReader<Blif>;
impl BlifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blif {
        match self.bits {
            false => Blif::_0,
            true => Blif::_1,
        }
    }
    #[doc = "No bus lock detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blif::_0
    }
    #[doc = "Bus lock detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blif::_1
    }
}
#[doc = "Field `BLIF` writer - Bus Lock Detect Flag"]
pub type BlifW<'a, REG> = crate::BitWriter<'a, REG, Blif>;
impl<'a, REG> BlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus lock detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blif::_0)
    }
    #[doc = "Bus lock detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blif::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bus Error Detect Flag"]
    #[inline(always)]
    pub fn beif(&self) -> BeifR {
        BeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error-Warning Detect Flag"]
    #[inline(always)]
    pub fn ewif(&self) -> EwifR {
        EwifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error-Passive Detect Flag"]
    #[inline(always)]
    pub fn epif(&self) -> EpifR {
        EpifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub fn boeif(&self) -> BoeifR {
        BoeifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub fn borif(&self) -> BorifR {
        BorifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun Detect Flag"]
    #[inline(always)]
    pub fn orif(&self) -> OrifR {
        OrifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub fn olif(&self) -> OlifR {
        OlifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Lock Detect Flag"]
    #[inline(always)]
    pub fn blif(&self) -> BlifR {
        BlifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIFR")
            .field("blif", &self.blif())
            .field("olif", &self.olif())
            .field("orif", &self.orif())
            .field("borif", &self.borif())
            .field("boeif", &self.boeif())
            .field("epif", &self.epif())
            .field("ewif", &self.ewif())
            .field("beif", &self.beif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Detect Flag"]
    #[inline(always)]
    pub fn beif(&mut self) -> BeifW<EifrSpec> {
        BeifW::new(self, 0)
    }
    #[doc = "Bit 1 - Error-Warning Detect Flag"]
    #[inline(always)]
    pub fn ewif(&mut self) -> EwifW<EifrSpec> {
        EwifW::new(self, 1)
    }
    #[doc = "Bit 2 - Error-Passive Detect Flag"]
    #[inline(always)]
    pub fn epif(&mut self) -> EpifW<EifrSpec> {
        EpifW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub fn boeif(&mut self) -> BoeifW<EifrSpec> {
        BoeifW::new(self, 3)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub fn borif(&mut self) -> BorifW<EifrSpec> {
        BorifW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun Detect Flag"]
    #[inline(always)]
    pub fn orif(&mut self) -> OrifW<EifrSpec> {
        OrifW::new(self, 5)
    }
    #[doc = "Bit 6 - Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub fn olif(&mut self) -> OlifW<EifrSpec> {
        OlifW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus Lock Detect Flag"]
    #[inline(always)]
    pub fn blif(&mut self) -> BlifW<EifrSpec> {
        BlifW::new(self, 7)
    }
}
#[doc = "Error Interrupt Factor Judge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EifrSpec;
impl crate::RegisterSpec for EifrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eifr::R`](R) reader structure"]
impl crate::Readable for EifrSpec {}
#[doc = "`write(|w| ..)` method takes [`eifr::W`](W) writer structure"]
impl crate::Writable for EifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EifrSpec {}

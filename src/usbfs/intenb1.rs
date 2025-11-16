#[doc = "Register `INTENB1` reader"]
pub type R = crate::R<Intenb1Spec>;
#[doc = "Register `INTENB1` writer"]
pub type W = crate::W<Intenb1Spec>;
#[doc = "PDDETINT0 Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetinte0 {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Pddetinte0> for bool {
    #[inline(always)]
    fn from(variant: Pddetinte0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDETINTE0` reader - PDDETINT0 Detection Interrupt Enable"]
pub type Pddetinte0R = crate::BitReader<Pddetinte0>;
impl Pddetinte0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pddetinte0 {
        match self.bits {
            false => Pddetinte0::_0,
            true => Pddetinte0::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetinte0::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetinte0::_1
    }
}
#[doc = "Field `PDDETINTE0` writer - PDDETINT0 Detection Interrupt Enable"]
pub type Pddetinte0W<'a, REG> = crate::BitWriter<'a, REG, Pddetinte0>;
impl<'a, REG> Pddetinte0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetinte0::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetinte0::_1)
    }
}
#[doc = "Setup Transaction Normal Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sacke {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Sacke> for bool {
    #[inline(always)]
    fn from(variant: Sacke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKE` reader - Setup Transaction Normal Response Interrupt Enable"]
pub type SackeR = crate::BitReader<Sacke>;
impl SackeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sacke {
        match self.bits {
            false => Sacke::_0,
            true => Sacke::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sacke::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sacke::_1
    }
}
#[doc = "Field `SACKE` writer - Setup Transaction Normal Response Interrupt Enable"]
pub type SackeW<'a, REG> = crate::BitWriter<'a, REG, Sacke>;
impl<'a, REG> SackeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sacke::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sacke::_1)
    }
}
#[doc = "Setup Transaction Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signe {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Signe> for bool {
    #[inline(always)]
    fn from(variant: Signe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNE` reader - Setup Transaction Error Interrupt Enable"]
pub type SigneR = crate::BitReader<Signe>;
impl SigneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Signe {
        match self.bits {
            false => Signe::_0,
            true => Signe::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Signe::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Signe::_1
    }
}
#[doc = "Field `SIGNE` writer - Setup Transaction Error Interrupt Enable"]
pub type SigneW<'a, REG> = crate::BitWriter<'a, REG, Signe>;
impl<'a, REG> SigneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Signe::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Signe::_1)
    }
}
#[doc = "EOF Error Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoferre {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Eoferre> for bool {
    #[inline(always)]
    fn from(variant: Eoferre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOFERRE` reader - EOF Error Detection Interrupt Enable"]
pub type EoferreR = crate::BitReader<Eoferre>;
impl EoferreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoferre {
        match self.bits {
            false => Eoferre::_0,
            true => Eoferre::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eoferre::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eoferre::_1
    }
}
#[doc = "Field `EOFERRE` writer - EOF Error Detection Interrupt Enable"]
pub type EoferreW<'a, REG> = crate::BitWriter<'a, REG, Eoferre>;
impl<'a, REG> EoferreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferre::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferre::_1)
    }
}
#[doc = "Connection Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attche {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Attche> for bool {
    #[inline(always)]
    fn from(variant: Attche) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATTCHE` reader - Connection Detection Interrupt Enable"]
pub type AttcheR = crate::BitReader<Attche>;
impl AttcheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Attche {
        match self.bits {
            false => Attche::_0,
            true => Attche::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Attche::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Attche::_1
    }
}
#[doc = "Field `ATTCHE` writer - Connection Detection Interrupt Enable"]
pub type AttcheW<'a, REG> = crate::BitWriter<'a, REG, Attche>;
impl<'a, REG> AttcheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Attche::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Attche::_1)
    }
}
#[doc = "Disconnection Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtche {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Dtche> for bool {
    #[inline(always)]
    fn from(variant: Dtche) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCHE` reader - Disconnection Detection Interrupt Enable"]
pub type DtcheR = crate::BitReader<Dtche>;
impl DtcheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtche {
        match self.bits {
            false => Dtche::_0,
            true => Dtche::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtche::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtche::_1
    }
}
#[doc = "Field `DTCHE` writer - Disconnection Detection Interrupt Enable"]
pub type DtcheW<'a, REG> = crate::BitWriter<'a, REG, Dtche>;
impl<'a, REG> DtcheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtche::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtche::_1)
    }
}
#[doc = "USB Bus Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bchge {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Bchge> for bool {
    #[inline(always)]
    fn from(variant: Bchge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCHGE` reader - USB Bus Change Interrupt Enable"]
pub type BchgeR = crate::BitReader<Bchge>;
impl BchgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bchge {
        match self.bits {
            false => Bchge::_0,
            true => Bchge::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bchge::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bchge::_1
    }
}
#[doc = "Field `BCHGE` writer - USB Bus Change Interrupt Enable"]
pub type BchgeW<'a, REG> = crate::BitWriter<'a, REG, Bchge>;
impl<'a, REG> BchgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bchge::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bchge::_1)
    }
}
#[doc = "Overcurrent Input Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrcre {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Ovrcre> for bool {
    #[inline(always)]
    fn from(variant: Ovrcre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCRE` reader - Overcurrent Input Change Interrupt Enable"]
pub type OvrcreR = crate::BitReader<Ovrcre>;
impl OvrcreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrcre {
        match self.bits {
            false => Ovrcre::_0,
            true => Ovrcre::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrcre::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrcre::_1
    }
}
#[doc = "Field `OVRCRE` writer - Overcurrent Input Change Interrupt Enable"]
pub type OvrcreW<'a, REG> = crate::BitWriter<'a, REG, Ovrcre>;
impl<'a, REG> OvrcreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcre::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcre::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn pddetinte0(&self) -> Pddetinte0R {
        Pddetinte0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    pub fn sacke(&self) -> SackeR {
        SackeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn signe(&self) -> SigneR {
        SigneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    pub fn eoferre(&self) -> EoferreR {
        EoferreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Connection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn attche(&self) -> AttcheR {
        AttcheR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn dtche(&self) -> DtcheR {
        DtcheR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Enable"]
    #[inline(always)]
    pub fn bchge(&self) -> BchgeR {
        BchgeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn ovrcre(&self) -> OvrcreR {
        OvrcreR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENB1")
            .field("ovrcre", &self.ovrcre())
            .field("bchge", &self.bchge())
            .field("dtche", &self.dtche())
            .field("attche", &self.attche())
            .field("eoferre", &self.eoferre())
            .field("signe", &self.signe())
            .field("sacke", &self.sacke())
            .field("pddetinte0", &self.pddetinte0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn pddetinte0(&mut self) -> Pddetinte0W<Intenb1Spec> {
        Pddetinte0W::new(self, 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Enable"]
    #[inline(always)]
    pub fn sacke(&mut self) -> SackeW<Intenb1Spec> {
        SackeW::new(self, 4)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn signe(&mut self) -> SigneW<Intenb1Spec> {
        SigneW::new(self, 5)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Enable"]
    #[inline(always)]
    pub fn eoferre(&mut self) -> EoferreW<Intenb1Spec> {
        EoferreW::new(self, 6)
    }
    #[doc = "Bit 11 - Connection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn attche(&mut self) -> AttcheW<Intenb1Spec> {
        AttcheW::new(self, 11)
    }
    #[doc = "Bit 12 - Disconnection Detection Interrupt Enable"]
    #[inline(always)]
    pub fn dtche(&mut self) -> DtcheW<Intenb1Spec> {
        DtcheW::new(self, 12)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Enable"]
    #[inline(always)]
    pub fn bchge(&mut self) -> BchgeW<Intenb1Spec> {
        BchgeW::new(self, 14)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn ovrcre(&mut self) -> OvrcreW<Intenb1Spec> {
        OvrcreW::new(self, 15)
    }
}
#[doc = "Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intenb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenb1Spec;
impl crate::RegisterSpec for Intenb1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenb1::R`](R) reader structure"]
impl crate::Readable for Intenb1Spec {}
#[doc = "`write(|w| ..)` method takes [`intenb1::W`](W) writer structure"]
impl crate::Writable for Intenb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENB1 to value 0"]
impl crate::Resettable for Intenb1Spec {}

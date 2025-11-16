#[doc = "Register `INTSTS1` reader"]
pub type R = crate::R<Intsts1Spec>;
#[doc = "Register `INTSTS1` writer"]
pub type W = crate::W<Intsts1Spec>;
#[doc = "PDDET0 Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetint0 {
    #[doc = "0: PDDET0 detection interrupts are not generated."]
    _0 = 0,
    #[doc = "1: PDDET0 detection interrupts are generated."]
    _1 = 1,
}
impl From<Pddetint0> for bool {
    #[inline(always)]
    fn from(variant: Pddetint0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDETINT0` reader - PDDET0 Detection Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Pddetint0R = crate::BitReader<Pddetint0>;
impl Pddetint0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pddetint0 {
        match self.bits {
            false => Pddetint0::_0,
            true => Pddetint0::_1,
        }
    }
    #[doc = "PDDET0 detection interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetint0::_0
    }
    #[doc = "PDDET0 detection interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetint0::_1
    }
}
#[doc = "Field `PDDETINT0` writer - PDDET0 Detection Interrupt Status"]
pub type Pddetint0W<'a, REG> = crate::BitWriter0C<'a, REG, Pddetint0>;
impl<'a, REG> Pddetint0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDDET0 detection interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetint0::_0)
    }
    #[doc = "PDDET0 detection interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pddetint0::_1)
    }
}
#[doc = "Setup Transaction Normal Response Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sack {
    #[doc = "0: SACK interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SACK interrupts are generated."]
    _1 = 1,
}
impl From<Sack> for bool {
    #[inline(always)]
    fn from(variant: Sack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACK` reader - Setup Transaction Normal Response Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SackR = crate::BitReader<Sack>;
impl SackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sack {
        match self.bits {
            false => Sack::_0,
            true => Sack::_1,
        }
    }
    #[doc = "SACK interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sack::_0
    }
    #[doc = "SACK interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sack::_1
    }
}
#[doc = "Field `SACK` writer - Setup Transaction Normal Response Interrupt Status"]
pub type SackW<'a, REG> = crate::BitWriter0C<'a, REG, Sack>;
impl<'a, REG> SackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SACK interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sack::_0)
    }
    #[doc = "SACK interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sack::_1)
    }
}
#[doc = "Setup Transaction Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    #[doc = "0: SIGN interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SIGN interrupts are generated."]
    _1 = 1,
}
impl From<Sign> for bool {
    #[inline(always)]
    fn from(variant: Sign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGN` reader - Setup Transaction Error Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SignR = crate::BitReader<Sign>;
impl SignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sign {
        match self.bits {
            false => Sign::_0,
            true => Sign::_1,
        }
    }
    #[doc = "SIGN interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sign::_0
    }
    #[doc = "SIGN interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sign::_1
    }
}
#[doc = "Field `SIGN` writer - Setup Transaction Error Interrupt Status"]
pub type SignW<'a, REG> = crate::BitWriter0C<'a, REG, Sign>;
impl<'a, REG> SignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SIGN interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::_0)
    }
    #[doc = "SIGN interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::_1)
    }
}
#[doc = "EOF Error Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoferr {
    #[doc = "0: EOFERR interrupts are not generated."]
    _0 = 0,
    #[doc = "1: EOFERR interrupts are generated."]
    _1 = 1,
}
impl From<Eoferr> for bool {
    #[inline(always)]
    fn from(variant: Eoferr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOFERR` reader - EOF Error Detection Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type EoferrR = crate::BitReader<Eoferr>;
impl EoferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoferr {
        match self.bits {
            false => Eoferr::_0,
            true => Eoferr::_1,
        }
    }
    #[doc = "EOFERR interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eoferr::_0
    }
    #[doc = "EOFERR interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eoferr::_1
    }
}
#[doc = "Field `EOFERR` writer - EOF Error Detection Interrupt Status"]
pub type EoferrW<'a, REG> = crate::BitWriter0C<'a, REG, Eoferr>;
impl<'a, REG> EoferrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOFERR interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferr::_0)
    }
    #[doc = "EOFERR interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoferr::_1)
    }
}
#[doc = "ATTCH Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attch {
    #[doc = "0: ATTCH interrupts are not generated."]
    _0 = 0,
    #[doc = "1: ATTCH interrupts are generated."]
    _1 = 1,
}
impl From<Attch> for bool {
    #[inline(always)]
    fn from(variant: Attch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATTCH` reader - ATTCH Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AttchR = crate::BitReader<Attch>;
impl AttchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Attch {
        match self.bits {
            false => Attch::_0,
            true => Attch::_1,
        }
    }
    #[doc = "ATTCH interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Attch::_0
    }
    #[doc = "ATTCH interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Attch::_1
    }
}
#[doc = "Field `ATTCH` writer - ATTCH Interrupt Status"]
pub type AttchW<'a, REG> = crate::BitWriter0C<'a, REG, Attch>;
impl<'a, REG> AttchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ATTCH interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Attch::_0)
    }
    #[doc = "ATTCH interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Attch::_1)
    }
}
#[doc = "USB Disconnection Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtch {
    #[doc = "0: DTCH interrupts are not generated."]
    _0 = 0,
    #[doc = "1: DTCH interrupts are generated."]
    _1 = 1,
}
impl From<Dtch> for bool {
    #[inline(always)]
    fn from(variant: Dtch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCH` reader - USB Disconnection Detection Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DtchR = crate::BitReader<Dtch>;
impl DtchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtch {
        match self.bits {
            false => Dtch::_0,
            true => Dtch::_1,
        }
    }
    #[doc = "DTCH interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtch::_0
    }
    #[doc = "DTCH interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtch::_1
    }
}
#[doc = "Field `DTCH` writer - USB Disconnection Detection Interrupt Status"]
pub type DtchW<'a, REG> = crate::BitWriter0C<'a, REG, Dtch>;
impl<'a, REG> DtchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTCH interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtch::_0)
    }
    #[doc = "DTCH interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtch::_1)
    }
}
#[doc = "USB Bus Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bchg {
    #[doc = "0: BCHG interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BCHG interrupts are generated."]
    _1 = 1,
}
impl From<Bchg> for bool {
    #[inline(always)]
    fn from(variant: Bchg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCHG` reader - USB Bus Change Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type BchgR = crate::BitReader<Bchg>;
impl BchgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bchg {
        match self.bits {
            false => Bchg::_0,
            true => Bchg::_1,
        }
    }
    #[doc = "BCHG interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bchg::_0
    }
    #[doc = "BCHG interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bchg::_1
    }
}
#[doc = "Field `BCHG` writer - USB Bus Change Interrupt Status"]
pub type BchgW<'a, REG> = crate::BitWriter0C<'a, REG, Bchg>;
impl<'a, REG> BchgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BCHG interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bchg::_0)
    }
    #[doc = "BCHG interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bchg::_1)
    }
}
#[doc = "Overcurrent Input Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrcr {
    #[doc = "0: OVRCR interrupts are not generated."]
    _0 = 0,
    #[doc = "1: OVRCR interrupts are generated."]
    _1 = 1,
}
impl From<Ovrcr> for bool {
    #[inline(always)]
    fn from(variant: Ovrcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCR` reader - Overcurrent Input Change Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OvrcrR = crate::BitReader<Ovrcr>;
impl OvrcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrcr {
        match self.bits {
            false => Ovrcr::_0,
            true => Ovrcr::_1,
        }
    }
    #[doc = "OVRCR interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrcr::_0
    }
    #[doc = "OVRCR interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrcr::_1
    }
}
#[doc = "Field `OVRCR` writer - Overcurrent Input Change Interrupt Status"]
pub type OvrcrW<'a, REG> = crate::BitWriter0C<'a, REG, Ovrcr>;
impl<'a, REG> OvrcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OVRCR interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcr::_0)
    }
    #[doc = "OVRCR interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrcr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub fn pddetint0(&self) -> Pddetint0R {
        Pddetint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub fn sack(&self) -> SackR {
        SackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub fn eoferr(&self) -> EoferrR {
        EoferrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    pub fn attch(&self) -> AttchR {
        AttchR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub fn dtch(&self) -> DtchR {
        DtchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub fn bchg(&self) -> BchgR {
        BchgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub fn ovrcr(&self) -> OvrcrR {
        OvrcrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS1").finish()
    }
}
impl W {
    #[doc = "Bit 0 - PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub fn pddetint0(&mut self) -> Pddetint0W<Intsts1Spec> {
        Pddetint0W::new(self, 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub fn sack(&mut self) -> SackW<Intsts1Spec> {
        SackW::new(self, 4)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<Intsts1Spec> {
        SignW::new(self, 5)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub fn eoferr(&mut self) -> EoferrW<Intsts1Spec> {
        EoferrW::new(self, 6)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    pub fn attch(&mut self) -> AttchW<Intsts1Spec> {
        AttchW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub fn dtch(&mut self) -> DtchW<Intsts1Spec> {
        DtchW::new(self, 12)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub fn bchg(&mut self) -> BchgW<Intsts1Spec> {
        BchgW::new(self, 14)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub fn ovrcr(&mut self) -> OvrcrW<Intsts1Spec> {
        OvrcrW::new(self, 15)
    }
}
#[doc = "Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intsts1Spec;
impl crate::RegisterSpec for Intsts1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intsts1::R`](R) reader structure"]
impl crate::Readable for Intsts1Spec {}
#[doc = "`write(|w| ..)` method takes [`intsts1::W`](W) writer structure"]
impl crate::Writable for Intsts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xd871;
}
#[doc = "`reset()` method sets INTSTS1 to value 0"]
impl crate::Resettable for Intsts1Spec {}

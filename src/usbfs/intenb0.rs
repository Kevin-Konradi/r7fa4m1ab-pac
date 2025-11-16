#[doc = "Register `INTENB0` reader"]
pub type R = crate::R<Intenb0Spec>;
#[doc = "Register `INTENB0` writer"]
pub type W = crate::W<Intenb0Spec>;
#[doc = "Buffer Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Brdye> for bool {
    #[inline(always)]
    fn from(variant: Brdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRDYE` reader - Buffer Ready Interrupt Enable"]
pub type BrdyeR = crate::BitReader<Brdye>;
impl BrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brdye {
        match self.bits {
            false => Brdye::_0,
            true => Brdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brdye::_1
    }
}
#[doc = "Field `BRDYE` writer - Buffer Ready Interrupt Enable"]
pub type BrdyeW<'a, REG> = crate::BitWriter<'a, REG, Brdye>;
impl<'a, REG> BrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brdye::_1)
    }
}
#[doc = "Buffer Not Ready Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nrdye {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Nrdye> for bool {
    #[inline(always)]
    fn from(variant: Nrdye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDYE` reader - Buffer Not Ready Response Interrupt Enable"]
pub type NrdyeR = crate::BitReader<Nrdye>;
impl NrdyeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nrdye {
        match self.bits {
            false => Nrdye::_0,
            true => Nrdye::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nrdye::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nrdye::_1
    }
}
#[doc = "Field `NRDYE` writer - Buffer Not Ready Response Interrupt Enable"]
pub type NrdyeW<'a, REG> = crate::BitWriter<'a, REG, Nrdye>;
impl<'a, REG> NrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nrdye::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nrdye::_1)
    }
}
#[doc = "Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bempe {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Bempe> for bool {
    #[inline(always)]
    fn from(variant: Bempe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEMPE` reader - Buffer Empty Interrupt Enable"]
pub type BempeR = crate::BitReader<Bempe>;
impl BempeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bempe {
        match self.bits {
            false => Bempe::_0,
            true => Bempe::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bempe::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bempe::_1
    }
}
#[doc = "Field `BEMPE` writer - Buffer Empty Interrupt Enable"]
pub type BempeW<'a, REG> = crate::BitWriter<'a, REG, Bempe>;
impl<'a, REG> BempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bempe::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bempe::_1)
    }
}
#[doc = "Control Transfer Stage Transition Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctre {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Ctre> for bool {
    #[inline(always)]
    fn from(variant: Ctre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRE` reader - Control Transfer Stage Transition Interrupt Enable"]
pub type CtreR = crate::BitReader<Ctre>;
impl CtreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctre {
        match self.bits {
            false => Ctre::_0,
            true => Ctre::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctre::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctre::_1
    }
}
#[doc = "Field `CTRE` writer - Control Transfer Stage Transition Interrupt Enable"]
pub type CtreW<'a, REG> = crate::BitWriter<'a, REG, Ctre>;
impl<'a, REG> CtreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctre::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctre::_1)
    }
}
#[doc = "Device State Transition Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvse {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Dvse> for bool {
    #[inline(always)]
    fn from(variant: Dvse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVSE` reader - Device State Transition Interrupt Enable"]
pub type DvseR = crate::BitReader<Dvse>;
impl DvseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dvse {
        match self.bits {
            false => Dvse::_0,
            true => Dvse::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvse::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvse::_1
    }
}
#[doc = "Field `DVSE` writer - Device State Transition Interrupt Enable"]
pub type DvseW<'a, REG> = crate::BitWriter<'a, REG, Dvse>;
impl<'a, REG> DvseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvse::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvse::_1)
    }
}
#[doc = "Frame Number Update Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofe {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Sofe> for bool {
    #[inline(always)]
    fn from(variant: Sofe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFE` reader - Frame Number Update Interrupt Enable"]
pub type SofeR = crate::BitReader<Sofe>;
impl SofeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sofe {
        match self.bits {
            false => Sofe::_0,
            true => Sofe::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sofe::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sofe::_1
    }
}
#[doc = "Field `SOFE` writer - Frame Number Update Interrupt Enable"]
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG, Sofe>;
impl<'a, REG> SofeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::_1)
    }
}
#[doc = "Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsme {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Rsme> for bool {
    #[inline(always)]
    fn from(variant: Rsme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSME` reader - Resume Interrupt Enable"]
pub type RsmeR = crate::BitReader<Rsme>;
impl RsmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsme {
        match self.bits {
            false => Rsme::_0,
            true => Rsme::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsme::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsme::_1
    }
}
#[doc = "Field `RSME` writer - Resume Interrupt Enable"]
pub type RsmeW<'a, REG> = crate::BitWriter<'a, REG, Rsme>;
impl<'a, REG> RsmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsme::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsme::_1)
    }
}
#[doc = "VBUS Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbse {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<Vbse> for bool {
    #[inline(always)]
    fn from(variant: Vbse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBSE` reader - VBUS Interrupt Enable"]
pub type VbseR = crate::BitReader<Vbse>;
impl VbseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbse {
        match self.bits {
            false => Vbse::_0,
            true => Vbse::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbse::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbse::_1
    }
}
#[doc = "Field `VBSE` writer - VBUS Interrupt Enable"]
pub type VbseW<'a, REG> = crate::BitWriter<'a, REG, Vbse>;
impl<'a, REG> VbseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbse::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbse::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brdye(&self) -> BrdyeR {
        BrdyeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    pub fn nrdye(&self) -> NrdyeR {
        NrdyeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn bempe(&self) -> BempeR {
        BempeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    pub fn ctre(&self) -> CtreR {
        CtreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Enable"]
    #[inline(always)]
    pub fn dvse(&self) -> DvseR {
        DvseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Number Update Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn rsme(&self) -> RsmeR {
        RsmeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBUS Interrupt Enable"]
    #[inline(always)]
    pub fn vbse(&self) -> VbseR {
        VbseR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENB0")
            .field("vbse", &self.vbse())
            .field("rsme", &self.rsme())
            .field("sofe", &self.sofe())
            .field("dvse", &self.dvse())
            .field("ctre", &self.ctre())
            .field("bempe", &self.bempe())
            .field("nrdye", &self.nrdye())
            .field("brdye", &self.brdye())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brdye(&mut self) -> BrdyeW<Intenb0Spec> {
        BrdyeW::new(self, 8)
    }
    #[doc = "Bit 9 - Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    pub fn nrdye(&mut self) -> NrdyeW<Intenb0Spec> {
        NrdyeW::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn bempe(&mut self) -> BempeW<Intenb0Spec> {
        BempeW::new(self, 10)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    pub fn ctre(&mut self) -> CtreW<Intenb0Spec> {
        CtreW::new(self, 11)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Enable"]
    #[inline(always)]
    pub fn dvse(&mut self) -> DvseW<Intenb0Spec> {
        DvseW::new(self, 12)
    }
    #[doc = "Bit 13 - Frame Number Update Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SofeW<Intenb0Spec> {
        SofeW::new(self, 13)
    }
    #[doc = "Bit 14 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn rsme(&mut self) -> RsmeW<Intenb0Spec> {
        RsmeW::new(self, 14)
    }
    #[doc = "Bit 15 - VBUS Interrupt Enable"]
    #[inline(always)]
    pub fn vbse(&mut self) -> VbseW<Intenb0Spec> {
        VbseW::new(self, 15)
    }
}
#[doc = "Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intenb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenb0Spec;
impl crate::RegisterSpec for Intenb0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenb0::R`](R) reader structure"]
impl crate::Readable for Intenb0Spec {}
#[doc = "`write(|w| ..)` method takes [`intenb0::W`](W) writer structure"]
impl crate::Writable for Intenb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENB0 to value 0"]
impl crate::Resettable for Intenb0Spec {}

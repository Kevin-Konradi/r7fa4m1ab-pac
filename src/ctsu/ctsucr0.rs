#[doc = "Register `CTSUCR0` reader"]
pub type R = crate::R<Ctsucr0Spec>;
#[doc = "Register `CTSUCR0` writer"]
pub type W = crate::W<Ctsucr0Spec>;
#[doc = "CTSU Measurement Operation Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsustrt {
    #[doc = "0: Measurement operation stops."]
    _0 = 0,
    #[doc = "1: Measurement operation starts."]
    _1 = 1,
}
impl From<Ctsustrt> for bool {
    #[inline(always)]
    fn from(variant: Ctsustrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUSTRT` reader - CTSU Measurement Operation Start"]
pub type CtsustrtR = crate::BitReader<Ctsustrt>;
impl CtsustrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsustrt {
        match self.bits {
            false => Ctsustrt::_0,
            true => Ctsustrt::_1,
        }
    }
    #[doc = "Measurement operation stops."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsustrt::_0
    }
    #[doc = "Measurement operation starts."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsustrt::_1
    }
}
#[doc = "Field `CTSUSTRT` writer - CTSU Measurement Operation Start"]
pub type CtsustrtW<'a, REG> = crate::BitWriter<'a, REG, Ctsustrt>;
impl<'a, REG> CtsustrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measurement operation stops."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsustrt::_0)
    }
    #[doc = "Measurement operation starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsustrt::_1)
    }
}
#[doc = "CTSU Measurement Operation Start Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsucap {
    #[doc = "0: Software trigger."]
    _0 = 0,
    #[doc = "1: External trigger."]
    _1 = 1,
}
impl From<Ctsucap> for bool {
    #[inline(always)]
    fn from(variant: Ctsucap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUCAP` reader - CTSU Measurement Operation Start Trigger Select"]
pub type CtsucapR = crate::BitReader<Ctsucap>;
impl CtsucapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsucap {
        match self.bits {
            false => Ctsucap::_0,
            true => Ctsucap::_1,
        }
    }
    #[doc = "Software trigger."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsucap::_0
    }
    #[doc = "External trigger."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsucap::_1
    }
}
#[doc = "Field `CTSUCAP` writer - CTSU Measurement Operation Start Trigger Select"]
pub type CtsucapW<'a, REG> = crate::BitWriter<'a, REG, Ctsucap>;
impl<'a, REG> CtsucapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsucap::_0)
    }
    #[doc = "External trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsucap::_1)
    }
}
#[doc = "CTSU Wait State Power-Saving Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsusnz {
    #[doc = "0: Power-saving function during wait state is disabled."]
    _0 = 0,
    #[doc = "1: Power-saving function during wait state is enabled."]
    _1 = 1,
}
impl From<Ctsusnz> for bool {
    #[inline(always)]
    fn from(variant: Ctsusnz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUSNZ` reader - CTSU Wait State Power-Saving Enable"]
pub type CtsusnzR = crate::BitReader<Ctsusnz>;
impl CtsusnzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsusnz {
        match self.bits {
            false => Ctsusnz::_0,
            true => Ctsusnz::_1,
        }
    }
    #[doc = "Power-saving function during wait state is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsusnz::_0
    }
    #[doc = "Power-saving function during wait state is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsusnz::_1
    }
}
#[doc = "Field `CTSUSNZ` writer - CTSU Wait State Power-Saving Enable"]
pub type CtsusnzW<'a, REG> = crate::BitWriter<'a, REG, Ctsusnz>;
impl<'a, REG> CtsusnzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power-saving function during wait state is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusnz::_0)
    }
    #[doc = "Power-saving function during wait state is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsusnz::_1)
    }
}
#[doc = "CTSU Control Block Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuinit {
    #[doc = "0: Writing a 0 has no effect, this bit is read as 0."]
    _0 = 0,
    #[doc = "1: initializes the CTSU control block and registers."]
    _1 = 1,
}
impl From<Ctsuinit> for bool {
    #[inline(always)]
    fn from(variant: Ctsuinit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUINIT` reader - CTSU Control Block Initialization"]
pub type CtsuinitR = crate::BitReader<Ctsuinit>;
impl CtsuinitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuinit {
        match self.bits {
            false => Ctsuinit::_0,
            true => Ctsuinit::_1,
        }
    }
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuinit::_0
    }
    #[doc = "initializes the CTSU control block and registers."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuinit::_1
    }
}
#[doc = "Field `CTSUINIT` writer - CTSU Control Block Initialization"]
pub type CtsuinitW<'a, REG> = crate::BitWriter<'a, REG, Ctsuinit>;
impl<'a, REG> CtsuinitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuinit::_0)
    }
    #[doc = "initializes the CTSU control block and registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuinit::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn ctsustrt(&self) -> CtsustrtR {
        CtsustrtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn ctsucap(&self) -> CtsucapR {
        CtsucapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn ctsusnz(&self) -> CtsusnzR {
        CtsusnzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    pub fn ctsuinit(&self) -> CtsuinitR {
        CtsuinitR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCR0")
            .field("ctsuinit", &self.ctsuinit())
            .field("ctsusnz", &self.ctsusnz())
            .field("ctsucap", &self.ctsucap())
            .field("ctsustrt", &self.ctsustrt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn ctsustrt(&mut self) -> CtsustrtW<Ctsucr0Spec> {
        CtsustrtW::new(self, 0)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn ctsucap(&mut self) -> CtsucapW<Ctsucr0Spec> {
        CtsucapW::new(self, 1)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn ctsusnz(&mut self) -> CtsusnzW<Ctsucr0Spec> {
        CtsusnzW::new(self, 2)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    pub fn ctsuinit(&mut self) -> CtsuinitW<Ctsucr0Spec> {
        CtsuinitW::new(self, 4)
    }
}
#[doc = "CTSU Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsucr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsucr0Spec;
impl crate::RegisterSpec for Ctsucr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsucr0::R`](R) reader structure"]
impl crate::Readable for Ctsucr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsucr0::W`](W) writer structure"]
impl crate::Writable for Ctsucr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCR0 to value 0"]
impl crate::Resettable for Ctsucr0Spec {}

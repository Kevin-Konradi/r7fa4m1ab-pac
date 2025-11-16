#[doc = "Register `MOMCR` reader"]
pub type R = crate::R<MomcrSpec>;
#[doc = "Register `MOMCR` writer"]
pub type W = crate::W<MomcrSpec>;
#[doc = "Main Clock Oscillator Drive Capability 1 Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modrv1 {
    #[doc = "0: 10 MHz to 20 MHz"]
    _0 = 0,
    #[doc = "1: 1 MHz to 10 MHz."]
    _1 = 1,
}
impl From<Modrv1> for bool {
    #[inline(always)]
    fn from(variant: Modrv1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODRV1` reader - Main Clock Oscillator Drive Capability 1 Switching"]
pub type Modrv1R = crate::BitReader<Modrv1>;
impl Modrv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modrv1 {
        match self.bits {
            false => Modrv1::_0,
            true => Modrv1::_1,
        }
    }
    #[doc = "10 MHz to 20 MHz"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Modrv1::_0
    }
    #[doc = "1 MHz to 10 MHz."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Modrv1::_1
    }
}
#[doc = "Field `MODRV1` writer - Main Clock Oscillator Drive Capability 1 Switching"]
pub type Modrv1W<'a, REG> = crate::BitWriter<'a, REG, Modrv1>;
impl<'a, REG> Modrv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10 MHz to 20 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv1::_0)
    }
    #[doc = "1 MHz to 10 MHz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Modrv1::_1)
    }
}
#[doc = "Main Clock Oscillator Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mosel {
    #[doc = "0: Resonator"]
    _0 = 0,
    #[doc = "1: External clock input"]
    _1 = 1,
}
impl From<Mosel> for bool {
    #[inline(always)]
    fn from(variant: Mosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOSEL` reader - Main Clock Oscillator Switching"]
pub type MoselR = crate::BitReader<Mosel>;
impl MoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mosel {
        match self.bits {
            false => Mosel::_0,
            true => Mosel::_1,
        }
    }
    #[doc = "Resonator"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mosel::_0
    }
    #[doc = "External clock input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mosel::_1
    }
}
#[doc = "Field `MOSEL` writer - Main Clock Oscillator Switching"]
pub type MoselW<'a, REG> = crate::BitWriter<'a, REG, Mosel>;
impl<'a, REG> MoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resonator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mosel::_0)
    }
    #[doc = "External clock input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mosel::_1)
    }
}
impl R {
    #[doc = "Bit 3 - Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(&self) -> Modrv1R {
        Modrv1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(&self) -> MoselR {
        MoselR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOMCR")
            .field("mosel", &self.mosel())
            .field("modrv1", &self.modrv1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(&mut self) -> Modrv1W<MomcrSpec> {
        Modrv1W::new(self, 3)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(&mut self) -> MoselW<MomcrSpec> {
        MoselW::new(self, 6)
    }
}
#[doc = "Main Clock Oscillator Mode Oscillation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`momcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MomcrSpec;
impl crate::RegisterSpec for MomcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`momcr::R`](R) reader structure"]
impl crate::Readable for MomcrSpec {}
#[doc = "`write(|w| ..)` method takes [`momcr::W`](W) writer structure"]
impl crate::Writable for MomcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOMCR to value 0"]
impl crate::Resettable for MomcrSpec {}

#[doc = "Register `VBTWTER` reader"]
pub type R = crate::R<VbtwterSpec>;
#[doc = "Register `VBTWTER` writer"]
pub type W = crate::W<VbtwterSpec>;
#[doc = "VBATWIO0 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch0e {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    _1 = 1,
}
impl From<Vch0e> for bool {
    #[inline(always)]
    fn from(variant: Vch0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH0E` reader - VBATWIO0 Pin Enable"]
pub type Vch0eR = crate::BitReader<Vch0e>;
impl Vch0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch0e {
        match self.bits {
            false => Vch0e::_0,
            true => Vch0e::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch0e::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch0e::_1
    }
}
#[doc = "Field `VCH0E` writer - VBATWIO0 Pin Enable"]
pub type Vch0eW<'a, REG> = crate::BitWriter<'a, REG, Vch0e>;
impl<'a, REG> Vch0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0e::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0e::_1)
    }
}
#[doc = "VBATWIO1 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch1e {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<Vch1e> for bool {
    #[inline(always)]
    fn from(variant: Vch1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH1E` reader - VBATWIO1 Pin Enable"]
pub type Vch1eR = crate::BitReader<Vch1e>;
impl Vch1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch1e {
        match self.bits {
            false => Vch1e::_0,
            true => Vch1e::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch1e::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch1e::_1
    }
}
#[doc = "Field `VCH1E` writer - VBATWIO1 Pin Enable"]
pub type Vch1eW<'a, REG> = crate::BitWriter<'a, REG, Vch1e>;
impl<'a, REG> Vch1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1e::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1e::_1)
    }
}
#[doc = "VBATWIO2 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch2e {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    _1 = 1,
}
impl From<Vch2e> for bool {
    #[inline(always)]
    fn from(variant: Vch2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH2E` reader - VBATWIO2 Pin Enable"]
pub type Vch2eR = crate::BitReader<Vch2e>;
impl Vch2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch2e {
        match self.bits {
            false => Vch2e::_0,
            true => Vch2e::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch2e::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch2e::_1
    }
}
#[doc = "Field `VCH2E` writer - VBATWIO2 Pin Enable"]
pub type Vch2eW<'a, REG> = crate::BitWriter<'a, REG, Vch2e>;
impl<'a, REG> Vch2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2e::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2e::_1)
    }
}
#[doc = "RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrtcie {
    #[doc = "0: VBATT wakeup triggered by RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<Vrtcie> for bool {
    #[inline(always)]
    fn from(variant: Vrtcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRTCIE` reader - RTC Periodic Signal Enable"]
pub type VrtcieR = crate::BitReader<Vrtcie>;
impl VrtcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrtcie {
        match self.bits {
            false => Vrtcie::_0,
            true => Vrtcie::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vrtcie::_0
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vrtcie::_1
    }
}
#[doc = "Field `VRTCIE` writer - RTC Periodic Signal Enable"]
pub type VrtcieW<'a, REG> = crate::BitWriter<'a, REG, Vrtcie>;
impl<'a, REG> VrtcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcie::_0)
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcie::_1)
    }
}
#[doc = "RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrtcae {
    #[doc = "0: VBATT wakeup triggered by RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<Vrtcae> for bool {
    #[inline(always)]
    fn from(variant: Vrtcae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRTCAE` reader - RTC Alarm Signal Enable"]
pub type VrtcaeR = crate::BitReader<Vrtcae>;
impl VrtcaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrtcae {
        match self.bits {
            false => Vrtcae::_0,
            true => Vrtcae::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vrtcae::_0
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vrtcae::_1
    }
}
#[doc = "Field `VRTCAE` writer - RTC Alarm Signal Enable"]
pub type VrtcaeW<'a, REG> = crate::BitWriter<'a, REG, Vrtcae>;
impl<'a, REG> VrtcaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcae::_0)
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcae::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Pin Enable"]
    #[inline(always)]
    pub fn vch0e(&self) -> Vch0eR {
        Vch0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Pin Enable"]
    #[inline(always)]
    pub fn vch1e(&self) -> Vch1eR {
        Vch1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Pin Enable"]
    #[inline(always)]
    pub fn vch2e(&self) -> Vch2eR {
        Vch2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn vrtcie(&self) -> VrtcieR {
        VrtcieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn vrtcae(&self) -> VrtcaeR {
        VrtcaeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWTER")
            .field("vrtcae", &self.vrtcae())
            .field("vrtcie", &self.vrtcie())
            .field("vch2e", &self.vch2e())
            .field("vch1e", &self.vch1e())
            .field("vch0e", &self.vch0e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Pin Enable"]
    #[inline(always)]
    pub fn vch0e(&mut self) -> Vch0eW<VbtwterSpec> {
        Vch0eW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Pin Enable"]
    #[inline(always)]
    pub fn vch1e(&mut self) -> Vch1eW<VbtwterSpec> {
        Vch1eW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATWIO2 Pin Enable"]
    #[inline(always)]
    pub fn vch2e(&mut self) -> Vch2eW<VbtwterSpec> {
        Vch2eW::new(self, 2)
    }
    #[doc = "Bit 3 - RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn vrtcie(&mut self) -> VrtcieW<VbtwterSpec> {
        VrtcieW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn vrtcae(&mut self) -> VrtcaeW<VbtwterSpec> {
        VrtcaeW::new(self, 4)
    }
}
#[doc = "VBATT Wakeup Trigger source Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtwterSpec;
impl crate::RegisterSpec for VbtwterSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwter::R`](R) reader structure"]
impl crate::Readable for VbtwterSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwter::W`](W) writer structure"]
impl crate::Writable for VbtwterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWTER to value 0"]
impl crate::Resettable for VbtwterSpec {}

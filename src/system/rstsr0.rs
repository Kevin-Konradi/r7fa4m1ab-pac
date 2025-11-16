#[doc = "Register `RSTSR0` reader"]
pub type R = crate::R<Rstsr0Spec>;
#[doc = "Register `RSTSR0` writer"]
pub type W = crate::W<Rstsr0Spec>;
#[doc = "Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porf {
    #[doc = "0: Power-on reset not detected."]
    _0 = 0,
    #[doc = "1: Power-on reset detected."]
    _1 = 1,
}
impl From<Porf> for bool {
    #[inline(always)]
    fn from(variant: Porf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORF` reader - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PorfR = crate::BitReader<Porf>;
impl PorfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porf {
        match self.bits {
            false => Porf::_0,
            true => Porf::_1,
        }
    }
    #[doc = "Power-on reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Porf::_0
    }
    #[doc = "Power-on reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Porf::_1
    }
}
#[doc = "Field `PORF` writer - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
pub type PorfW<'a, REG> = crate::BitWriter0C<'a, REG, Porf>;
impl<'a, REG> PorfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power-on reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porf::_0)
    }
    #[doc = "Power-on reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porf::_1)
    }
}
#[doc = "Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd0rf {
    #[doc = "0: Voltage Monitor 0 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 0 reset detected."]
    _1 = 1,
}
impl From<Lvd0rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd0rf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD0RF` reader - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Lvd0rfR = crate::BitReader<Lvd0rf>;
impl Lvd0rfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd0rf {
        match self.bits {
            false => Lvd0rf::_0,
            true => Lvd0rf::_1,
        }
    }
    #[doc = "Voltage Monitor 0 reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd0rf::_0
    }
    #[doc = "Voltage Monitor 0 reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd0rf::_1
    }
}
#[doc = "Field `LVD0RF` writer - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
pub type Lvd0rfW<'a, REG> = crate::BitWriter0C<'a, REG, Lvd0rf>;
impl<'a, REG> Lvd0rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor 0 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd0rf::_0)
    }
    #[doc = "Voltage Monitor 0 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd0rf::_1)
    }
}
#[doc = "Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1rf {
    #[doc = "0: Voltage Monitor 1 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 1 reset detected."]
    _1 = 1,
}
impl From<Lvd1rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd1rf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1RF` reader - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Lvd1rfR = crate::BitReader<Lvd1rf>;
impl Lvd1rfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1rf {
        match self.bits {
            false => Lvd1rf::_0,
            true => Lvd1rf::_1,
        }
    }
    #[doc = "Voltage Monitor 1 reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1rf::_0
    }
    #[doc = "Voltage Monitor 1 reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1rf::_1
    }
}
#[doc = "Field `LVD1RF` writer - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
pub type Lvd1rfW<'a, REG> = crate::BitWriter0C<'a, REG, Lvd1rf>;
impl<'a, REG> Lvd1rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor 1 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1rf::_0)
    }
    #[doc = "Voltage Monitor 1 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1rf::_1)
    }
}
#[doc = "Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2rf {
    #[doc = "0: Voltage Monitor 2 reset not detected."]
    _0 = 0,
    #[doc = "1: Voltage Monitor 2 reset detected."]
    _1 = 1,
}
impl From<Lvd2rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd2rf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2RF` reader - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Lvd2rfR = crate::BitReader<Lvd2rf>;
impl Lvd2rfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2rf {
        match self.bits {
            false => Lvd2rf::_0,
            true => Lvd2rf::_1,
        }
    }
    #[doc = "Voltage Monitor 2 reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2rf::_0
    }
    #[doc = "Voltage Monitor 2 reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2rf::_1
    }
}
#[doc = "Field `LVD2RF` writer - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
pub type Lvd2rfW<'a, REG> = crate::BitWriter0C<'a, REG, Lvd2rf>;
impl<'a, REG> Lvd2rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor 2 reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2rf::_0)
    }
    #[doc = "Voltage Monitor 2 reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2rf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn porf(&self) -> PorfR {
        PorfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd0rf(&self) -> Lvd0rfR {
        Lvd0rfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd1rf(&self) -> Lvd1rfR {
        Lvd1rfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd2rf(&self) -> Lvd2rfR {
        Lvd2rfR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR0").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn porf(&mut self) -> PorfW<Rstsr0Spec> {
        PorfW::new(self, 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd0rf(&mut self) -> Lvd0rfW<Rstsr0Spec> {
        Lvd0rfW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd1rf(&mut self) -> Lvd1rfW<Rstsr0Spec> {
        Lvd1rfW::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd2rf(&mut self) -> Lvd2rfW<Rstsr0Spec> {
        Lvd2rfW::new(self, 3)
    }
}
#[doc = "Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rstsr0Spec;
impl crate::RegisterSpec for Rstsr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rstsr0::R`](R) reader structure"]
impl crate::Readable for Rstsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rstsr0::W`](W) writer structure"]
impl crate::Writable for Rstsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x0f;
}
#[doc = "`reset()` method sets RSTSR0 to value 0"]
impl crate::Resettable for Rstsr0Spec {}

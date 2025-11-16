#[doc = "Register `VBTWEGR` reader"]
pub type R = crate::R<VbtwegrSpec>;
#[doc = "Register `VBTWEGR` writer"]
pub type W = crate::W<VbtwegrSpec>;
#[doc = "VBATWIO0 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch0eg {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<Vch0eg> for bool {
    #[inline(always)]
    fn from(variant: Vch0eg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH0EG` reader - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type Vch0egR = crate::BitReader<Vch0eg>;
impl Vch0egR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch0eg {
        match self.bits {
            false => Vch0eg::_0,
            true => Vch0eg::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch0eg::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch0eg::_1
    }
}
#[doc = "Field `VCH0EG` writer - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type Vch0egW<'a, REG> = crate::BitWriter<'a, REG, Vch0eg>;
impl<'a, REG> Vch0egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0eg::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0eg::_1)
    }
}
#[doc = "VBATWIO1 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch1eg {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<Vch1eg> for bool {
    #[inline(always)]
    fn from(variant: Vch1eg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH1EG` reader - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type Vch1egR = crate::BitReader<Vch1eg>;
impl Vch1egR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch1eg {
        match self.bits {
            false => Vch1eg::_0,
            true => Vch1eg::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch1eg::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch1eg::_1
    }
}
#[doc = "Field `VCH1EG` writer - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type Vch1egW<'a, REG> = crate::BitWriter<'a, REG, Vch1eg>;
impl<'a, REG> Vch1egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1eg::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1eg::_1)
    }
}
#[doc = "VBATWIO2 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch2eg {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<Vch2eg> for bool {
    #[inline(always)]
    fn from(variant: Vch2eg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH2EG` reader - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type Vch2egR = crate::BitReader<Vch2eg>;
impl Vch2egR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch2eg {
        match self.bits {
            false => Vch2eg::_0,
            true => Vch2eg::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch2eg::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch2eg::_1
    }
}
#[doc = "Field `VCH2EG` writer - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type Vch2egW<'a, REG> = crate::BitWriter<'a, REG, Vch2eg>;
impl<'a, REG> Vch2egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2eg::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2eg::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch0eg(&self) -> Vch0egR {
        Vch0egR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch1eg(&self) -> Vch1egR {
        Vch1egR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch2eg(&self) -> Vch2egR {
        Vch2egR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWEGR")
            .field("vch2eg", &self.vch2eg())
            .field("vch1eg", &self.vch1eg())
            .field("vch0eg", &self.vch0eg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch0eg(&mut self) -> Vch0egW<VbtwegrSpec> {
        Vch0egW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch1eg(&mut self) -> Vch1egW<VbtwegrSpec> {
        Vch1egW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch2eg(&mut self) -> Vch2egW<VbtwegrSpec> {
        Vch2egW::new(self, 2)
    }
}
#[doc = "VBATT Wakeup Trigger source Edge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwegr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwegr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtwegrSpec;
impl crate::RegisterSpec for VbtwegrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwegr::R`](R) reader structure"]
impl crate::Readable for VbtwegrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwegr::W`](W) writer structure"]
impl crate::Writable for VbtwegrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWEGR to value 0"]
impl crate::Resettable for VbtwegrSpec {}

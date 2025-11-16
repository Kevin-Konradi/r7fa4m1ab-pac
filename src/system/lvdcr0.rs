#[doc = "Register `LVD%sCR0` reader"]
pub type R = crate::R<Lvdcr0Spec>;
#[doc = "Register `LVD%sCR0` writer"]
pub type W = crate::W<Lvdcr0Spec>;
#[doc = "Voltage Monitor Interrupt/Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Voltage Monitor Interrupt/Reset Enable"]
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
#[doc = "Field `RIE` writer - Voltage Monitor Interrupt/Reset Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
#[doc = "Voltage Monitor Circuit Comparison Result Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpe {
    #[doc = "0: Voltage Monitor circuit comparison result output disabled."]
    _0 = 0,
    #[doc = "1: Voltage Monitor circuit comparison result output enabled."]
    _1 = 1,
}
impl From<Cmpe> for bool {
    #[inline(always)]
    fn from(variant: Cmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPE` reader - Voltage Monitor Circuit Comparison Result Output Enable"]
pub type CmpeR = crate::BitReader<Cmpe>;
impl CmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpe {
        match self.bits {
            false => Cmpe::_0,
            true => Cmpe::_1,
        }
    }
    #[doc = "Voltage Monitor circuit comparison result output disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpe::_0
    }
    #[doc = "Voltage Monitor circuit comparison result output enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpe::_1
    }
}
#[doc = "Field `CMPE` writer - Voltage Monitor Circuit Comparison Result Output Enable"]
pub type CmpeW<'a, REG> = crate::BitWriter<'a, REG, Cmpe>;
impl<'a, REG> CmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor circuit comparison result output disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_0)
    }
    #[doc = "Voltage Monitor circuit comparison result output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_1)
    }
}
#[doc = "Voltage Monitor Circuit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ri {
    #[doc = "0: Voltage Monitor interrupt during Vdet1 passage"]
    _0 = 0,
    #[doc = "1: Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    _1 = 1,
}
impl From<Ri> for bool {
    #[inline(always)]
    fn from(variant: Ri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Voltage Monitor Circuit Mode Select"]
pub type RiR = crate::BitReader<Ri>;
impl RiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ri {
        match self.bits {
            false => Ri::_0,
            true => Ri::_1,
        }
    }
    #[doc = "Voltage Monitor interrupt during Vdet1 passage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ri::_0
    }
    #[doc = "Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ri::_1
    }
}
#[doc = "Field `RI` writer - Voltage Monitor Circuit Mode Select"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG, Ri>;
impl<'a, REG> RiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor interrupt during Vdet1 passage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_0)
    }
    #[doc = "Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_1)
    }
}
#[doc = "Voltage Monitor Reset Negate Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rn {
    #[doc = "0: Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    _0 = 0,
    #[doc = "1: Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    _1 = 1,
}
impl From<Rn> for bool {
    #[inline(always)]
    fn from(variant: Rn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RN` reader - Voltage Monitor Reset Negate Select"]
pub type RnR = crate::BitReader<Rn>;
impl RnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rn {
        match self.bits {
            false => Rn::_0,
            true => Rn::_1,
        }
    }
    #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rn::_0
    }
    #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rn::_1
    }
}
#[doc = "Field `RN` writer - Voltage Monitor Reset Negate Select"]
pub type RnW<'a, REG> = crate::BitWriter<'a, REG, Rn>;
impl<'a, REG> RnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_0)
    }
    #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CmpeR {
        CmpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&self) -> RnR {
        RnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDCR0")
            .field("rn", &self.rn())
            .field("ri", &self.ri())
            .field("cmpe", &self.cmpe())
            .field("rie", &self.rie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<Lvdcr0Spec> {
        RieW::new(self, 0)
    }
    #[doc = "Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&mut self) -> CmpeW<Lvdcr0Spec> {
        CmpeW::new(self, 2)
    }
    #[doc = "Bit 6 - Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<Lvdcr0Spec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&mut self) -> RnW<Lvdcr0Spec> {
        RnW::new(self, 7)
    }
}
#[doc = "Voltage Monitor %s Circuit Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvdcr0Spec;
impl crate::RegisterSpec for Lvdcr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdcr0::R`](R) reader structure"]
impl crate::Readable for Lvdcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`lvdcr0::W`](W) writer structure"]
impl crate::Writable for Lvdcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVD%sCR0 to value 0x80"]
impl crate::Resettable for Lvdcr0Spec {
    const RESET_VALUE: u8 = 0x80;
}

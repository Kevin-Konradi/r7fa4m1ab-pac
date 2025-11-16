#[doc = "Register `VBTWCTLR` reader"]
pub type R = crate::R<VbtwctlrSpec>;
#[doc = "Register `VBTWCTLR` writer"]
pub type W = crate::W<VbtwctlrSpec>;
#[doc = "VBATT wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vwen {
    #[doc = "0: Disable Wakeup function"]
    _0 = 0,
    #[doc = "1: Enable Wakeup function"]
    _1 = 1,
}
impl From<Vwen> for bool {
    #[inline(always)]
    fn from(variant: Vwen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VWEN` reader - VBATT wakeup enable"]
pub type VwenR = crate::BitReader<Vwen>;
impl VwenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vwen {
        match self.bits {
            false => Vwen::_0,
            true => Vwen::_1,
        }
    }
    #[doc = "Disable Wakeup function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vwen::_0
    }
    #[doc = "Enable Wakeup function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vwen::_1
    }
}
#[doc = "Field `VWEN` writer - VBATT wakeup enable"]
pub type VwenW<'a, REG> = crate::BitWriter<'a, REG, Vwen>;
impl<'a, REG> VwenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Wakeup function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vwen::_0)
    }
    #[doc = "Enable Wakeup function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vwen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    pub fn vwen(&self) -> VwenR {
        VwenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWCTLR")
            .field("vwen", &self.vwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    pub fn vwen(&mut self) -> VwenW<VbtwctlrSpec> {
        VwenW::new(self, 0)
    }
}
#[doc = "VBATT Wakeup function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtwctlrSpec;
impl crate::RegisterSpec for VbtwctlrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwctlr::R`](R) reader structure"]
impl crate::Readable for VbtwctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwctlr::W`](W) writer structure"]
impl crate::Writable for VbtwctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWCTLR to value 0"]
impl crate::Resettable for VbtwctlrSpec {}

#[doc = "Register `VBTCMPCR` reader"]
pub type R = crate::R<VbtcmpcrSpec>;
#[doc = "Register `VBTCMPCR` writer"]
pub type W = crate::W<VbtcmpcrSpec>;
#[doc = "VBATT pin low voltage detect circuit output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtcmpe {
    #[doc = "0: VBATT pin low voltage detect circuit output disabled"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detect circuit output enabled"]
    _1 = 1,
}
impl From<Vbtcmpe> for bool {
    #[inline(always)]
    fn from(variant: Vbtcmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTCMPE` reader - VBATT pin low voltage detect circuit output enable"]
pub type VbtcmpeR = crate::BitReader<Vbtcmpe>;
impl VbtcmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtcmpe {
        match self.bits {
            false => Vbtcmpe::_0,
            true => Vbtcmpe::_1,
        }
    }
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtcmpe::_0
    }
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtcmpe::_1
    }
}
#[doc = "Field `VBTCMPE` writer - VBATT pin low voltage detect circuit output enable"]
pub type VbtcmpeW<'a, REG> = crate::BitWriter<'a, REG, Vbtcmpe>;
impl<'a, REG> VbtcmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtcmpe::_0)
    }
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtcmpe::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub fn vbtcmpe(&self) -> VbtcmpeR {
        VbtcmpeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTCMPCR")
            .field("vbtcmpe", &self.vbtcmpe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub fn vbtcmpe(&mut self) -> VbtcmpeW<VbtcmpcrSpec> {
        VbtcmpeW::new(self, 0)
    }
}
#[doc = "VBATT Comparator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtcmpcrSpec;
impl crate::RegisterSpec for VbtcmpcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcmpcr::R`](R) reader structure"]
impl crate::Readable for VbtcmpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtcmpcr::W`](W) writer structure"]
impl crate::Writable for VbtcmpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTCMPCR to value 0"]
impl crate::Resettable for VbtcmpcrSpec {}

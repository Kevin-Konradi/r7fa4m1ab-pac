#[doc = "Register `DMCNT` reader"]
pub type R = crate::R<DmcntSpec>;
#[doc = "Register `DMCNT` writer"]
pub type W = crate::W<DmcntSpec>;
#[doc = "DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Dte> for bool {
    #[inline(always)]
    fn from(variant: Dte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE` reader - DMA Transfer Enable\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DteR = crate::BitReader<Dte>;
impl DteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte {
        match self.bits {
            false => Dte::_0,
            true => Dte::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dte::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dte::_1
    }
}
#[doc = "Field `DTE` writer - DMA Transfer Enable"]
pub type DteW<'a, REG> = crate::BitWriter<'a, REG, Dte>;
impl<'a, REG> DteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dte::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dte::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DteR {
        DteR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMCNT").finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DteW<DmcntSpec> {
        DteW::new(self, 0)
    }
}
#[doc = "DMA Transfer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmcntSpec;
impl crate::RegisterSpec for DmcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmcnt::R`](R) reader structure"]
impl crate::Readable for DmcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dmcnt::W`](W) writer structure"]
impl crate::Writable for DmcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMCNT to value 0"]
impl crate::Resettable for DmcntSpec {}

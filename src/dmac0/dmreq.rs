#[doc = "Register `DMREQ` reader"]
pub type R = crate::R<DmreqSpec>;
#[doc = "Register `DMREQ` writer"]
pub type W = crate::W<DmreqSpec>;
#[doc = "DMA Software Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swreq {
    #[doc = "0: DMA transfer is not requested."]
    _0 = 0,
    #[doc = "1: DMA transfer is requested."]
    _1 = 1,
}
impl From<Swreq> for bool {
    #[inline(always)]
    fn from(variant: Swreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - DMA Software Start\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SwreqR = crate::BitReader<Swreq>;
impl SwreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swreq {
        match self.bits {
            false => Swreq::_0,
            true => Swreq::_1,
        }
    }
    #[doc = "DMA transfer is not requested."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swreq::_0
    }
    #[doc = "DMA transfer is requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swreq::_1
    }
}
#[doc = "Field `SWREQ` writer - DMA Software Start"]
pub type SwreqW<'a, REG> = crate::BitWriter<'a, REG, Swreq>;
impl<'a, REG> SwreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfer is not requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swreq::_0)
    }
    #[doc = "DMA transfer is requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swreq::_1)
    }
}
#[doc = "DMA Software Start Bit Auto Clear Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrs {
    #[doc = "0: SWREQ bit is cleared after DMA transfer is started by software."]
    _0 = 0,
    #[doc = "1: SWREQ bit is not cleared after DMA transfer is started by software."]
    _1 = 1,
}
impl From<Clrs> for bool {
    #[inline(always)]
    fn from(variant: Clrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRS` reader - DMA Software Start Bit Auto Clear Select"]
pub type ClrsR = crate::BitReader<Clrs>;
impl ClrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clrs {
        match self.bits {
            false => Clrs::_0,
            true => Clrs::_1,
        }
    }
    #[doc = "SWREQ bit is cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clrs::_0
    }
    #[doc = "SWREQ bit is not cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clrs::_1
    }
}
#[doc = "Field `CLRS` writer - DMA Software Start Bit Auto Clear Select"]
pub type ClrsW<'a, REG> = crate::BitWriter<'a, REG, Clrs>;
impl<'a, REG> ClrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SWREQ bit is cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clrs::_0)
    }
    #[doc = "SWREQ bit is not cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrs::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Software Start"]
    #[inline(always)]
    pub fn swreq(&self) -> SwreqR {
        SwreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    pub fn clrs(&self) -> ClrsR {
        ClrsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMREQ").field("clrs", &self.clrs()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA Software Start"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<DmreqSpec> {
        SwreqW::new(self, 0)
    }
    #[doc = "Bit 4 - DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    pub fn clrs(&mut self) -> ClrsW<DmreqSpec> {
        ClrsW::new(self, 4)
    }
}
#[doc = "DMA Software Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmreqSpec;
impl crate::RegisterSpec for DmreqSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmreq::R`](R) reader structure"]
impl crate::Readable for DmreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dmreq::W`](W) writer structure"]
impl crate::Writable for DmreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMREQ to value 0"]
impl crate::Resettable for DmreqSpec {}

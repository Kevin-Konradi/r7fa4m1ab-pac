#[doc = "Register `ADGSPCR` reader"]
pub type R = crate::R<AdgspcrSpec>;
#[doc = "Register `ADGSPCR` writer"]
pub type W = crate::W<AdgspcrSpec>;
#[doc = "Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgs {
    #[doc = "0: Operation is without group A priority control"]
    _0 = 0,
    #[doc = "1: Operation is with group A priority control"]
    _1 = 1,
}
impl From<Pgs> for bool {
    #[inline(always)]
    fn from(variant: Pgs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGS` reader - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
pub type PgsR = crate::BitReader<Pgs>;
impl PgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgs {
        match self.bits {
            false => Pgs::_0,
            true => Pgs::_1,
        }
    }
    #[doc = "Operation is without group A priority control"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pgs::_0
    }
    #[doc = "Operation is with group A priority control"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pgs::_1
    }
}
#[doc = "Field `PGS` writer - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
pub type PgsW<'a, REG> = crate::BitWriter<'a, REG, Pgs>;
impl<'a, REG> PgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation is without group A priority control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgs::_0)
    }
    #[doc = "Operation is with group A priority control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgs::_1)
    }
}
#[doc = "Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbrscn {
    #[doc = "0: Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    _0 = 0,
    #[doc = "1: Scanning for group B is restarted after having been discontinued due to group A priority control."]
    _1 = 1,
}
impl From<Gbrscn> for bool {
    #[inline(always)]
    fn from(variant: Gbrscn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GBRSCN` reader - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
pub type GbrscnR = crate::BitReader<Gbrscn>;
impl GbrscnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gbrscn {
        match self.bits {
            false => Gbrscn::_0,
            true => Gbrscn::_1,
        }
    }
    #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbrscn::_0
    }
    #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbrscn::_1
    }
}
#[doc = "Field `GBRSCN` writer - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
pub type GbrscnW<'a, REG> = crate::BitWriter<'a, REG, Gbrscn>;
impl<'a, REG> GbrscnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrscn::_0)
    }
    #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrscn::_1)
    }
}
#[doc = "Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbrp {
    #[doc = "0: Single scan for group B is not continuously activated."]
    _0 = 0,
    #[doc = "1: Single scan for group B is continuously activated."]
    _1 = 1,
}
impl From<Gbrp> for bool {
    #[inline(always)]
    fn from(variant: Gbrp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GBRP` reader - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
pub type GbrpR = crate::BitReader<Gbrp>;
impl GbrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gbrp {
        match self.bits {
            false => Gbrp::_0,
            true => Gbrp::_1,
        }
    }
    #[doc = "Single scan for group B is not continuously activated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbrp::_0
    }
    #[doc = "Single scan for group B is continuously activated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbrp::_1
    }
}
#[doc = "Field `GBRP` writer - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
pub type GbrpW<'a, REG> = crate::BitWriter<'a, REG, Gbrp>;
impl<'a, REG> GbrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single scan for group B is not continuously activated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrp::_0)
    }
    #[doc = "Single scan for group B is continuously activated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbrp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    pub fn pgs(&self) -> PgsR {
        PgsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    pub fn gbrscn(&self) -> GbrscnR {
        GbrscnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    pub fn gbrp(&self) -> GbrpR {
        GbrpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADGSPCR")
            .field("gbrp", &self.gbrp())
            .field("gbrscn", &self.gbrscn())
            .field("pgs", &self.pgs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    pub fn pgs(&mut self) -> PgsW<AdgspcrSpec> {
        PgsW::new(self, 0)
    }
    #[doc = "Bit 1 - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    pub fn gbrscn(&mut self) -> GbrscnW<AdgspcrSpec> {
        GbrscnW::new(self, 1)
    }
    #[doc = "Bit 15 - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    pub fn gbrp(&mut self) -> GbrpW<AdgspcrSpec> {
        GbrpW::new(self, 15)
    }
}
#[doc = "A/D Group Scan Priority Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adgspcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdgspcrSpec;
impl crate::RegisterSpec for AdgspcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adgspcr::R`](R) reader structure"]
impl crate::Readable for AdgspcrSpec {}
#[doc = "`write(|w| ..)` method takes [`adgspcr::W`](W) writer structure"]
impl crate::Writable for AdgspcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADGSPCR to value 0"]
impl crate::Resettable for AdgspcrSpec {}

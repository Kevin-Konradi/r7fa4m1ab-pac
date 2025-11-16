#[doc = "Register `VBTSR` reader"]
pub type R = crate::R<VbtsrSpec>;
#[doc = "Register `VBTSR` writer"]
pub type W = crate::W<VbtsrSpec>;
#[doc = "VBAT_R Reset Detect Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtrdf {
    #[doc = "0: VBATT_R voltage power-on reset not detected"]
    _0 = 0,
    #[doc = "1: VBATT_R selected voltage power-on reset detected."]
    _1 = 1,
}
impl From<Vbtrdf> for bool {
    #[inline(always)]
    fn from(variant: Vbtrdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTRDF` reader - VBAT_R Reset Detect Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type VbtrdfR = crate::BitReader<Vbtrdf>;
impl VbtrdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtrdf {
        match self.bits {
            false => Vbtrdf::_0,
            true => Vbtrdf::_1,
        }
    }
    #[doc = "VBATT_R voltage power-on reset not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtrdf::_0
    }
    #[doc = "VBATT_R selected voltage power-on reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtrdf::_1
    }
}
#[doc = "Field `VBTRDF` writer - VBAT_R Reset Detect Flag"]
pub type VbtrdfW<'a, REG> = crate::BitWriter0C<'a, REG, Vbtrdf>;
impl<'a, REG> VbtrdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT_R voltage power-on reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtrdf::_0)
    }
    #[doc = "VBATT_R selected voltage power-on reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtrdf::_1)
    }
}
#[doc = "VBATT Battery Low voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtbldf {
    #[doc = "0: VBATT pin low voltage not detected"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detected."]
    _1 = 1,
}
impl From<Vbtbldf> for bool {
    #[inline(always)]
    fn from(variant: Vbtbldf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTBLDF` reader - VBATT Battery Low voltage Detect Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type VbtbldfR = crate::BitReader<Vbtbldf>;
impl VbtbldfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtbldf {
        match self.bits {
            false => Vbtbldf::_0,
            true => Vbtbldf::_1,
        }
    }
    #[doc = "VBATT pin low voltage not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtbldf::_0
    }
    #[doc = "VBATT pin low voltage detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtbldf::_1
    }
}
#[doc = "Field `VBTBLDF` writer - VBATT Battery Low voltage Detect Flag"]
pub type VbtbldfW<'a, REG> = crate::BitWriter0C<'a, REG, Vbtbldf>;
impl<'a, REG> VbtbldfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT pin low voltage not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtbldf::_0)
    }
    #[doc = "VBATT pin low voltage detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtbldf::_1)
    }
}
#[doc = "VBATT_R Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtrvld {
    #[doc = "0: VBATT_R area not valid"]
    _0 = 0,
    #[doc = "1: VBATT_R area valid"]
    _1 = 1,
}
impl From<Vbtrvld> for bool {
    #[inline(always)]
    fn from(variant: Vbtrvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTRVLD` reader - VBATT_R Valid"]
pub type VbtrvldR = crate::BitReader<Vbtrvld>;
impl VbtrvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtrvld {
        match self.bits {
            false => Vbtrvld::_0,
            true => Vbtrvld::_1,
        }
    }
    #[doc = "VBATT_R area not valid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtrvld::_0
    }
    #[doc = "VBATT_R area valid"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtrvld::_1
    }
}
impl R {
    #[doc = "Bit 0 - VBAT_R Reset Detect Flag"]
    #[inline(always)]
    pub fn vbtrdf(&self) -> VbtrdfR {
        VbtrdfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    pub fn vbtbldf(&self) -> VbtbldfR {
        VbtbldfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT_R Valid"]
    #[inline(always)]
    pub fn vbtrvld(&self) -> VbtrvldR {
        VbtrvldR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTSR")
            .field("vbtrvld", &self.vbtrvld())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBAT_R Reset Detect Flag"]
    #[inline(always)]
    pub fn vbtrdf(&mut self) -> VbtrdfW<VbtsrSpec> {
        VbtrdfW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    pub fn vbtbldf(&mut self) -> VbtbldfW<VbtsrSpec> {
        VbtbldfW::new(self, 1)
    }
}
#[doc = "VBATT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtsrSpec;
impl crate::RegisterSpec for VbtsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtsr::R`](R) reader structure"]
impl crate::Readable for VbtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtsr::W`](W) writer structure"]
impl crate::Writable for VbtsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets VBTSR to value 0x01"]
impl crate::Resettable for VbtsrSpec {
    const RESET_VALUE: u8 = 0x01;
}

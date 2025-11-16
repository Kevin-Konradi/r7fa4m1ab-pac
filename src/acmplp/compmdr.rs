#[doc = "Register `COMPMDR` reader"]
pub type R = crate::R<CompmdrSpec>;
#[doc = "Register `COMPMDR` writer"]
pub type W = crate::W<CompmdrSpec>;
#[doc = "ACMPLP0 Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0enb {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0enb> for bool {
    #[inline(always)]
    fn from(variant: C0enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0ENB` reader - ACMPLP0 Operation Enable"]
pub type C0enbR = crate::BitReader<C0enb>;
impl C0enbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0enb {
        match self.bits {
            false => C0enb::_0,
            true => C0enb::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0enb::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0enb::_1
    }
}
#[doc = "Field `C0ENB` writer - ACMPLP0 Operation Enable"]
pub type C0enbW<'a, REG> = crate::BitWriter<'a, REG, C0enb>;
impl<'a, REG> C0enbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0enb::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0enb::_1)
    }
}
#[doc = "ACMPLP0 Window Function Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0wde {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0wde> for bool {
    #[inline(always)]
    fn from(variant: C0wde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0WDE` reader - ACMPLP0 Window Function Mode Enable"]
pub type C0wdeR = crate::BitReader<C0wde>;
impl C0wdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0wde {
        match self.bits {
            false => C0wde::_0,
            true => C0wde::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0wde::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0wde::_1
    }
}
#[doc = "Field `C0WDE` writer - ACMPLP0 Window Function Mode Enable"]
pub type C0wdeW<'a, REG> = crate::BitWriter<'a, REG, C0wde>;
impl<'a, REG> C0wdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0wde::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0wde::_1)
    }
}
#[doc = "ACMPLP0 Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0vrf {
    #[doc = "0: IVREF0"]
    _0 = 0,
    #[doc = "1: internal reference voltage (Vref)"]
    _1 = 1,
}
impl From<C0vrf> for bool {
    #[inline(always)]
    fn from(variant: C0vrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0VRF` reader - ACMPLP0 Reference Voltage Selection"]
pub type C0vrfR = crate::BitReader<C0vrf>;
impl C0vrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0vrf {
        match self.bits {
            false => C0vrf::_0,
            true => C0vrf::_1,
        }
    }
    #[doc = "IVREF0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0vrf::_0
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0vrf::_1
    }
}
#[doc = "Field `C0VRF` writer - ACMPLP0 Reference Voltage Selection"]
pub type C0vrfW<'a, REG> = crate::BitWriter<'a, REG, C0vrf>;
impl<'a, REG> C0vrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IVREF0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0vrf::_0)
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0vrf::_1)
    }
}
#[doc = "ACMPLP0 Monitor Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0mon {
    #[doc = "0: CMPIN0 < CMPREF0, CMPIN0 < internal reference voltage, or ACMPLP0 operation disabled.(When the window function is disabled)/CMPIN0 < VRFL, CMPIN0 > VRFH, or ACMPLP0 operation disabled.(When the window function is enabled)"]
    _0 = 0,
    #[doc = "1: CMPIN0 > CMPREF0, or CMPIN0 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN0 < VRFH.(When the window function is enabled)"]
    _1 = 1,
}
impl From<C0mon> for bool {
    #[inline(always)]
    fn from(variant: C0mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0MON` reader - ACMPLP0 Monitor Flag"]
pub type C0monR = crate::BitReader<C0mon>;
impl C0monR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0mon {
        match self.bits {
            false => C0mon::_0,
            true => C0mon::_1,
        }
    }
    #[doc = "CMPIN0 < CMPREF0, CMPIN0 < internal reference voltage, or ACMPLP0 operation disabled.(When the window function is disabled)/CMPIN0 < VRFL, CMPIN0 > VRFH, or ACMPLP0 operation disabled.(When the window function is enabled)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0mon::_0
    }
    #[doc = "CMPIN0 > CMPREF0, or CMPIN0 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN0 < VRFH.(When the window function is enabled)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0mon::_1
    }
}
#[doc = "ACMPLP1 Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1enb {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1enb> for bool {
    #[inline(always)]
    fn from(variant: C1enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1ENB` reader - ACMPLP1 Operation Enable"]
pub type C1enbR = crate::BitReader<C1enb>;
impl C1enbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1enb {
        match self.bits {
            false => C1enb::_0,
            true => C1enb::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1enb::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1enb::_1
    }
}
#[doc = "Field `C1ENB` writer - ACMPLP1 Operation Enable"]
pub type C1enbW<'a, REG> = crate::BitWriter<'a, REG, C1enb>;
impl<'a, REG> C1enbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1enb::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1enb::_1)
    }
}
#[doc = "ACMPLP1 Window Function Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1wde {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1wde> for bool {
    #[inline(always)]
    fn from(variant: C1wde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1WDE` reader - ACMPLP1 Window Function Mode Enable"]
pub type C1wdeR = crate::BitReader<C1wde>;
impl C1wdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1wde {
        match self.bits {
            false => C1wde::_0,
            true => C1wde::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1wde::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1wde::_1
    }
}
#[doc = "Field `C1WDE` writer - ACMPLP1 Window Function Mode Enable"]
pub type C1wdeW<'a, REG> = crate::BitWriter<'a, REG, C1wde>;
impl<'a, REG> C1wdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1wde::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1wde::_1)
    }
}
#[doc = "ACMPLP1 Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1vrf {
    #[doc = "0: IVREF0 or IVREF1"]
    _0 = 0,
    #[doc = "1: internal reference voltage (Vref)"]
    _1 = 1,
}
impl From<C1vrf> for bool {
    #[inline(always)]
    fn from(variant: C1vrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1VRF` reader - ACMPLP1 Reference Voltage Selection"]
pub type C1vrfR = crate::BitReader<C1vrf>;
impl C1vrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1vrf {
        match self.bits {
            false => C1vrf::_0,
            true => C1vrf::_1,
        }
    }
    #[doc = "IVREF0 or IVREF1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1vrf::_0
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1vrf::_1
    }
}
#[doc = "Field `C1VRF` writer - ACMPLP1 Reference Voltage Selection"]
pub type C1vrfW<'a, REG> = crate::BitWriter<'a, REG, C1vrf>;
impl<'a, REG> C1vrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IVREF0 or IVREF1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf::_0)
    }
    #[doc = "internal reference voltage (Vref)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf::_1)
    }
}
#[doc = "ACMPLP1 Monitor Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1mon {
    #[doc = "0: CMPIN1 < CMPREF1, CMPIN1 < internal reference voltage, or ACMPLP1 operation disabled.(When the window function is disabled)/CMPIN1 < VRFL, CMPIN1 > VRFH, or ACMPLP1 operation disabled.(When the window function is enabled)"]
    _0 = 0,
    #[doc = "1: CMPIN1 > CMPREF1, or CMPIN1 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN1 < VRFH.(When the window function is enabled)"]
    _1 = 1,
}
impl From<C1mon> for bool {
    #[inline(always)]
    fn from(variant: C1mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1MON` reader - ACMPLP1 Monitor Flag"]
pub type C1monR = crate::BitReader<C1mon>;
impl C1monR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1mon {
        match self.bits {
            false => C1mon::_0,
            true => C1mon::_1,
        }
    }
    #[doc = "CMPIN1 < CMPREF1, CMPIN1 < internal reference voltage, or ACMPLP1 operation disabled.(When the window function is disabled)/CMPIN1 < VRFL, CMPIN1 > VRFH, or ACMPLP1 operation disabled.(When the window function is enabled)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1mon::_0
    }
    #[doc = "CMPIN1 > CMPREF1, or CMPIN1 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN1 < VRFH.(When the window function is enabled)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1mon::_1
    }
}
impl R {
    #[doc = "Bit 0 - ACMPLP0 Operation Enable"]
    #[inline(always)]
    pub fn c0enb(&self) -> C0enbR {
        C0enbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACMPLP0 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c0wde(&self) -> C0wdeR {
        C0wdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMPLP0 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c0vrf(&self) -> C0vrfR {
        C0vrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACMPLP0 Monitor Flag"]
    #[inline(always)]
    pub fn c0mon(&self) -> C0monR {
        C0monR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACMPLP1 Operation Enable"]
    #[inline(always)]
    pub fn c1enb(&self) -> C1enbR {
        C1enbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMPLP1 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c1wde(&self) -> C1wdeR {
        C1wdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf(&self) -> C1vrfR {
        C1vrfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP1 Monitor Flag"]
    #[inline(always)]
    pub fn c1mon(&self) -> C1monR {
        C1monR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPMDR")
            .field("c1mon", &self.c1mon())
            .field("c1vrf", &self.c1vrf())
            .field("c1wde", &self.c1wde())
            .field("c1enb", &self.c1enb())
            .field("c0mon", &self.c0mon())
            .field("c0vrf", &self.c0vrf())
            .field("c0wde", &self.c0wde())
            .field("c0enb", &self.c0enb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ACMPLP0 Operation Enable"]
    #[inline(always)]
    pub fn c0enb(&mut self) -> C0enbW<CompmdrSpec> {
        C0enbW::new(self, 0)
    }
    #[doc = "Bit 1 - ACMPLP0 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c0wde(&mut self) -> C0wdeW<CompmdrSpec> {
        C0wdeW::new(self, 1)
    }
    #[doc = "Bit 2 - ACMPLP0 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c0vrf(&mut self) -> C0vrfW<CompmdrSpec> {
        C0vrfW::new(self, 2)
    }
    #[doc = "Bit 4 - ACMPLP1 Operation Enable"]
    #[inline(always)]
    pub fn c1enb(&mut self) -> C1enbW<CompmdrSpec> {
        C1enbW::new(self, 4)
    }
    #[doc = "Bit 5 - ACMPLP1 Window Function Mode Enable"]
    #[inline(always)]
    pub fn c1wde(&mut self) -> C1wdeW<CompmdrSpec> {
        C1wdeW::new(self, 5)
    }
    #[doc = "Bit 6 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf(&mut self) -> C1vrfW<CompmdrSpec> {
        C1vrfW::new(self, 6)
    }
}
#[doc = "ACMPLP Mode Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompmdrSpec;
impl crate::RegisterSpec for CompmdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compmdr::R`](R) reader structure"]
impl crate::Readable for CompmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`compmdr::W`](W) writer structure"]
impl crate::Writable for CompmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPMDR to value 0"]
impl crate::Resettable for CompmdrSpec {}

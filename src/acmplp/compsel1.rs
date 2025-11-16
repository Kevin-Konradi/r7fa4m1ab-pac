#[doc = "Register `COMPSEL1` reader"]
pub type R = crate::R<Compsel1Spec>;
#[doc = "Register `COMPSEL1` writer"]
pub type W = crate::W<Compsel1Spec>;
#[doc = "ACMPLP0 Reference Voltage(IVREF0) Selection*\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crvs20 {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPREF0 (P101)"]
    _001 = 1,
    #[doc = "2: DAC8 (ch0) output"]
    _010 = 2,
    #[doc = "4: CMPREF0 (P502)"]
    _100 = 4,
    #[doc = "3: settings prohibited."]
    Others = 3,
}
impl From<Crvs20> for u8 {
    #[inline(always)]
    fn from(variant: Crvs20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crvs20 {
    type Ux = u8;
}
impl crate::IsEnum for Crvs20 {}
#[doc = "Field `CRVS20` reader - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
pub type Crvs20R = crate::FieldReader<Crvs20>;
impl Crvs20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crvs20 {
        match self.bits {
            0 => Crvs20::_000,
            1 => Crvs20::_001,
            2 => Crvs20::_010,
            4 => Crvs20::_100,
            _ => Crvs20::Others,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Crvs20::_000
    }
    #[doc = "CMPREF0 (P101)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Crvs20::_001
    }
    #[doc = "DAC8 (ch0) output"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Crvs20::_010
    }
    #[doc = "CMPREF0 (P502)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Crvs20::_100
    }
    #[doc = "settings prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Crvs20::Others)
    }
}
#[doc = "Field `CRVS20` writer - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
pub type Crvs20W<'a, REG> = crate::FieldWriter<'a, REG, 3, Crvs20, crate::Safe>;
impl<'a, REG> Crvs20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs20::_000)
    }
    #[doc = "CMPREF0 (P101)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs20::_001)
    }
    #[doc = "DAC8 (ch0) output"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs20::_010)
    }
    #[doc = "CMPREF0 (P502)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs20::_100)
    }
    #[doc = "settings prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs20::Others)
    }
}
#[doc = "ACMPLP1 Reference Voltage(IVREF1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crvs64 {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPREF1 (P103)"]
    _001 = 1,
    #[doc = "2: DAC8 (ch1) output"]
    _010 = 2,
    #[doc = "4: CMPREF1 (P500)"]
    _100 = 4,
    #[doc = "3: settings prohibited."]
    Others = 3,
}
impl From<Crvs64> for u8 {
    #[inline(always)]
    fn from(variant: Crvs64) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crvs64 {
    type Ux = u8;
}
impl crate::IsEnum for Crvs64 {}
#[doc = "Field `CRVS64` reader - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type Crvs64R = crate::FieldReader<Crvs64>;
impl Crvs64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crvs64 {
        match self.bits {
            0 => Crvs64::_000,
            1 => Crvs64::_001,
            2 => Crvs64::_010,
            4 => Crvs64::_100,
            _ => Crvs64::Others,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Crvs64::_000
    }
    #[doc = "CMPREF1 (P103)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Crvs64::_001
    }
    #[doc = "DAC8 (ch1) output"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Crvs64::_010
    }
    #[doc = "CMPREF1 (P500)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Crvs64::_100
    }
    #[doc = "settings prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Crvs64::Others)
    }
}
#[doc = "Field `CRVS64` writer - ACMPLP1 Reference Voltage(IVREF1) Selection"]
pub type Crvs64W<'a, REG> = crate::FieldWriter<'a, REG, 3, Crvs64, crate::Safe>;
impl<'a, REG> Crvs64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs64::_000)
    }
    #[doc = "CMPREF1 (P103)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs64::_001)
    }
    #[doc = "DAC8 (ch1) output"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs64::_010)
    }
    #[doc = "CMPREF1 (P500)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs64::_100)
    }
    #[doc = "settings prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Crvs64::Others)
    }
}
#[doc = "ACMPLP1 Reference Voltage Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1vrf2 {
    #[doc = "0: IVREF0 selected"]
    _0 = 0,
    #[doc = "1: IVREF1 selected."]
    _1 = 1,
}
impl From<C1vrf2> for bool {
    #[inline(always)]
    fn from(variant: C1vrf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1VRF2` reader - ACMPLP1 Reference Voltage Selection"]
pub type C1vrf2R = crate::BitReader<C1vrf2>;
impl C1vrf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1vrf2 {
        match self.bits {
            false => C1vrf2::_0,
            true => C1vrf2::_1,
        }
    }
    #[doc = "IVREF0 selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1vrf2::_0
    }
    #[doc = "IVREF1 selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1vrf2::_1
    }
}
#[doc = "Field `C1VRF2` writer - ACMPLP1 Reference Voltage Selection"]
pub type C1vrf2W<'a, REG> = crate::BitWriter<'a, REG, C1vrf2>;
impl<'a, REG> C1vrf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IVREF0 selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf2::_0)
    }
    #[doc = "IVREF1 selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf2::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[inline(always)]
    pub fn crvs20(&self) -> Crvs20R {
        Crvs20R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    pub fn crvs64(&self) -> Crvs64R {
        Crvs64R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf2(&self) -> C1vrf2R {
        C1vrf2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPSEL1")
            .field("c1vrf2", &self.c1vrf2())
            .field("crvs64", &self.crvs64())
            .field("crvs20", &self.crvs20())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*"]
    #[inline(always)]
    pub fn crvs20(&mut self) -> Crvs20W<Compsel1Spec> {
        Crvs20W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection"]
    #[inline(always)]
    pub fn crvs64(&mut self) -> Crvs64W<Compsel1Spec> {
        Crvs64W::new(self, 4)
    }
    #[doc = "Bit 7 - ACMPLP1 Reference Voltage Selection"]
    #[inline(always)]
    pub fn c1vrf2(&mut self) -> C1vrf2W<Compsel1Spec> {
        C1vrf2W::new(self, 7)
    }
}
#[doc = "Comparator Reference Voltage Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compsel1Spec;
impl crate::RegisterSpec for Compsel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compsel1::R`](R) reader structure"]
impl crate::Readable for Compsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`compsel1::W`](W) writer structure"]
impl crate::Writable for Compsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPSEL1 to value 0x91"]
impl crate::Resettable for Compsel1Spec {
    const RESET_VALUE: u8 = 0x91;
}

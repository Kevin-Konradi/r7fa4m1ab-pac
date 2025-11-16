#[doc = "Register `COMPSEL0` reader"]
pub type R = crate::R<Compsel0Spec>;
#[doc = "Register `COMPSEL0` writer"]
pub type W = crate::W<Compsel0Spec>;
#[doc = "ACMPLP0 Input(IVCMP0) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpsel20 {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN0 (P100)"]
    _001 = 1,
    #[doc = "4: CMPIN0 (P503)"]
    _100 = 4,
    #[doc = "2: settings prohibited"]
    Others = 2,
}
impl From<Cmpsel20> for u8 {
    #[inline(always)]
    fn from(variant: Cmpsel20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpsel20 {
    type Ux = u8;
}
impl crate::IsEnum for Cmpsel20 {}
#[doc = "Field `CMPSEL20` reader - ACMPLP0 Input(IVCMP0) Selection"]
pub type Cmpsel20R = crate::FieldReader<Cmpsel20>;
impl Cmpsel20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpsel20 {
        match self.bits {
            0 => Cmpsel20::_000,
            1 => Cmpsel20::_001,
            4 => Cmpsel20::_100,
            _ => Cmpsel20::Others,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cmpsel20::_000
    }
    #[doc = "CMPIN0 (P100)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cmpsel20::_001
    }
    #[doc = "CMPIN0 (P503)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cmpsel20::_100
    }
    #[doc = "settings prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmpsel20::Others)
    }
}
#[doc = "Field `CMPSEL20` writer - ACMPLP0 Input(IVCMP0) Selection"]
pub type Cmpsel20W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmpsel20, crate::Safe>;
impl<'a, REG> Cmpsel20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel20::_000)
    }
    #[doc = "CMPIN0 (P100)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel20::_001)
    }
    #[doc = "CMPIN0 (P503)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel20::_100)
    }
    #[doc = "settings prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel20::Others)
    }
}
#[doc = "ACMPLP1 Input (IVCMP1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpsel64 {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN1 (P102)"]
    _001 = 1,
    #[doc = "4: CMPIN1 (P501)"]
    _100 = 4,
    #[doc = "2: settings prohibited"]
    Others = 2,
}
impl From<Cmpsel64> for u8 {
    #[inline(always)]
    fn from(variant: Cmpsel64) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpsel64 {
    type Ux = u8;
}
impl crate::IsEnum for Cmpsel64 {}
#[doc = "Field `CMPSEL64` reader - ACMPLP1 Input (IVCMP1) Selection"]
pub type Cmpsel64R = crate::FieldReader<Cmpsel64>;
impl Cmpsel64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpsel64 {
        match self.bits {
            0 => Cmpsel64::_000,
            1 => Cmpsel64::_001,
            4 => Cmpsel64::_100,
            _ => Cmpsel64::Others,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cmpsel64::_000
    }
    #[doc = "CMPIN1 (P102)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cmpsel64::_001
    }
    #[doc = "CMPIN1 (P501)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cmpsel64::_100
    }
    #[doc = "settings prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmpsel64::Others)
    }
}
#[doc = "Field `CMPSEL64` writer - ACMPLP1 Input (IVCMP1) Selection"]
pub type Cmpsel64W<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmpsel64, crate::Safe>;
impl<'a, REG> Cmpsel64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel64::_000)
    }
    #[doc = "CMPIN1 (P102)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel64::_001)
    }
    #[doc = "CMPIN1 (P501)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel64::_100)
    }
    #[doc = "settings prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsel64::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    pub fn cmpsel20(&self) -> Cmpsel20R {
        Cmpsel20R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub fn cmpsel64(&self) -> Cmpsel64R {
        Cmpsel64R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPSEL0")
            .field("cmpsel64", &self.cmpsel64())
            .field("cmpsel20", &self.cmpsel20())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    pub fn cmpsel20(&mut self) -> Cmpsel20W<Compsel0Spec> {
        Cmpsel20W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub fn cmpsel64(&mut self) -> Cmpsel64W<Compsel0Spec> {
        Cmpsel64W::new(self, 4)
    }
}
#[doc = "Comparator Input Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compsel0Spec;
impl crate::RegisterSpec for Compsel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compsel0::R`](R) reader structure"]
impl crate::Readable for Compsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`compsel0::W`](W) writer structure"]
impl crate::Writable for Compsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPSEL0 to value 0x11"]
impl crate::Resettable for Compsel0Spec {
    const RESET_VALUE: u8 = 0x11;
}

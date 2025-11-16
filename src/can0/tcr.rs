#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "CAN Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tste {
    #[doc = "0: CAN test mode disabled"]
    _0 = 0,
    #[doc = "1: CAN test mode enabled"]
    _1 = 1,
}
impl From<Tste> for bool {
    #[inline(always)]
    fn from(variant: Tste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTE` reader - CAN Test Mode Enable"]
pub type TsteR = crate::BitReader<Tste>;
impl TsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tste {
        match self.bits {
            false => Tste::_0,
            true => Tste::_1,
        }
    }
    #[doc = "CAN test mode disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tste::_0
    }
    #[doc = "CAN test mode enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tste::_1
    }
}
#[doc = "Field `TSTE` writer - CAN Test Mode Enable"]
pub type TsteW<'a, REG> = crate::BitWriter<'a, REG, Tste>;
impl<'a, REG> TsteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN test mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tste::_0)
    }
    #[doc = "CAN test mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tste::_1)
    }
}
#[doc = "CAN Test Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstm {
    #[doc = "0: Other than CAN test mode"]
    _00 = 0,
    #[doc = "1: Listen-only mode"]
    _01 = 1,
    #[doc = "2: Self-test mode 0 (external loopback)"]
    _10 = 2,
    #[doc = "3: Self-test mode 1 (internal loopback)"]
    _11 = 3,
}
impl From<Tstm> for u8 {
    #[inline(always)]
    fn from(variant: Tstm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstm {
    type Ux = u8;
}
impl crate::IsEnum for Tstm {}
#[doc = "Field `TSTM` reader - CAN Test Mode Select"]
pub type TstmR = crate::FieldReader<Tstm>;
impl TstmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstm {
        match self.bits {
            0 => Tstm::_00,
            1 => Tstm::_01,
            2 => Tstm::_10,
            3 => Tstm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Other than CAN test mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tstm::_00
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tstm::_01
    }
    #[doc = "Self-test mode 0 (external loopback)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tstm::_10
    }
    #[doc = "Self-test mode 1 (internal loopback)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tstm::_11
    }
}
#[doc = "Field `TSTM` writer - CAN Test Mode Select"]
pub type TstmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tstm, crate::Safe>;
impl<'a, REG> TstmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Other than CAN test mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tstm::_00)
    }
    #[doc = "Listen-only mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tstm::_01)
    }
    #[doc = "Self-test mode 0 (external loopback)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tstm::_10)
    }
    #[doc = "Self-test mode 1 (internal loopback)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tstm::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Test Mode Enable"]
    #[inline(always)]
    pub fn tste(&self) -> TsteR {
        TsteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - CAN Test Mode Select"]
    #[inline(always)]
    pub fn tstm(&self) -> TstmR {
        TstmR::new((self.bits >> 1) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("tstm", &self.tstm())
            .field("tste", &self.tste())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CAN Test Mode Enable"]
    #[inline(always)]
    pub fn tste(&mut self) -> TsteW<TcrSpec> {
        TsteW::new(self, 0)
    }
    #[doc = "Bits 1:2 - CAN Test Mode Select"]
    #[inline(always)]
    pub fn tstm(&mut self) -> TstmW<TcrSpec> {
        TstmW::new(self, 1)
    }
}
#[doc = "Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}

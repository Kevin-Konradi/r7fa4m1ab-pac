#[doc = "Register `AGTMR2` reader"]
pub type R = crate::R<Agtmr2Spec>;
#[doc = "Register `AGTMR2` writer"]
pub type W = crate::W<Agtmr2Spec>;
#[doc = "AGTLCLK/AGTSCLK count source clock frequency division ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: 1/1"]
    _000 = 0,
    #[doc = "1: 1/2"]
    _001 = 1,
    #[doc = "2: 1/4"]
    _010 = 2,
    #[doc = "3: 1/8"]
    _011 = 3,
    #[doc = "4: 1/16"]
    _100 = 4,
    #[doc = "5: 1/32"]
    _101 = 5,
    #[doc = "6: 1/64"]
    _110 = 6,
    #[doc = "7: 1/128."]
    _111 = 7,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_000,
            1 => Cks::_001,
            2 => Cks::_010,
            3 => Cks::_011,
            4 => Cks::_100,
            5 => Cks::_101,
            6 => Cks::_110,
            7 => Cks::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cks::_000
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cks::_001
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cks::_010
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cks::_011
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cks::_100
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cks::_101
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cks::_110
    }
    #[doc = "1/128."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cks::_111
    }
}
#[doc = "Field `CKS` writer - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_000)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_001)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_010)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_011)
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_100)
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_101)
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_110)
    }
    #[doc = "1/128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_111)
    }
}
#[doc = "Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpm {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Low Power mode"]
    _1 = 1,
}
impl From<Lpm> for bool {
    #[inline(always)]
    fn from(variant: Lpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM` reader - Low Power Mode"]
pub type LpmR = crate::BitReader<Lpm>;
impl LpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpm {
        match self.bits {
            false => Lpm::_0,
            true => Lpm::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpm::_0
    }
    #[doc = "Low Power mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpm::_1
    }
}
#[doc = "Field `LPM` writer - Low Power Mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG, Lpm>;
impl<'a, REG> LpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm::_0)
    }
    #[doc = "Low Power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTMR2")
            .field("lpm", &self.lpm())
            .field("cks", &self.cks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - AGTLCLK/AGTSCLK count source clock frequency division ratio"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<Agtmr2Spec> {
        CksW::new(self, 0)
    }
    #[doc = "Bit 7 - Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LpmW<Agtmr2Spec> {
        LpmW::new(self, 7)
    }
}
#[doc = "AGT Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`agtmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agtmr2Spec;
impl crate::RegisterSpec for Agtmr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtmr2::R`](R) reader structure"]
impl crate::Readable for Agtmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`agtmr2::W`](W) writer structure"]
impl crate::Writable for Agtmr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTMR2 to value 0"]
impl crate::Resettable for Agtmr2Spec {}

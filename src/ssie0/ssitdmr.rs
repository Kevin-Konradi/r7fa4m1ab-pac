#[doc = "Register `SSITDMR` reader"]
pub type R = crate::R<SsitdmrSpec>;
#[doc = "Register `SSITDMR` writer"]
pub type W = crate::W<SsitdmrSpec>;
#[doc = "Audio Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Omod {
    #[doc = "0: I2S format"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Monaural format"]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<Omod> for u8 {
    #[inline(always)]
    fn from(variant: Omod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Omod {
    type Ux = u8;
}
impl crate::IsEnum for Omod {}
#[doc = "Field `OMOD` reader - Audio Format Select"]
pub type OmodR = crate::FieldReader<Omod>;
impl OmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Omod {
        match self.bits {
            0 => Omod::_00,
            1 => Omod::_01,
            2 => Omod::_10,
            3 => Omod::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S format"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Omod::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Omod::_01
    }
    #[doc = "Monaural format"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Omod::_10
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Omod::_11
    }
}
#[doc = "Field `OMOD` writer - Audio Format Select"]
pub type OmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Omod, crate::Safe>;
impl<'a, REG> OmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S format"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_01)
    }
    #[doc = "Monaural format"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Omod::_11)
    }
}
#[doc = "Whether to Enable LRCK/FS Continuation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrcont {
    #[doc = "0: Disables LRCK/FS continuation"]
    _0 = 0,
    #[doc = "1: Enables LRCK/FS continuation."]
    _1 = 1,
}
impl From<Lrcont> for bool {
    #[inline(always)]
    fn from(variant: Lrcont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRCONT` reader - Whether to Enable LRCK/FS Continuation"]
pub type LrcontR = crate::BitReader<Lrcont>;
impl LrcontR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrcont {
        match self.bits {
            false => Lrcont::_0,
            true => Lrcont::_1,
        }
    }
    #[doc = "Disables LRCK/FS continuation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrcont::_0
    }
    #[doc = "Enables LRCK/FS continuation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrcont::_1
    }
}
#[doc = "Field `LRCONT` writer - Whether to Enable LRCK/FS Continuation"]
pub type LrcontW<'a, REG> = crate::BitWriter<'a, REG, Lrcont>;
impl<'a, REG> LrcontW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables LRCK/FS continuation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrcont::_0)
    }
    #[doc = "Enables LRCK/FS continuation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrcont::_1)
    }
}
#[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bckastp {
    #[doc = "0: Always outputs BCK to the SSIBCK pin"]
    _0 = 0,
    #[doc = "1: Automatically controls output of BCK to the SSIBCK pin."]
    _1 = 1,
}
impl From<Bckastp> for bool {
    #[inline(always)]
    fn from(variant: Bckastp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCKASTP` reader - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
pub type BckastpR = crate::BitReader<Bckastp>;
impl BckastpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bckastp {
        match self.bits {
            false => Bckastp::_0,
            true => Bckastp::_1,
        }
    }
    #[doc = "Always outputs BCK to the SSIBCK pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bckastp::_0
    }
    #[doc = "Automatically controls output of BCK to the SSIBCK pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bckastp::_1
    }
}
#[doc = "Field `BCKASTP` writer - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
pub type BckastpW<'a, REG> = crate::BitWriter<'a, REG, Bckastp>;
impl<'a, REG> BckastpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always outputs BCK to the SSIBCK pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bckastp::_0)
    }
    #[doc = "Automatically controls output of BCK to the SSIBCK pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bckastp::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Audio Format Select"]
    #[inline(always)]
    pub fn omod(&self) -> OmodR {
        OmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub fn lrcont(&self) -> LrcontR {
        LrcontR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub fn bckastp(&self) -> BckastpR {
        BckastpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSITDMR")
            .field("bckastp", &self.bckastp())
            .field("lrcont", &self.lrcont())
            .field("omod", &self.omod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio Format Select"]
    #[inline(always)]
    pub fn omod(&mut self) -> OmodW<SsitdmrSpec> {
        OmodW::new(self, 0)
    }
    #[doc = "Bit 8 - Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub fn lrcont(&mut self) -> LrcontW<SsitdmrSpec> {
        LrcontW::new(self, 8)
    }
    #[doc = "Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub fn bckastp(&mut self) -> BckastpW<SsitdmrSpec> {
        BckastpW::new(self, 9)
    }
}
#[doc = "TDM Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssitdmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssitdmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsitdmrSpec;
impl crate::RegisterSpec for SsitdmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssitdmr::R`](R) reader structure"]
impl crate::Readable for SsitdmrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssitdmr::W`](W) writer structure"]
impl crate::Writable for SsitdmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSITDMR to value 0"]
impl crate::Resettable for SsitdmrSpec {}

#[doc = "Register `SOMCR` reader"]
pub type R = crate::R<SomcrSpec>;
#[doc = "Register `SOMCR` writer"]
pub type W = crate::W<SomcrSpec>;
#[doc = "Sub-Clock Oscillator Drive Capability Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sodrv {
    #[doc = "0: Normal mode"]
    _00 = 0,
    #[doc = "1: Low power mode 1"]
    _01 = 1,
    #[doc = "2: Low power mode 2"]
    _10 = 2,
    #[doc = "3: Low power mode 3."]
    _11 = 3,
}
impl From<Sodrv> for u8 {
    #[inline(always)]
    fn from(variant: Sodrv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sodrv {
    type Ux = u8;
}
impl crate::IsEnum for Sodrv {}
#[doc = "Field `SODRV` reader - Sub-Clock Oscillator Drive Capability Switching"]
pub type SodrvR = crate::FieldReader<Sodrv>;
impl SodrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sodrv {
        match self.bits {
            0 => Sodrv::_00,
            1 => Sodrv::_01,
            2 => Sodrv::_10,
            3 => Sodrv::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sodrv::_00
    }
    #[doc = "Low power mode 1"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sodrv::_01
    }
    #[doc = "Low power mode 2"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sodrv::_10
    }
    #[doc = "Low power mode 3."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sodrv::_11
    }
}
#[doc = "Field `SODRV` writer - Sub-Clock Oscillator Drive Capability Switching"]
pub type SodrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sodrv, crate::Safe>;
impl<'a, REG> SodrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_00)
    }
    #[doc = "Low power mode 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_01)
    }
    #[doc = "Low power mode 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_10)
    }
    #[doc = "Low power mode 3."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(&self) -> SodrvR {
        SodrvR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOMCR")
            .field("sodrv", &self.sodrv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(&mut self) -> SodrvW<SomcrSpec> {
        SodrvW::new(self, 0)
    }
}
#[doc = "Sub Clock Oscillator Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`somcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SomcrSpec;
impl crate::RegisterSpec for SomcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`somcr::R`](R) reader structure"]
impl crate::Readable for SomcrSpec {}
#[doc = "`write(|w| ..)` method takes [`somcr::W`](W) writer structure"]
impl crate::Writable for SomcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOMCR to value 0"]
impl crate::Resettable for SomcrSpec {}

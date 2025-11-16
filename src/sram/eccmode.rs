#[doc = "Register `ECCMODE` reader"]
pub type R = crate::R<EccmodeSpec>;
#[doc = "Register `ECCMODE` writer"]
pub type W = crate::W<EccmodeSpec>;
#[doc = "ECC Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eccmod {
    #[doc = "0: Disable ECC function"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Enable ECC function without error checking"]
    _10 = 2,
    #[doc = "3: Enable ECC function with error checking"]
    _11 = 3,
}
impl From<Eccmod> for u8 {
    #[inline(always)]
    fn from(variant: Eccmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eccmod {
    type Ux = u8;
}
impl crate::IsEnum for Eccmod {}
#[doc = "Field `ECCMOD` reader - ECC Operating Mode Select"]
pub type EccmodR = crate::FieldReader<Eccmod>;
impl EccmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccmod {
        match self.bits {
            0 => Eccmod::_00,
            1 => Eccmod::_01,
            2 => Eccmod::_10,
            3 => Eccmod::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable ECC function"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Eccmod::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Eccmod::_01
    }
    #[doc = "Enable ECC function without error checking"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Eccmod::_10
    }
    #[doc = "Enable ECC function with error checking"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Eccmod::_11
    }
}
#[doc = "Field `ECCMOD` writer - ECC Operating Mode Select"]
pub type EccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eccmod, crate::Safe>;
impl<'a, REG> EccmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable ECC function"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_01)
    }
    #[doc = "Enable ECC function without error checking"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_10)
    }
    #[doc = "Enable ECC function with error checking"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Eccmod::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(&self) -> EccmodR {
        EccmodR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCMODE")
            .field("eccmod", &self.eccmod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(&mut self) -> EccmodW<EccmodeSpec> {
        EccmodW::new(self, 0)
    }
}
#[doc = "ECC Operating Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccmodeSpec;
impl crate::RegisterSpec for EccmodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eccmode::R`](R) reader structure"]
impl crate::Readable for EccmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`eccmode::W`](W) writer structure"]
impl crate::Writable for EccmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCMODE to value 0"]
impl crate::Resettable for EccmodeSpec {}

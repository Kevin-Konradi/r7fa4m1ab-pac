#[doc = "Register `ECCPRCR` reader"]
pub type R = crate::R<EccprcrSpec>;
#[doc = "Register `ECCPRCR` writer"]
pub type W = crate::W<EccprcrSpec>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccprcr {
    #[doc = "0: Disable writes to the protected registers"]
    _0 = 0,
    #[doc = "1: Enable writes to the protected registers"]
    _1 = 1,
}
impl From<Eccprcr> for bool {
    #[inline(always)]
    fn from(variant: Eccprcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCPRCR` reader - Register Write Control"]
pub type EccprcrR = crate::BitReader<Eccprcr>;
impl EccprcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccprcr {
        match self.bits {
            false => Eccprcr::_0,
            true => Eccprcr::_1,
        }
    }
    #[doc = "Disable writes to the protected registers"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eccprcr::_0
    }
    #[doc = "Enable writes to the protected registers"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eccprcr::_1
    }
}
#[doc = "Field `ECCPRCR` writer - Register Write Control"]
pub type EccprcrW<'a, REG> = crate::BitWriter<'a, REG, Eccprcr>;
impl<'a, REG> EccprcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes to the protected registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eccprcr::_0)
    }
    #[doc = "Enable writes to the protected registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eccprcr::_1)
    }
}
#[doc = "Write Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kw {
    #[doc = "120: Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    _1111000 = 120,
    #[doc = "0: Writing to the ECCRAMPRCR bit is invalid."]
    Others = 0,
}
impl From<Kw> for u8 {
    #[inline(always)]
    fn from(variant: Kw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Kw {
    type Ux = u8;
}
impl crate::IsEnum for Kw {}
#[doc = "Field `KW` writer - Write Key Code"]
pub type KwW<'a, REG> = crate::FieldWriter<'a, REG, 7, Kw, crate::Safe>;
impl<'a, REG> KwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b."]
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(Kw::_1111000)
    }
    #[doc = "Writing to the ECCRAMPRCR bit is invalid."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Kw::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn eccprcr(&self) -> EccprcrR {
        EccprcrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCPRCR")
            .field("eccprcr", &self.eccprcr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn eccprcr(&mut self) -> EccprcrW<EccprcrSpec> {
        EccprcrW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    pub fn kw(&mut self) -> KwW<EccprcrSpec> {
        KwW::new(self, 1)
    }
}
#[doc = "ECC Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccprcrSpec;
impl crate::RegisterSpec for EccprcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eccprcr::R`](R) reader structure"]
impl crate::Readable for EccprcrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccprcr::W`](W) writer structure"]
impl crate::Writable for EccprcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCPRCR to value 0"]
impl crate::Resettable for EccprcrSpec {}

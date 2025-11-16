#[doc = "Register `PCNTR4` reader"]
pub type R = crate::R<Pcntr4Spec>;
#[doc = "Register `PCNTR4` writer"]
pub type W = crate::W<Pcntr4Spec>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Eosr {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<Eosr> for u16 {
    #[inline(always)]
    fn from(variant: Eosr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eosr {
    type Ux = u16;
}
impl crate::IsEnum for Eosr {}
#[doc = "Field `EOSR` reader - Pmn Event Output Set"]
pub type EosrR = crate::FieldReader<Eosr>;
impl EosrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eosr> {
        match self.bits {
            0 => Some(Eosr::_0),
            1 => Some(Eosr::_1),
            _ => None,
        }
    }
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr::_0
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr::_1
    }
}
#[doc = "Field `EOSR` writer - Pmn Event Output Set"]
pub type EosrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Eosr>;
impl<'a, REG> EosrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr::_1)
    }
}
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Eorr {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Eorr> for u16 {
    #[inline(always)]
    fn from(variant: Eorr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eorr {
    type Ux = u16;
}
impl crate::IsEnum for Eorr {}
#[doc = "Field `EORR` reader - Pmn Event Output Reset"]
pub type EorrR = crate::FieldReader<Eorr>;
impl EorrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eorr> {
        match self.bits {
            0 => Some(Eorr::_0),
            1 => Some(Eorr::_1),
            _ => None,
        }
    }
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr::_0
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr::_1
    }
}
#[doc = "Field `EORR` writer - Pmn Event Output Reset"]
pub type EorrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Eorr>;
impl<'a, REG> EorrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&self) -> EosrR {
        EosrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&self) -> EorrR {
        EorrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR4")
            .field("eorr", &self.eorr())
            .field("eosr", &self.eosr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&mut self) -> EosrW<Pcntr4Spec> {
        EosrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&mut self) -> EorrW<Pcntr4Spec> {
        EorrW::new(self, 16)
    }
}
#[doc = "Port Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcntr4Spec;
impl crate::RegisterSpec for Pcntr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntr4::R`](R) reader structure"]
impl crate::Readable for Pcntr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pcntr4::W`](W) writer structure"]
impl crate::Writable for Pcntr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNTR4 to value 0"]
impl crate::Resettable for Pcntr4Spec {}

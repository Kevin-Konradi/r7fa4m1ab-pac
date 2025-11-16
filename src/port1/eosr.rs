#[doc = "Register `EOSR` reader"]
pub type R = crate::R<EosrSpec>;
#[doc = "Register `EOSR` writer"]
pub type W = crate::W<EosrSpec>;
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
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&self) -> EosrR {
        EosrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EOSR").field("eosr", &self.eosr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&mut self) -> EosrW<EosrSpec> {
        EosrW::new(self, 0)
    }
}
#[doc = "Event output reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`eosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EosrSpec;
impl crate::RegisterSpec for EosrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eosr::R`](R) reader structure"]
impl crate::Readable for EosrSpec {}
#[doc = "`write(|w| ..)` method takes [`eosr::W`](W) writer structure"]
impl crate::Writable for EosrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EOSR to value 0"]
impl crate::Resettable for EosrSpec {}

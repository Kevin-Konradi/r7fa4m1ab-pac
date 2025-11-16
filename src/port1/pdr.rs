#[doc = "Register `PDR` reader"]
pub type R = crate::R<PdrSpec>;
#[doc = "Register `PDR` writer"]
pub type W = crate::W<PdrSpec>;
#[doc = "Pmn Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pdr {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)."]
    _1 = 1,
}
impl From<Pdr> for u16 {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdr {
    type Ux = u16;
}
impl crate::IsEnum for Pdr {}
#[doc = "Field `PDR` reader - Pmn Direction"]
pub type PdrR = crate::FieldReader<Pdr>;
impl PdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pdr> {
        match self.bits {
            0 => Some(Pdr::_0),
            1 => Some(Pdr::_1),
            _ => None,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr::_0
    }
    #[doc = "Output (functions as an output pin)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr::_1
    }
}
#[doc = "Field `PDR` writer - Pmn Direction"]
pub type PdrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Pdr>;
impl<'a, REG> PdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_0)
    }
    #[doc = "Output (functions as an output pin)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDR").field("pdr", &self.pdr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<PdrSpec> {
        PdrW::new(self, 0)
    }
}
#[doc = "Data direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdrSpec;
impl crate::RegisterSpec for PdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdr::R`](R) reader structure"]
impl crate::Readable for PdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pdr::W`](W) writer structure"]
impl crate::Writable for PdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PdrSpec {}

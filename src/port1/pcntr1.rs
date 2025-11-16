#[doc = "Register `PCNTR1` reader"]
pub type R = crate::R<Pcntr1Spec>;
#[doc = "Register `PCNTR1` writer"]
pub type W = crate::W<Pcntr1Spec>;
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
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Podr {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<Podr> for u16 {
    #[inline(always)]
    fn from(variant: Podr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Podr {
    type Ux = u16;
}
impl crate::IsEnum for Podr {}
#[doc = "Field `PODR` reader - Pmn Output Data"]
pub type PodrR = crate::FieldReader<Podr>;
impl PodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Podr> {
        match self.bits {
            0 => Some(Podr::_0),
            1 => Some(Podr::_1),
            _ => None,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr::_0
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr::_1
    }
}
#[doc = "Field `PODR` writer - Pmn Output Data"]
pub type PodrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Podr>;
impl<'a, REG> PodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR1")
            .field("podr", &self.podr())
            .field("pdr", &self.pdr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Direction"]
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<Pcntr1Spec> {
        PdrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<Pcntr1Spec> {
        PodrW::new(self, 16)
    }
}
#[doc = "Port Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcntr1Spec;
impl crate::RegisterSpec for Pcntr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntr1::R`](R) reader structure"]
impl crate::Readable for Pcntr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcntr1::W`](W) writer structure"]
impl crate::Writable for Pcntr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNTR1 to value 0"]
impl crate::Resettable for Pcntr1Spec {}

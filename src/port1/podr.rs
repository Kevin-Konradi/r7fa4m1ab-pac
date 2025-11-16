#[doc = "Register `PODR` reader"]
pub type R = crate::R<PodrSpec>;
#[doc = "Register `PODR` writer"]
pub type W = crate::W<PodrSpec>;
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
    #[doc = "Bits 0:15 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PODR").field("podr", &self.podr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<PodrSpec> {
        PodrW::new(self, 0)
    }
}
#[doc = "Output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`podr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PodrSpec;
impl crate::RegisterSpec for PodrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`podr::R`](R) reader structure"]
impl crate::Readable for PodrSpec {}
#[doc = "`write(|w| ..)` method takes [`podr::W`](W) writer structure"]
impl crate::Writable for PodrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PODR to value 0"]
impl crate::Resettable for PodrSpec {}

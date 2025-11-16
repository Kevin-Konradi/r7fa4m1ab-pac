#[doc = "Register `DELSR%s` reader"]
pub type R = crate::R<DelsrSpec>;
#[doc = "Register `DELSR%s` writer"]
pub type W = crate::W<DelsrSpec>;
#[doc = "Event selection to DMAC Start request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dels {
    #[doc = "0: Nothing is selected."]
    _0x000 = 0,
    #[doc = "1: See Event Table"]
    Others = 1,
}
impl From<Dels> for u8 {
    #[inline(always)]
    fn from(variant: Dels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dels {
    type Ux = u8;
}
impl crate::IsEnum for Dels {}
#[doc = "Field `DELS` reader - Event selection to DMAC Start request"]
pub type DelsR = crate::FieldReader<Dels>;
impl DelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dels {
        match self.bits {
            0 => Dels::_0x000,
            _ => Dels::Others,
        }
    }
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == Dels::_0x000
    }
    #[doc = "See Event Table"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dels::Others)
    }
}
#[doc = "Field `DELS` writer - Event selection to DMAC Start request"]
pub type DelsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Dels, crate::Safe>;
impl<'a, REG> DelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(Dels::_0x000)
    }
    #[doc = "See Event Table"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dels::Others)
    }
}
impl R {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    pub fn dels(&self) -> DelsR {
        DelsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELSR").field("dels", &self.dels()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    pub fn dels(&mut self) -> DelsW<DelsrSpec> {
        DelsW::new(self, 0)
    }
}
#[doc = "DMAC Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`delsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelsrSpec;
impl crate::RegisterSpec for DelsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`delsr::R`](R) reader structure"]
impl crate::Readable for DelsrSpec {}
#[doc = "`write(|w| ..)` method takes [`delsr::W`](W) writer structure"]
impl crate::Writable for DelsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DELSR%s to value 0"]
impl crate::Resettable for DelsrSpec {}

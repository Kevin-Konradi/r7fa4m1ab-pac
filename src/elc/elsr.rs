#[doc = "Register `ELSR%s` reader"]
pub type R = crate::R<ElsrSpec>;
#[doc = "Register `ELSR%s` writer"]
pub type W = crate::W<ElsrSpec>;
#[doc = "Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Els {
    #[doc = "0: Event output to the corresponding peripheral module is disabled."]
    _0x00 = 0,
    #[doc = "1: Set the number for the event signal to be linked."]
    Others = 1,
}
impl From<Els> for u8 {
    #[inline(always)]
    fn from(variant: Els) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Els {
    type Ux = u8;
}
impl crate::IsEnum for Els {}
#[doc = "Field `ELS` reader - Event Link Select"]
pub type ElsR = crate::FieldReader<Els>;
impl ElsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Els {
        match self.bits {
            0 => Els::_0x00,
            _ => Els::Others,
        }
    }
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Els::_0x00
    }
    #[doc = "Set the number for the event signal to be linked."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Els::Others)
    }
}
#[doc = "Field `ELS` writer - Event Link Select"]
pub type ElsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Els, crate::Safe>;
impl<'a, REG> ElsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Els::_0x00)
    }
    #[doc = "Set the number for the event signal to be linked."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Els::Others)
    }
}
impl R {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    pub fn els(&self) -> ElsR {
        ElsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELSR").field("els", &self.els()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    pub fn els(&mut self) -> ElsW<ElsrSpec> {
        ElsW::new(self, 0)
    }
}
#[doc = "Event Link Setting Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`elsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElsrSpec;
impl crate::RegisterSpec for ElsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`elsr::R`](R) reader structure"]
impl crate::Readable for ElsrSpec {}
#[doc = "`write(|w| ..)` method takes [`elsr::W`](W) writer structure"]
impl crate::Writable for ElsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ELSR%s to value 0"]
impl crate::Resettable for ElsrSpec {}

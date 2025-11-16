#[doc = "Register `SELSR0` reader"]
pub type R = crate::R<Selsr0Spec>;
#[doc = "Register `SELSR0` writer"]
pub type W = crate::W<Selsr0Spec>;
#[doc = "SYS Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sels {
    #[doc = "0: Disable event output to the associated low-power mode module"]
    _0x00 = 0,
    #[doc = "1: Event signal number to be linked"]
    Others = 1,
}
impl From<Sels> for u8 {
    #[inline(always)]
    fn from(variant: Sels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sels {
    type Ux = u8;
}
impl crate::IsEnum for Sels {}
#[doc = "Field `SELS` reader - SYS Event Link Select"]
pub type SelsR = crate::FieldReader<Sels>;
impl SelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sels {
        match self.bits {
            0 => Sels::_0x00,
            _ => Sels::Others,
        }
    }
    #[doc = "Disable event output to the associated low-power mode module"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Sels::_0x00
    }
    #[doc = "Event signal number to be linked"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Sels::Others)
    }
}
#[doc = "Field `SELS` writer - SYS Event Link Select"]
pub type SelsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Sels, crate::Safe>;
impl<'a, REG> SelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable event output to the associated low-power mode module"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::_0x00)
    }
    #[doc = "Event signal number to be linked"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Others)
    }
}
impl R {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(&self) -> SelsR {
        SelsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SELSR0")
            .field("sels", &self.sels())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(&mut self) -> SelsW<Selsr0Spec> {
        SelsW::new(self, 0)
    }
}
#[doc = "SYS Event Link Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Selsr0Spec;
impl crate::RegisterSpec for Selsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`selsr0::R`](R) reader structure"]
impl crate::Readable for Selsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`selsr0::W`](W) writer structure"]
impl crate::Writable for Selsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELSR0 to value 0"]
impl crate::Resettable for Selsr0Spec {}

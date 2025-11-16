#[doc = "Register `DAVREFCR` reader"]
pub type R = crate::R<DavrefcrSpec>;
#[doc = "Register `DAVREFCR` writer"]
pub type W = crate::W<DavrefcrSpec>;
#[doc = "D/A Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ref {
    #[doc = "0: Not selected"]
    _000 = 0,
    #[doc = "1: AVCC0/AVSS0"]
    _001 = 1,
    #[doc = "3: Internal reference voltage/AVSS0"]
    _011 = 3,
    #[doc = "6: VREFH/VREFL"]
    _110 = 6,
    #[doc = "2: Setting prohibited"]
    Others = 2,
}
impl From<Ref> for u8 {
    #[inline(always)]
    fn from(variant: Ref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ref {
    type Ux = u8;
}
impl crate::IsEnum for Ref {}
#[doc = "Field `REF` reader - D/A Reference Voltage Select"]
pub type RefR = crate::FieldReader<Ref>;
impl RefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ref {
        match self.bits {
            0 => Ref::_000,
            1 => Ref::_001,
            3 => Ref::_011,
            6 => Ref::_110,
            _ => Ref::Others,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ref::_000
    }
    #[doc = "AVCC0/AVSS0"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ref::_001
    }
    #[doc = "Internal reference voltage/AVSS0"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ref::_011
    }
    #[doc = "VREFH/VREFL"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ref::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ref::Others)
    }
}
#[doc = "Field `REF` writer - D/A Reference Voltage Select"]
pub type RefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ref, crate::Safe>;
impl<'a, REG> RefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_000)
    }
    #[doc = "AVCC0/AVSS0"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_001)
    }
    #[doc = "Internal reference voltage/AVSS0"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_011)
    }
    #[doc = "VREFH/VREFL"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAVREFCR")
            .field("ref_", &self.ref_())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    pub fn ref_(&mut self) -> RefW<DavrefcrSpec> {
        RefW::new(self, 0)
    }
}
#[doc = "D/A VREF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`davrefcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`davrefcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DavrefcrSpec;
impl crate::RegisterSpec for DavrefcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`davrefcr::R`](R) reader structure"]
impl crate::Readable for DavrefcrSpec {}
#[doc = "`write(|w| ..)` method takes [`davrefcr::W`](W) writer structure"]
impl crate::Writable for DavrefcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAVREFCR to value 0"]
impl crate::Resettable for DavrefcrSpec {}

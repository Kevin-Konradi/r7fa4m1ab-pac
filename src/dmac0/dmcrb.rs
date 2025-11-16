#[doc = "Register `DMCRB` reader"]
pub type R = crate::R<DmcrbSpec>;
#[doc = "Register `DMCRB` writer"]
pub type W = crate::W<DmcrbSpec>;
#[doc = "Specifies the number of block transfer operations or repeat transfer operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Dmcrb {
    #[doc = "0: 65,536 blocks"]
    _0000 = 0,
    #[doc = "1: DMCRB blocks"]
    Others = 1,
}
impl From<Dmcrb> for u16 {
    #[inline(always)]
    fn from(variant: Dmcrb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmcrb {
    type Ux = u16;
}
impl crate::IsEnum for Dmcrb {}
#[doc = "Field `DMCRB` reader - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DmcrbR = crate::FieldReader<Dmcrb>;
impl DmcrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmcrb {
        match self.bits {
            0 => Dmcrb::_0000,
            _ => Dmcrb::Others,
        }
    }
    #[doc = "65,536 blocks"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Dmcrb::_0000
    }
    #[doc = "DMCRB blocks"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dmcrb::Others)
    }
}
#[doc = "Field `DMCRB` writer - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DmcrbW<'a, REG> = crate::FieldWriter<'a, REG, 16, Dmcrb, crate::Safe>;
impl<'a, REG> DmcrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "65,536 blocks"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Dmcrb::_0000)
    }
    #[doc = "DMCRB blocks"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dmcrb::Others)
    }
}
impl R {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub fn dmcrb(&self) -> DmcrbR {
        DmcrbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMCRB")
            .field("dmcrb", &self.dmcrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub fn dmcrb(&mut self) -> DmcrbW<DmcrbSpec> {
        DmcrbW::new(self, 0)
    }
}
#[doc = "DMA Block Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmcrbSpec;
impl crate::RegisterSpec for DmcrbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmcrb::R`](R) reader structure"]
impl crate::Readable for DmcrbSpec {}
#[doc = "`write(|w| ..)` method takes [`dmcrb::W`](W) writer structure"]
impl crate::Writable for DmcrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMCRB to value 0"]
impl crate::Resettable for DmcrbSpec {}

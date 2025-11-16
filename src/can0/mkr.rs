#[doc = "Register `MKR[%s]` reader"]
pub type R = crate::R<MkrSpec>;
#[doc = "Register `MKR[%s]` writer"]
pub type W = crate::W<MkrSpec>;
#[doc = "Field `EID` reader - Extended ID"]
pub type EidR = crate::FieldReader<u32>;
#[doc = "Field `EID` writer - Extended ID"]
pub type EidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SID` reader - Standard ID"]
pub type SidR = crate::FieldReader<u16>;
#[doc = "Field `SID` writer - Standard ID"]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EidR {
        EidR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MKR")
            .field("sid", &self.sid())
            .field("eid", &self.eid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&mut self) -> EidW<MkrSpec> {
        EidW::new(self, 0)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&mut self) -> SidW<MkrSpec> {
        SidW::new(self, 18)
    }
}
#[doc = "Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MkrSpec;
impl crate::RegisterSpec for MkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mkr::R`](R) reader structure"]
impl crate::Readable for MkrSpec {}
#[doc = "`write(|w| ..)` method takes [`mkr::W`](W) writer structure"]
impl crate::Writable for MkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MKR[%s] to value 0"]
impl crate::Resettable for MkrSpec {}

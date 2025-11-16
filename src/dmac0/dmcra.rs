#[doc = "Register `DMCRA` reader"]
pub type R = crate::R<DmcraSpec>;
#[doc = "Register `DMCRA` writer"]
pub type W = crate::W<DmcraSpec>;
#[doc = "Field `DMCRAL` reader - Lower bits of transfer count"]
pub type DmcralR = crate::FieldReader<u16>;
#[doc = "Field `DMCRAL` writer - Lower bits of transfer count"]
pub type DmcralW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DMCRAH` reader - Upper bits of transfer count"]
pub type DmcrahR = crate::FieldReader<u16>;
#[doc = "Field `DMCRAH` writer - Upper bits of transfer count"]
pub type DmcrahW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    pub fn dmcral(&self) -> DmcralR {
        DmcralR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    pub fn dmcrah(&self) -> DmcrahR {
        DmcrahR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMCRA")
            .field("dmcrah", &self.dmcrah())
            .field("dmcral", &self.dmcral())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    pub fn dmcral(&mut self) -> DmcralW<DmcraSpec> {
        DmcralW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    pub fn dmcrah(&mut self) -> DmcrahW<DmcraSpec> {
        DmcrahW::new(self, 16)
    }
}
#[doc = "DMA Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmcraSpec;
impl crate::RegisterSpec for DmcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcra::R`](R) reader structure"]
impl crate::Readable for DmcraSpec {}
#[doc = "`write(|w| ..)` method takes [`dmcra::W`](W) writer structure"]
impl crate::Writable for DmcraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMCRA to value 0"]
impl crate::Resettable for DmcraSpec {}

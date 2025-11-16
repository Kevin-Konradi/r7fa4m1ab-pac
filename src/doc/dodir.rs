#[doc = "Register `DODIR` reader"]
pub type R = crate::R<DodirSpec>;
#[doc = "Register `DODIR` writer"]
pub type W = crate::W<DodirSpec>;
#[doc = "Field `DODIR` reader - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
pub type DodirR = crate::FieldReader<u16>;
#[doc = "Field `DODIR` writer - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
pub type DodirW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[inline(always)]
    pub fn dodir(&self) -> DodirR {
        DodirR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DODIR")
            .field("dodir", &self.dodir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[inline(always)]
    pub fn dodir(&mut self) -> DodirW<DodirSpec> {
        DodirW::new(self, 0)
    }
}
#[doc = "DOC Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dodir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DodirSpec;
impl crate::RegisterSpec for DodirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dodir::R`](R) reader structure"]
impl crate::Readable for DodirSpec {}
#[doc = "`write(|w| ..)` method takes [`dodir::W`](W) writer structure"]
impl crate::Writable for DodirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DODIR to value 0"]
impl crate::Resettable for DodirSpec {}

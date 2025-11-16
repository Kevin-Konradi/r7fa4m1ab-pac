#[doc = "Register `PIPE%sTRN` reader"]
pub type R = crate::R<PipetrnSpec>;
#[doc = "Register `PIPE%sTRN` writer"]
pub type W = crate::W<PipetrnSpec>;
#[doc = "Field `TRNCNT` reader - Transaction Counter"]
pub type TrncntR = crate::FieldReader<u16>;
#[doc = "Field `TRNCNT` writer - Transaction Counter"]
pub type TrncntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transaction Counter"]
    #[inline(always)]
    pub fn trncnt(&self) -> TrncntR {
        TrncntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPETRN")
            .field("trncnt", &self.trncnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Transaction Counter"]
    #[inline(always)]
    pub fn trncnt(&mut self) -> TrncntW<PipetrnSpec> {
        TrncntW::new(self, 0)
    }
}
#[doc = "Pipe %s Transaction Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipetrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipetrnSpec;
impl crate::RegisterSpec for PipetrnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipetrn::R`](R) reader structure"]
impl crate::Readable for PipetrnSpec {}
#[doc = "`write(|w| ..)` method takes [`pipetrn::W`](W) writer structure"]
impl crate::Writable for PipetrnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPE%sTRN to value 0"]
impl crate::Resettable for PipetrnSpec {}

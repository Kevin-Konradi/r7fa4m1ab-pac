#[doc = "Register `IWDTRR` reader"]
pub type R = crate::R<IwdtrrSpec>;
#[doc = "Register `IWDTRR` writer"]
pub type W = crate::W<IwdtrrSpec>;
#[doc = "Field `IWDTRR` reader - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
pub type IwdtrrR = crate::FieldReader;
#[doc = "Field `IWDTRR` writer - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
pub type IwdtrrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    pub fn iwdtrr(&self) -> IwdtrrR {
        IwdtrrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDTRR")
            .field("iwdtrr", &self.iwdtrr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    pub fn iwdtrr(&mut self) -> IwdtrrW<IwdtrrSpec> {
        IwdtrrW::new(self, 0)
    }
}
#[doc = "IWDT Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdtrrSpec;
impl crate::RegisterSpec for IwdtrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iwdtrr::R`](R) reader structure"]
impl crate::Readable for IwdtrrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdtrr::W`](W) writer structure"]
impl crate::Writable for IwdtrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDTRR to value 0xff"]
impl crate::Resettable for IwdtrrSpec {
    const RESET_VALUE: u8 = 0xff;
}

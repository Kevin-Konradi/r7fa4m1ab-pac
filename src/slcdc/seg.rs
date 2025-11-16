#[doc = "Register `SEG%s` reader"]
pub type R = crate::R<SegSpec>;
#[doc = "Register `SEG%s` writer"]
pub type W = crate::W<SegSpec>;
#[doc = "Field `SEG` reader - LCD Display Data"]
pub type SegR = crate::FieldReader;
#[doc = "Field `SEG` writer - LCD Display Data"]
pub type SegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    pub fn seg(&self) -> SegR {
        SegR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEG").field("seg", &self.seg()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    pub fn seg(&mut self) -> SegW<SegSpec> {
        SegW::new(self, 0)
    }
}
#[doc = "LCD Display Data Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`seg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SegSpec;
impl crate::RegisterSpec for SegSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seg::R`](R) reader structure"]
impl crate::Readable for SegSpec {}
#[doc = "`write(|w| ..)` method takes [`seg::W`](W) writer structure"]
impl crate::Writable for SegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEG%s to value 0"]
impl crate::Resettable for SegSpec {}

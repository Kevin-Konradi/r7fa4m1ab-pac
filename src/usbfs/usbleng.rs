#[doc = "Register `USBLENG` reader"]
pub type R = crate::R<UsblengSpec>;
#[doc = "Register `USBLENG` writer"]
pub type W = crate::W<UsblengSpec>;
#[doc = "Field `WLENTUH` reader - Length These bits store the USB request wLength value."]
pub type WlentuhR = crate::FieldReader<u16>;
#[doc = "Field `WLENTUH` writer - Length These bits store the USB request wLength value."]
pub type WlentuhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Length These bits store the USB request wLength value."]
    #[inline(always)]
    pub fn wlentuh(&self) -> WlentuhR {
        WlentuhR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBLENG")
            .field("wlentuh", &self.wlentuh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Length These bits store the USB request wLength value."]
    #[inline(always)]
    pub fn wlentuh(&mut self) -> WlentuhW<UsblengSpec> {
        WlentuhW::new(self, 0)
    }
}
#[doc = "USB Request Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbleng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbleng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsblengSpec;
impl crate::RegisterSpec for UsblengSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbleng::R`](R) reader structure"]
impl crate::Readable for UsblengSpec {}
#[doc = "`write(|w| ..)` method takes [`usbleng::W`](W) writer structure"]
impl crate::Writable for UsblengSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBLENG to value 0"]
impl crate::Resettable for UsblengSpec {}

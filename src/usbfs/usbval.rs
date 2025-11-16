#[doc = "Register `USBVAL` reader"]
pub type R = crate::R<UsbvalSpec>;
#[doc = "Register `USBVAL` writer"]
pub type W = crate::W<UsbvalSpec>;
#[doc = "Field `WVALUE` reader - Value These bits store the USB request Value value."]
pub type WvalueR = crate::FieldReader<u16>;
#[doc = "Field `WVALUE` writer - Value These bits store the USB request Value value."]
pub type WvalueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value These bits store the USB request Value value."]
    #[inline(always)]
    pub fn wvalue(&self) -> WvalueR {
        WvalueR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBVAL")
            .field("wvalue", &self.wvalue())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Value These bits store the USB request Value value."]
    #[inline(always)]
    pub fn wvalue(&mut self) -> WvalueW<UsbvalSpec> {
        WvalueW::new(self, 0)
    }
}
#[doc = "USB Request Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbvalSpec;
impl crate::RegisterSpec for UsbvalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbval::R`](R) reader structure"]
impl crate::Readable for UsbvalSpec {}
#[doc = "`write(|w| ..)` method takes [`usbval::W`](W) writer structure"]
impl crate::Writable for UsbvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBVAL to value 0"]
impl crate::Resettable for UsbvalSpec {}

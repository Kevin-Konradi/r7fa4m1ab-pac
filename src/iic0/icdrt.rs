#[doc = "Register `ICDRT` reader"]
pub type R = crate::R<IcdrtSpec>;
#[doc = "Register `ICDRT` writer"]
pub type W = crate::W<IcdrtSpec>;
#[doc = "Field `ICDRT` reader - 8-bit read-write register that stores transmit data."]
pub type IcdrtR = crate::FieldReader;
#[doc = "Field `ICDRT` writer - 8-bit read-write register that stores transmit data."]
pub type IcdrtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit read-write register that stores transmit data."]
    #[inline(always)]
    pub fn icdrt(&self) -> IcdrtR {
        IcdrtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICDRT")
            .field("icdrt", &self.icdrt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit read-write register that stores transmit data."]
    #[inline(always)]
    pub fn icdrt(&mut self) -> IcdrtW<IcdrtSpec> {
        IcdrtW::new(self, 0)
    }
}
#[doc = "I2C Bus Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdrtSpec;
impl crate::RegisterSpec for IcdrtSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icdrt::R`](R) reader structure"]
impl crate::Readable for IcdrtSpec {}
#[doc = "`write(|w| ..)` method takes [`icdrt::W`](W) writer structure"]
impl crate::Writable for IcdrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICDRT to value 0xff"]
impl crate::Resettable for IcdrtSpec {
    const RESET_VALUE: u8 = 0xff;
}

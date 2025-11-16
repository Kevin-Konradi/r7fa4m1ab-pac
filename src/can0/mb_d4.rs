#[doc = "Register `MB%s_D4` reader"]
pub type R = crate::R<MbD4Spec>;
#[doc = "Register `MB%s_D4` writer"]
pub type W = crate::W<MbD4Spec>;
#[doc = "Field `DATA4` reader - Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D4")
            .field("data4", &self.data4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 4 DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<MbD4Spec> {
        Data4W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD4Spec;
impl crate::RegisterSpec for MbD4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d4::R`](R) reader structure"]
impl crate::Readable for MbD4Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d4::W`](W) writer structure"]
impl crate::Writable for MbD4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D4 to value 0"]
impl crate::Resettable for MbD4Spec {}

#[doc = "Register `MB%s_D7` reader"]
pub type R = crate::R<MbD7Spec>;
#[doc = "Register `MB%s_D7` writer"]
pub type W = crate::W<MbD7Spec>;
#[doc = "Field `DATA7` reader - Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D7")
            .field("data7", &self.data7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 7 DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<MbD7Spec> {
        Data7W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD7Spec;
impl crate::RegisterSpec for MbD7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d7::R`](R) reader structure"]
impl crate::Readable for MbD7Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d7::W`](W) writer structure"]
impl crate::Writable for MbD7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D7 to value 0"]
impl crate::Resettable for MbD7Spec {}

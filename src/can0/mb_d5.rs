#[doc = "Register `MB%s_D5` reader"]
pub type R = crate::R<MbD5Spec>;
#[doc = "Register `MB%s_D5` writer"]
pub type W = crate::W<MbD5Spec>;
#[doc = "Field `DATA5` reader - Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D5")
            .field("data5", &self.data5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 5 DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<MbD5Spec> {
        Data5W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD5Spec;
impl crate::RegisterSpec for MbD5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d5::R`](R) reader structure"]
impl crate::Readable for MbD5Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d5::W`](W) writer structure"]
impl crate::Writable for MbD5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D5 to value 0"]
impl crate::Resettable for MbD5Spec {}

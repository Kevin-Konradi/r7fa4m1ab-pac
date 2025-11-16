#[doc = "Register `MB%s_D0` reader"]
pub type R = crate::R<MbD0Spec>;
#[doc = "Register `MB%s_D0` writer"]
pub type W = crate::W<MbD0Spec>;
#[doc = "Field `DATA0` reader - Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D0")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 0. DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<MbD0Spec> {
        Data0W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD0Spec;
impl crate::RegisterSpec for MbD0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d0::R`](R) reader structure"]
impl crate::Readable for MbD0Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d0::W`](W) writer structure"]
impl crate::Writable for MbD0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D0 to value 0"]
impl crate::Resettable for MbD0Spec {}

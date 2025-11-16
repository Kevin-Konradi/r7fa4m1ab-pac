#[doc = "Register `MB%s_D6` reader"]
pub type R = crate::R<MbD6Spec>;
#[doc = "Register `MB%s_D6` writer"]
pub type W = crate::W<MbD6Spec>;
#[doc = "Field `DATA6` reader - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D6")
            .field("data6", &self.data6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<MbD6Spec> {
        Data6W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD6Spec;
impl crate::RegisterSpec for MbD6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d6::R`](R) reader structure"]
impl crate::Readable for MbD6Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d6::W`](W) writer structure"]
impl crate::Writable for MbD6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D6 to value 0"]
impl crate::Resettable for MbD6Spec {}

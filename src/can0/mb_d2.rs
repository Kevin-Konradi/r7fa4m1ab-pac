#[doc = "Register `MB%s_D2` reader"]
pub type R = crate::R<MbD2Spec>;
#[doc = "Register `MB%s_D2` writer"]
pub type W = crate::W<MbD2Spec>;
#[doc = "Field `DATA2` reader - Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D2")
            .field("data2", &self.data2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 2 DATA2 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<MbD2Spec> {
        Data2W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD2Spec;
impl crate::RegisterSpec for MbD2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d2::R`](R) reader structure"]
impl crate::Readable for MbD2Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d2::W`](W) writer structure"]
impl crate::Writable for MbD2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D2 to value 0"]
impl crate::Resettable for MbD2Spec {}

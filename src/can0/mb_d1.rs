#[doc = "Register `MB%s_D1` reader"]
pub type R = crate::R<MbD1Spec>;
#[doc = "Register `MB%s_D1` writer"]
pub type W = crate::W<MbD1Spec>;
#[doc = "Field `DATA1` reader - Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D1")
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 1 DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<MbD1Spec> {
        Data1W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD1Spec;
impl crate::RegisterSpec for MbD1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d1::R`](R) reader structure"]
impl crate::Readable for MbD1Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d1::W`](W) writer structure"]
impl crate::Writable for MbD1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D1 to value 0"]
impl crate::Resettable for MbD1Spec {}

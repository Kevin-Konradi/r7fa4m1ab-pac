#[doc = "Register `MB%s_D3` reader"]
pub type R = crate::R<MbD3Spec>;
#[doc = "Register `MB%s_D3` writer"]
pub type W = crate::W<MbD3Spec>;
#[doc = "Field `DATA3` reader - Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MB_D3")
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 3 DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<MbD3Spec> {
        Data3W::new(self, 0)
    }
}
#[doc = "Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbD3Spec;
impl crate::RegisterSpec for MbD3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d3::R`](R) reader structure"]
impl crate::Readable for MbD3Spec {}
#[doc = "`write(|w| ..)` method takes [`mb_d3::W`](W) writer structure"]
impl crate::Writable for MbD3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MB%s_D3 to value 0"]
impl crate::Resettable for MbD3Spec {}

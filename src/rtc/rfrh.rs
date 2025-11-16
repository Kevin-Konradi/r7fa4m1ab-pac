#[doc = "Register `RFRH` reader"]
pub type R = crate::R<RfrhSpec>;
#[doc = "Register `RFRH` writer"]
pub type W = crate::W<RfrhSpec>;
#[doc = "Field `RFC16` reader - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type Rfc16R = crate::BitReader;
#[doc = "Field `RFC16` writer - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type Rfc16W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc16(&self) -> Rfc16R {
        Rfc16R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFRH")
            .field("rfc16", &self.rfc16())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc16(&mut self) -> Rfc16W<RfrhSpec> {
        Rfc16W::new(self, 0)
    }
}
#[doc = "Frequency Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfrhSpec;
impl crate::RegisterSpec for RfrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rfrh::R`](R) reader structure"]
impl crate::Readable for RfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`rfrh::W`](W) writer structure"]
impl crate::Writable for RfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFRH to value 0"]
impl crate::Resettable for RfrhSpec {}

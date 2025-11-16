#[doc = "Register `R64CNT` reader"]
pub type R = crate::R<R64cntSpec>;
#[doc = "Field `F64HZ` reader - 64Hz"]
pub type F64hzR = crate::BitReader;
#[doc = "Field `F32HZ` reader - 32Hz"]
pub type F32hzR = crate::BitReader;
#[doc = "Field `F16HZ` reader - 16Hz"]
pub type F16hzR = crate::BitReader;
#[doc = "Field `F8HZ` reader - 8Hz"]
pub type F8hzR = crate::BitReader;
#[doc = "Field `F4HZ` reader - 4Hz"]
pub type F4hzR = crate::BitReader;
#[doc = "Field `F2HZ` reader - 2Hz"]
pub type F2hzR = crate::BitReader;
#[doc = "Field `F1HZ` reader - 1Hz"]
pub type F1hzR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 64Hz"]
    #[inline(always)]
    pub fn f64hz(&self) -> F64hzR {
        F64hzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32Hz"]
    #[inline(always)]
    pub fn f32hz(&self) -> F32hzR {
        F32hzR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16Hz"]
    #[inline(always)]
    pub fn f16hz(&self) -> F16hzR {
        F16hzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8Hz"]
    #[inline(always)]
    pub fn f8hz(&self) -> F8hzR {
        F8hzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4Hz"]
    #[inline(always)]
    pub fn f4hz(&self) -> F4hzR {
        F4hzR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2Hz"]
    #[inline(always)]
    pub fn f2hz(&self) -> F2hzR {
        F2hzR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1Hz"]
    #[inline(always)]
    pub fn f1hz(&self) -> F1hzR {
        F1hzR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R64CNT")
            .field("f1hz", &self.f1hz())
            .field("f2hz", &self.f2hz())
            .field("f4hz", &self.f4hz())
            .field("f8hz", &self.f8hz())
            .field("f16hz", &self.f16hz())
            .field("f32hz", &self.f32hz())
            .field("f64hz", &self.f64hz())
            .finish()
    }
}
#[doc = "64-Hz Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`r64cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R64cntSpec;
impl crate::RegisterSpec for R64cntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r64cnt::R`](R) reader structure"]
impl crate::Readable for R64cntSpec {}
#[doc = "`reset()` method sets R64CNT to value 0"]
impl crate::Resettable for R64cntSpec {}

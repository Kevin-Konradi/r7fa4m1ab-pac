#[doc = "Register `CTSUDCLKC` reader"]
pub type R = crate::R<CtsudclkcSpec>;
#[doc = "Register `CTSUDCLKC` writer"]
pub type W = crate::W<CtsudclkcSpec>;
#[doc = "Field `CTSUSSMOD` reader - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CtsussmodR = crate::FieldReader;
#[doc = "Field `CTSUSSMOD` writer - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CtsussmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTSUSSCNT` reader - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CtsusscntR = crate::FieldReader;
#[doc = "Field `CTSUSSCNT` writer - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CtsusscntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    pub fn ctsussmod(&self) -> CtsussmodR {
        CtsussmodR::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    pub fn ctsusscnt(&self) -> CtsusscntR {
        CtsusscntR::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUDCLKC")
            .field("ctsusscnt", &self.ctsusscnt())
            .field("ctsussmod", &self.ctsussmod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    pub fn ctsussmod(&mut self) -> CtsussmodW<CtsudclkcSpec> {
        CtsussmodW::new(self, 0)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    pub fn ctsusscnt(&mut self) -> CtsusscntW<CtsudclkcSpec> {
        CtsusscntW::new(self, 4)
    }
}
#[doc = "CTSU High-Pass Noise Reduction Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsudclkc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsudclkc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsudclkcSpec;
impl crate::RegisterSpec for CtsudclkcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsudclkc::R`](R) reader structure"]
impl crate::Readable for CtsudclkcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctsudclkc::W`](W) writer structure"]
impl crate::Writable for CtsudclkcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUDCLKC to value 0"]
impl crate::Resettable for CtsudclkcSpec {}

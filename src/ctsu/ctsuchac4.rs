#[doc = "Register `CTSUCHAC4` reader"]
pub type R = crate::R<Ctsuchac4Spec>;
#[doc = "Register `CTSUCHAC4` writer"]
pub type W = crate::W<Ctsuchac4Spec>;
#[doc = "CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac4 {
    #[doc = "0: TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target."]
    Ctsuchac4 = 0,
}
impl From<Ctsuchac4> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac4 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac4 {}
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
pub type Ctsuchac4R = crate::FieldReader<Ctsuchac4>;
impl Ctsuchac4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuchac4 {
        match self.bits {
            _ => Ctsuchac4::Ctsuchac4,
        }
    }
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target."]
    #[inline(always)]
    pub fn is_ctsuchac4(&self) -> bool {
        matches!(self.variant(), Ctsuchac4::Ctsuchac4)
    }
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
pub type Ctsuchac4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctsuchac4, crate::Safe>;
impl<'a, REG> Ctsuchac4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target."]
    #[inline(always)]
    pub fn ctsuchac4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac4::Ctsuchac4)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> Ctsuchac4R {
        Ctsuchac4R::new(self.bits & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHAC4")
            .field("ctsuchac4", &self.ctsuchac4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\] corresponds to TS32 and CTSUCHAC4\\[3\\] corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\] should be 0."]
    #[inline(always)]
    pub fn ctsuchac4(&mut self) -> Ctsuchac4W<Ctsuchac4Spec> {
        Ctsuchac4W::new(self, 0)
    }
}
#[doc = "CTSU Channel Enable Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchac4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchac4Spec;
impl crate::RegisterSpec for Ctsuchac4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac4::R`](R) reader structure"]
impl crate::Readable for Ctsuchac4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac4::W`](W) writer structure"]
impl crate::Writable for Ctsuchac4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHAC4 to value 0"]
impl crate::Resettable for Ctsuchac4Spec {}

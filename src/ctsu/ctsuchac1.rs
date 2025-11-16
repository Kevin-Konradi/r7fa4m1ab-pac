#[doc = "Register `CTSUCHAC1` reader"]
pub type R = crate::R<Ctsuchac1Spec>;
#[doc = "Register `CTSUCHAC1` writer"]
pub type W = crate::W<Ctsuchac1Spec>;
#[doc = "CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac1 {
    #[doc = "0: TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target."]
    Ctsuchac1 = 0,
}
impl From<Ctsuchac1> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac1 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac1 {}
#[doc = "Field `CTSUCHAC1` reader - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
pub type Ctsuchac1R = crate::FieldReader<Ctsuchac1>;
impl Ctsuchac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuchac1 {
        match self.bits {
            _ => Ctsuchac1::Ctsuchac1,
        }
    }
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target."]
    #[inline(always)]
    pub fn is_ctsuchac1(&self) -> bool {
        matches!(self.variant(), Ctsuchac1::Ctsuchac1)
    }
}
#[doc = "Field `CTSUCHAC1` writer - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
pub type Ctsuchac1W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchac1, crate::Safe>;
impl<'a, REG> Ctsuchac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target."]
    #[inline(always)]
    pub fn ctsuchac1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac1::Ctsuchac1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
    #[inline(always)]
    pub fn ctsuchac1(&self) -> Ctsuchac1R {
        Ctsuchac1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHAC1")
            .field("ctsuchac1", &self.ctsuchac1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\] corresponds to TS08 and CTSUCHAC1\\[7\\] corresponds to TS15."]
    #[inline(always)]
    pub fn ctsuchac1(&mut self) -> Ctsuchac1W<Ctsuchac1Spec> {
        Ctsuchac1W::new(self, 0)
    }
}
#[doc = "CTSU Channel Enable Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchac1Spec;
impl crate::RegisterSpec for Ctsuchac1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac1::R`](R) reader structure"]
impl crate::Readable for Ctsuchac1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac1::W`](W) writer structure"]
impl crate::Writable for Ctsuchac1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHAC1 to value 0"]
impl crate::Resettable for Ctsuchac1Spec {}

#[doc = "Register `CTSUCHAC0` reader"]
pub type R = crate::R<Ctsuchac0Spec>;
#[doc = "Register `CTSUCHAC0` writer"]
pub type W = crate::W<Ctsuchac0Spec>;
#[doc = "CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac0 {
    #[doc = "0: TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target."]
    Ctsuchac0 = 0,
}
impl From<Ctsuchac0> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac0 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac0 {}
#[doc = "Field `CTSUCHAC0` reader - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
pub type Ctsuchac0R = crate::FieldReader<Ctsuchac0>;
impl Ctsuchac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuchac0 {
        match self.bits {
            _ => Ctsuchac0::Ctsuchac0,
        }
    }
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target."]
    #[inline(always)]
    pub fn is_ctsuchac0(&self) -> bool {
        matches!(self.variant(), Ctsuchac0::Ctsuchac0)
    }
}
#[doc = "Field `CTSUCHAC0` writer - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
pub type Ctsuchac0W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchac0, crate::Safe>;
impl<'a, REG> Ctsuchac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target."]
    #[inline(always)]
    pub fn ctsuchac0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac0::Ctsuchac0)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
    #[inline(always)]
    pub fn ctsuchac0(&self) -> Ctsuchac0R {
        Ctsuchac0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHAC0")
            .field("ctsuchac0", &self.ctsuchac0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\] corresponds to TS00 and CTSUCHAC0\\[7\\] corresponds to TS07. but the write value of CTSUCHAC0\\[2\\] should be 0."]
    #[inline(always)]
    pub fn ctsuchac0(&mut self) -> Ctsuchac0W<Ctsuchac0Spec> {
        Ctsuchac0W::new(self, 0)
    }
}
#[doc = "CTSU Channel Enable Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchac0Spec;
impl crate::RegisterSpec for Ctsuchac0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac0::R`](R) reader structure"]
impl crate::Readable for Ctsuchac0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac0::W`](W) writer structure"]
impl crate::Writable for Ctsuchac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHAC0 to value 0"]
impl crate::Resettable for Ctsuchac0Spec {}

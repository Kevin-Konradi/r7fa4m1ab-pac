#[doc = "Register `AMPTRS` reader"]
pub type R = crate::R<AmptrsSpec>;
#[doc = "Register `AMPTRS` writer"]
pub type W = crate::W<AmptrsSpec>;
#[doc = "ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amptrs {
    #[doc = "0: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
    _00 = 0,
    #[doc = "1: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
    _11 = 3,
}
impl From<Amptrs> for u8 {
    #[inline(always)]
    fn from(variant: Amptrs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amptrs {
    type Ux = u8;
}
impl crate::IsEnum for Amptrs {}
#[doc = "Field `AMPTRS` reader - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
pub type AmptrsR = crate::FieldReader<Amptrs>;
impl AmptrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amptrs {
        match self.bits {
            0 => Amptrs::_00,
            1 => Amptrs::_01,
            2 => Amptrs::_10,
            3 => Amptrs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Amptrs::_00
    }
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Amptrs::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Amptrs::_10
    }
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Amptrs::_11
    }
}
#[doc = "Field `AMPTRS` writer - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
pub type AmptrsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Amptrs, crate::Safe>;
impl<'a, REG> AmptrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrs::_00)
    }
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrs::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrs::_10)
    }
    #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Amptrs::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    pub fn amptrs(&self) -> AmptrsR {
        AmptrsR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPTRS")
            .field("amptrs", &self.amptrs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    pub fn amptrs(&mut self) -> AmptrsW<AmptrsSpec> {
        AmptrsW::new(self, 0)
    }
}
#[doc = "Operational Amplifier Activation Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amptrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmptrsSpec;
impl crate::RegisterSpec for AmptrsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`amptrs::R`](R) reader structure"]
impl crate::Readable for AmptrsSpec {}
#[doc = "`write(|w| ..)` method takes [`amptrs::W`](W) writer structure"]
impl crate::Writable for AmptrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMPTRS to value 0"]
impl crate::Resettable for AmptrsSpec {}

#[doc = "Register `PLLCCR2` reader"]
pub type R = crate::R<Pllccr2Spec>;
#[doc = "Register `PLLCCR2` writer"]
pub type W = crate::W<Pllccr2Spec>;
#[doc = "PLL Frequency Multiplication Factor Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllmul {
    #[doc = "15: Settings prohibited."]
    _1111 = 15,
    #[doc = "0: x PLLMUL\\[4:0\\] +1"]
    Others = 0,
}
impl From<Pllmul> for u8 {
    #[inline(always)]
    fn from(variant: Pllmul) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllmul {
    type Ux = u8;
}
impl crate::IsEnum for Pllmul {}
#[doc = "Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select"]
pub type PllmulR = crate::FieldReader<Pllmul>;
impl PllmulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllmul {
        match self.bits {
            15 => Pllmul::_1111,
            _ => Pllmul::Others,
        }
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Pllmul::_1111
    }
    #[doc = "x PLLMUL\\[4:0\\] +1"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pllmul::Others)
    }
}
#[doc = "Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select"]
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pllmul, crate::Safe>;
impl<'a, REG> PllmulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmul::_1111)
    }
    #[doc = "x PLLMUL\\[4:0\\] +1"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pllmul::Others)
    }
}
#[doc = "PLL Output Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plodiv {
    #[doc = "0: /1."]
    _00 = 0,
    #[doc = "1: /2."]
    _01 = 1,
    #[doc = "2: /4."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<Plodiv> for u8 {
    #[inline(always)]
    fn from(variant: Plodiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plodiv {
    type Ux = u8;
}
impl crate::IsEnum for Plodiv {}
#[doc = "Field `PLODIV` reader - PLL Output Frequency Division Ratio Select"]
pub type PlodivR = crate::FieldReader<Plodiv>;
impl PlodivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plodiv {
        match self.bits {
            0 => Plodiv::_00,
            1 => Plodiv::_01,
            2 => Plodiv::_10,
            3 => Plodiv::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "/1."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Plodiv::_00
    }
    #[doc = "/2."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Plodiv::_01
    }
    #[doc = "/4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Plodiv::_10
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Plodiv::_11
    }
}
#[doc = "Field `PLODIV` writer - PLL Output Frequency Division Ratio Select"]
pub type PlodivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plodiv, crate::Safe>;
impl<'a, REG> PlodivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Plodiv::_00)
    }
    #[doc = "/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Plodiv::_01)
    }
    #[doc = "/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Plodiv::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Plodiv::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 6:7 - PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plodiv(&self) -> PlodivR {
        PlodivR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCCR2")
            .field("plodiv", &self.plodiv())
            .field("pllmul", &self.pllmul())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PllmulW<Pllccr2Spec> {
        PllmulW::new(self, 0)
    }
    #[doc = "Bits 6:7 - PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plodiv(&mut self) -> PlodivW<Pllccr2Spec> {
        PlodivW::new(self, 6)
    }
}
#[doc = "PLL Clock Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pllccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllccr2Spec;
impl crate::RegisterSpec for Pllccr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllccr2::R`](R) reader structure"]
impl crate::Readable for Pllccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pllccr2::W`](W) writer structure"]
impl crate::Writable for Pllccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCCR2 to value 0x07"]
impl crate::Resettable for Pllccr2Spec {
    const RESET_VALUE: u8 = 0x07;
}

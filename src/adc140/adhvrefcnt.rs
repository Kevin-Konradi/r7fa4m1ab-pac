#[doc = "Register `ADHVREFCNT` reader"]
pub type R = crate::R<AdhvrefcntSpec>;
#[doc = "Register `ADHVREFCNT` writer"]
pub type W = crate::W<AdhvrefcntSpec>;
#[doc = "High-Potential Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hvsel {
    #[doc = "0: AVCC0 is selected as the high-potential reference voltage"]
    _00 = 0,
    #[doc = "1: VREFH0 is selected as the high-potential reference voltage"]
    _01 = 1,
    #[doc = "2: Internal reference voltage is selected as the high-potential reference voltage"]
    _10 = 2,
    #[doc = "3: Internal node discharge. No reference voltage pin is selected."]
    _11 = 3,
}
impl From<Hvsel> for u8 {
    #[inline(always)]
    fn from(variant: Hvsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hvsel {
    type Ux = u8;
}
impl crate::IsEnum for Hvsel {}
#[doc = "Field `HVSEL` reader - High-Potential Reference Voltage Select"]
pub type HvselR = crate::FieldReader<Hvsel>;
impl HvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvsel {
        match self.bits {
            0 => Hvsel::_00,
            1 => Hvsel::_01,
            2 => Hvsel::_10,
            3 => Hvsel::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Hvsel::_00
    }
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Hvsel::_01
    }
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Hvsel::_10
    }
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Hvsel::_11
    }
}
#[doc = "Field `HVSEL` writer - High-Potential Reference Voltage Select"]
pub type HvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hvsel, crate::Safe>;
impl<'a, REG> HvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Hvsel::_00)
    }
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Hvsel::_01)
    }
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Hvsel::_10)
    }
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Hvsel::_11)
    }
}
#[doc = "Low-Potential Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvsel {
    #[doc = "0: AVSS0 is selected as the low-potential reference voltage"]
    _0 = 0,
    #[doc = "1: VREFL0 is selected as the low-potential reference voltage."]
    _1 = 1,
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(variant: Lvsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvsel {
    type Ux = u8;
}
impl crate::IsEnum for Lvsel {}
#[doc = "Field `LVSEL` reader - Low-Potential Reference Voltage Select"]
pub type LvselR = crate::FieldReader<Lvsel>;
impl LvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lvsel> {
        match self.bits {
            0 => Some(Lvsel::_0),
            1 => Some(Lvsel::_1),
            _ => None,
        }
    }
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvsel::_0
    }
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvsel::_1
    }
}
#[doc = "Field `LVSEL` writer - Low-Potential Reference Voltage Select"]
pub type LvselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lvsel>;
impl<'a, REG> LvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvsel::_0)
    }
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvsel::_1)
    }
}
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adslp {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Standby state."]
    _1 = 1,
}
impl From<Adslp> for bool {
    #[inline(always)]
    fn from(variant: Adslp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSLP` reader - Sleep"]
pub type AdslpR = crate::BitReader<Adslp>;
impl AdslpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adslp {
        match self.bits {
            false => Adslp::_0,
            true => Adslp::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adslp::_0
    }
    #[doc = "Standby state."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adslp::_1
    }
}
#[doc = "Field `ADSLP` writer - Sleep"]
pub type AdslpW<'a, REG> = crate::BitWriter<'a, REG, Adslp>;
impl<'a, REG> AdslpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adslp::_0)
    }
    #[doc = "Standby state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adslp::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - High-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn hvsel(&self) -> HvselR {
        HvselR::new(self.bits & 3)
    }
    #[doc = "Bits 2:4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn lvsel(&self) -> LvselR {
        LvselR::new((self.bits >> 2) & 7)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    pub fn adslp(&self) -> AdslpR {
        AdslpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADHVREFCNT")
            .field("adslp", &self.adslp())
            .field("lvsel", &self.lvsel())
            .field("hvsel", &self.hvsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - High-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn hvsel(&mut self) -> HvselW<AdhvrefcntSpec> {
        HvselW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn lvsel(&mut self) -> LvselW<AdhvrefcntSpec> {
        LvselW::new(self, 2)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    pub fn adslp(&mut self) -> AdslpW<AdhvrefcntSpec> {
        AdslpW::new(self, 7)
    }
}
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adhvrefcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adhvrefcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdhvrefcntSpec;
impl crate::RegisterSpec for AdhvrefcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adhvrefcnt::R`](R) reader structure"]
impl crate::Readable for AdhvrefcntSpec {}
#[doc = "`write(|w| ..)` method takes [`adhvrefcnt::W`](W) writer structure"]
impl crate::Writable for AdhvrefcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADHVREFCNT to value 0"]
impl crate::Resettable for AdhvrefcntSpec {}

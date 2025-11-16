#[doc = "Register `IRQCR%s` reader"]
pub type R = crate::R<IrqcrSpec>;
#[doc = "Register `IRQCR%s` writer"]
pub type W = crate::W<IrqcrSpec>;
#[doc = "IRQ Detection Sense Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irqmd {
    #[doc = "0: Falling edge"]
    _00 = 0,
    #[doc = "1: Rising edge"]
    _01 = 1,
    #[doc = "2: Rising and falling edges"]
    _10 = 2,
    #[doc = "3: Low level"]
    _11 = 3,
}
impl From<Irqmd> for u8 {
    #[inline(always)]
    fn from(variant: Irqmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqmd {
    type Ux = u8;
}
impl crate::IsEnum for Irqmd {}
#[doc = "Field `IRQMD` reader - IRQ Detection Sense Select"]
pub type IrqmdR = crate::FieldReader<Irqmd>;
impl IrqmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqmd {
        match self.bits {
            0 => Irqmd::_00,
            1 => Irqmd::_01,
            2 => Irqmd::_10,
            3 => Irqmd::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Irqmd::_00
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Irqmd::_01
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Irqmd::_10
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Irqmd::_11
    }
}
#[doc = "Field `IRQMD` writer - IRQ Detection Sense Select"]
pub type IrqmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Irqmd, crate::Safe>;
impl<'a, REG> IrqmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Irqmd::_00)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Irqmd::_01)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Irqmd::_10)
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Irqmd::_11)
    }
}
#[doc = "IRQ Digital Filter Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fclksel {
    #[doc = "0: PCLKB"]
    _00 = 0,
    #[doc = "1: PCLKB/8"]
    _01 = 1,
    #[doc = "2: PCLKB/32"]
    _10 = 2,
    #[doc = "3: PCLKB/64"]
    _11 = 3,
}
impl From<Fclksel> for u8 {
    #[inline(always)]
    fn from(variant: Fclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fclksel {
    type Ux = u8;
}
impl crate::IsEnum for Fclksel {}
#[doc = "Field `FCLKSEL` reader - IRQ Digital Filter Sampling Clock Select"]
pub type FclkselR = crate::FieldReader<Fclksel>;
impl FclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fclksel {
        match self.bits {
            0 => Fclksel::_00,
            1 => Fclksel::_01,
            2 => Fclksel::_10,
            3 => Fclksel::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Fclksel::_00
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Fclksel::_01
    }
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Fclksel::_10
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Fclksel::_11
    }
}
#[doc = "Field `FCLKSEL` writer - IRQ Digital Filter Sampling Clock Select"]
pub type FclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fclksel, crate::Safe>;
impl<'a, REG> FclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Fclksel::_00)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Fclksel::_01)
    }
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Fclksel::_10)
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Fclksel::_11)
    }
}
#[doc = "IRQ Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flten {
    #[doc = "0: Digital filter disabled."]
    _0 = 0,
    #[doc = "1: Digital filter enabled."]
    _1 = 1,
}
impl From<Flten> for bool {
    #[inline(always)]
    fn from(variant: Flten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTEN` reader - IRQ Digital Filter Enable"]
pub type FltenR = crate::BitReader<Flten>;
impl FltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flten {
        match self.bits {
            false => Flten::_0,
            true => Flten::_1,
        }
    }
    #[doc = "Digital filter disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Flten::_0
    }
    #[doc = "Digital filter enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flten::_1
    }
}
#[doc = "Field `FLTEN` writer - IRQ Digital Filter Enable"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG, Flten>;
impl<'a, REG> FltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital filter disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Flten::_0)
    }
    #[doc = "Digital filter enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flten::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - IRQ Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(&self) -> IrqmdR {
        IrqmdR::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(&self) -> FclkselR {
        FclkselR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - IRQ Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQCR")
            .field("flten", &self.flten())
            .field("fclksel", &self.fclksel())
            .field("irqmd", &self.irqmd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - IRQ Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(&mut self) -> IrqmdW<IrqcrSpec> {
        IrqmdW::new(self, 0)
    }
    #[doc = "Bits 4:5 - IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(&mut self) -> FclkselW<IrqcrSpec> {
        FclkselW::new(self, 4)
    }
    #[doc = "Bit 7 - IRQ Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<IrqcrSpec> {
        FltenW::new(self, 7)
    }
}
#[doc = "IRQ Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`irqcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqcrSpec;
impl crate::RegisterSpec for IrqcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`irqcr::R`](R) reader structure"]
impl crate::Readable for IrqcrSpec {}
#[doc = "`write(|w| ..)` method takes [`irqcr::W`](W) writer structure"]
impl crate::Writable for IrqcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQCR%s to value 0"]
impl crate::Resettable for IrqcrSpec {}

#[doc = "Register `LVD%sCR1` reader"]
pub type R = crate::R<Lvdcr1Spec>;
#[doc = "Register `LVD%sCR1` writer"]
pub type W = crate::W<Lvdcr1Spec>;
#[doc = "Voltage Monitor Interrupt Generation Condition Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idtsel {
    #[doc = "0: When VCC>=Vdet (rise) is detected"]
    _00 = 0,
    #[doc = "1: When VCC<Vdet (drop) is detected"]
    _01 = 1,
    #[doc = "2: When drop and rise are detected"]
    _10 = 2,
    #[doc = "3: Settings prohibited"]
    _11 = 3,
}
impl From<Idtsel> for u8 {
    #[inline(always)]
    fn from(variant: Idtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idtsel {
    type Ux = u8;
}
impl crate::IsEnum for Idtsel {}
#[doc = "Field `IDTSEL` reader - Voltage Monitor Interrupt Generation Condition Select"]
pub type IdtselR = crate::FieldReader<Idtsel>;
impl IdtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idtsel {
        match self.bits {
            0 => Idtsel::_00,
            1 => Idtsel::_01,
            2 => Idtsel::_10,
            3 => Idtsel::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "When VCC>=Vdet (rise) is detected"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Idtsel::_00
    }
    #[doc = "When VCC<Vdet (drop) is detected"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Idtsel::_01
    }
    #[doc = "When drop and rise are detected"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Idtsel::_10
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Idtsel::_11
    }
}
#[doc = "Field `IDTSEL` writer - Voltage Monitor Interrupt Generation Condition Select"]
pub type IdtselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idtsel, crate::Safe>;
impl<'a, REG> IdtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When VCC>=Vdet (rise) is detected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_00)
    }
    #[doc = "When VCC<Vdet (drop) is detected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_01)
    }
    #[doc = "When drop and rise are detected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_10)
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Idtsel::_11)
    }
}
#[doc = "Voltage Monitor Interrupt Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqsel {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Maskable interrupt"]
    _1 = 1,
}
impl From<Irqsel> for bool {
    #[inline(always)]
    fn from(variant: Irqsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQSEL` reader - Voltage Monitor Interrupt Type Select"]
pub type IrqselR = crate::BitReader<Irqsel>;
impl IrqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqsel {
        match self.bits {
            false => Irqsel::_0,
            true => Irqsel::_1,
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqsel::_0
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqsel::_1
    }
}
#[doc = "Field `IRQSEL` writer - Voltage Monitor Interrupt Type Select"]
pub type IrqselW<'a, REG> = crate::BitWriter<'a, REG, Irqsel>;
impl<'a, REG> IrqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqsel::_0)
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqsel::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(&self) -> IdtselR {
        IdtselR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(&self) -> IrqselR {
        IrqselR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDCR1")
            .field("irqsel", &self.irqsel())
            .field("idtsel", &self.idtsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(&mut self) -> IdtselW<Lvdcr1Spec> {
        IdtselW::new(self, 0)
    }
    #[doc = "Bit 2 - Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(&mut self) -> IrqselW<Lvdcr1Spec> {
        IrqselW::new(self, 2)
    }
}
#[doc = "Voltage Monitor %s Circuit Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvdcr1Spec;
impl crate::RegisterSpec for Lvdcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdcr1::R`](R) reader structure"]
impl crate::Readable for Lvdcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lvdcr1::W`](W) writer structure"]
impl crate::Writable for Lvdcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVD%sCR1 to value 0x01"]
impl crate::Resettable for Lvdcr1Spec {
    const RESET_VALUE: u8 = 0x01;
}

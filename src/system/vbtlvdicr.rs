#[doc = "Register `VBTLVDICR` reader"]
pub type R = crate::R<VbtlvdicrSpec>;
#[doc = "Register `VBTLVDICR` writer"]
pub type W = crate::W<VbtlvdicrSpec>;
#[doc = "VBATT Pin Low Voltage Detect Interrupt Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtlvdie {
    #[doc = "0: VBATT Pin Low Voltage Detect Interrupt Disable"]
    _0 = 0,
    #[doc = "1: VBATT Pin Low Voltage Detect Interrupt Enable"]
    _1 = 1,
}
impl From<Vbtlvdie> for bool {
    #[inline(always)]
    fn from(variant: Vbtlvdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTLVDIE` reader - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
pub type VbtlvdieR = crate::BitReader<Vbtlvdie>;
impl VbtlvdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtlvdie {
        match self.bits {
            false => Vbtlvdie::_0,
            true => Vbtlvdie::_1,
        }
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtlvdie::_0
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtlvdie::_1
    }
}
#[doc = "Field `VBTLVDIE` writer - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
pub type VbtlvdieW<'a, REG> = crate::BitWriter<'a, REG, Vbtlvdie>;
impl<'a, REG> VbtlvdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdie::_0)
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdie::_1)
    }
}
#[doc = "Pin Low Voltage Detect Interrupt Select bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtlvdisel {
    #[doc = "0: Non Maskable Interrupt"]
    _0 = 0,
    #[doc = "1: Maskable Interrupt"]
    _1 = 1,
}
impl From<Vbtlvdisel> for bool {
    #[inline(always)]
    fn from(variant: Vbtlvdisel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTLVDISEL` reader - Pin Low Voltage Detect Interrupt Select bit"]
pub type VbtlvdiselR = crate::BitReader<Vbtlvdisel>;
impl VbtlvdiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtlvdisel {
        match self.bits {
            false => Vbtlvdisel::_0,
            true => Vbtlvdisel::_1,
        }
    }
    #[doc = "Non Maskable Interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtlvdisel::_0
    }
    #[doc = "Maskable Interrupt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtlvdisel::_1
    }
}
#[doc = "Field `VBTLVDISEL` writer - Pin Low Voltage Detect Interrupt Select bit"]
pub type VbtlvdiselW<'a, REG> = crate::BitWriter<'a, REG, Vbtlvdisel>;
impl<'a, REG> VbtlvdiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non Maskable Interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdisel::_0)
    }
    #[doc = "Maskable Interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdisel::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    pub fn vbtlvdie(&self) -> VbtlvdieR {
        VbtlvdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    pub fn vbtlvdisel(&self) -> VbtlvdiselR {
        VbtlvdiselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTLVDICR")
            .field("vbtlvdisel", &self.vbtlvdisel())
            .field("vbtlvdie", &self.vbtlvdie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    pub fn vbtlvdie(&mut self) -> VbtlvdieW<VbtlvdicrSpec> {
        VbtlvdieW::new(self, 0)
    }
    #[doc = "Bit 1 - Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    pub fn vbtlvdisel(&mut self) -> VbtlvdiselW<VbtlvdicrSpec> {
        VbtlvdiselW::new(self, 1)
    }
}
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtlvdicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtlvdicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtlvdicrSpec;
impl crate::RegisterSpec for VbtlvdicrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtlvdicr::R`](R) reader structure"]
impl crate::Readable for VbtlvdicrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtlvdicr::W`](W) writer structure"]
impl crate::Writable for VbtlvdicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTLVDICR to value 0"]
impl crate::Resettable for VbtlvdicrSpec {}

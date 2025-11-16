#[doc = "Register `VBTCR2` reader"]
pub type R = crate::R<Vbtcr2Spec>;
#[doc = "Register `VBTCR2` writer"]
pub type W = crate::W<Vbtcr2Spec>;
#[doc = "VBATT Pin Low Voltage Detect Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbtlvden {
    #[doc = "0: VBATT pin low voltage detect disable"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detect enable"]
    _1 = 1,
}
impl From<Vbtlvden> for bool {
    #[inline(always)]
    fn from(variant: Vbtlvden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBTLVDEN` reader - VBATT Pin Low Voltage Detect Enable Bit"]
pub type VbtlvdenR = crate::BitReader<Vbtlvden>;
impl VbtlvdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtlvden {
        match self.bits {
            false => Vbtlvden::_0,
            true => Vbtlvden::_1,
        }
    }
    #[doc = "VBATT pin low voltage detect disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbtlvden::_0
    }
    #[doc = "VBATT pin low voltage detect enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbtlvden::_1
    }
}
#[doc = "Field `VBTLVDEN` writer - VBATT Pin Low Voltage Detect Enable Bit"]
pub type VbtlvdenW<'a, REG> = crate::BitWriter<'a, REG, Vbtlvden>;
impl<'a, REG> VbtlvdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT pin low voltage detect disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvden::_0)
    }
    #[doc = "VBATT pin low voltage detect enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvden::_1)
    }
}
#[doc = "VBATT Pin Voltage Low Voltage Detect Level Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vbtlvdlvl {
    #[doc = "0: 2.7V"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: 2.3V"]
    _10 = 2,
    #[doc = "3: 2.1V"]
    _11 = 3,
}
impl From<Vbtlvdlvl> for u8 {
    #[inline(always)]
    fn from(variant: Vbtlvdlvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vbtlvdlvl {
    type Ux = u8;
}
impl crate::IsEnum for Vbtlvdlvl {}
#[doc = "Field `VBTLVDLVL` reader - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
pub type VbtlvdlvlR = crate::FieldReader<Vbtlvdlvl>;
impl VbtlvdlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbtlvdlvl {
        match self.bits {
            0 => Vbtlvdlvl::_00,
            1 => Vbtlvdlvl::_01,
            2 => Vbtlvdlvl::_10,
            3 => Vbtlvdlvl::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Vbtlvdlvl::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Vbtlvdlvl::_01
    }
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Vbtlvdlvl::_10
    }
    #[doc = "2.1V"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Vbtlvdlvl::_11
    }
}
#[doc = "Field `VBTLVDLVL` writer - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
pub type VbtlvdlvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vbtlvdlvl, crate::Safe>;
impl<'a, REG> VbtlvdlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdlvl::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdlvl::_01)
    }
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdlvl::_10)
    }
    #[doc = "2.1V"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Vbtlvdlvl::_11)
    }
}
impl R {
    #[doc = "Bit 4 - VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    pub fn vbtlvden(&self) -> VbtlvdenR {
        VbtlvdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    pub fn vbtlvdlvl(&self) -> VbtlvdlvlR {
        VbtlvdlvlR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTCR2")
            .field("vbtlvdlvl", &self.vbtlvdlvl())
            .field("vbtlvden", &self.vbtlvden())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    pub fn vbtlvden(&mut self) -> VbtlvdenW<Vbtcr2Spec> {
        VbtlvdenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    pub fn vbtlvdlvl(&mut self) -> VbtlvdlvlW<Vbtcr2Spec> {
        VbtlvdlvlW::new(self, 6)
    }
}
#[doc = "VBATT Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbtcr2Spec;
impl crate::RegisterSpec for Vbtcr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcr2::R`](R) reader structure"]
impl crate::Readable for Vbtcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`vbtcr2::W`](W) writer structure"]
impl crate::Writable for Vbtcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTCR2 to value 0"]
impl crate::Resettable for Vbtcr2Spec {}

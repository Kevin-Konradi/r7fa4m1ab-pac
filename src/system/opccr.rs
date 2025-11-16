#[doc = "Register `OPCCR` reader"]
pub type R = crate::R<OpccrSpec>;
#[doc = "Register `OPCCR` writer"]
pub type W = crate::W<OpccrSpec>;
#[doc = "Operating Power Control Mode Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcm {
    #[doc = "0: High-speed mode"]
    _00 = 0,
    #[doc = "1: Middle-speed mode"]
    _01 = 1,
    #[doc = "2: Low-voltage mode"]
    _10 = 2,
    #[doc = "3: Low-speed mode"]
    _11 = 3,
}
impl From<Opcm> for u8 {
    #[inline(always)]
    fn from(variant: Opcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opcm {
    type Ux = u8;
}
impl crate::IsEnum for Opcm {}
#[doc = "Field `OPCM` reader - Operating Power Control Mode Select"]
pub type OpcmR = crate::FieldReader<Opcm>;
impl OpcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opcm {
        match self.bits {
            0 => Opcm::_00,
            1 => Opcm::_01,
            2 => Opcm::_10,
            3 => Opcm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Opcm::_00
    }
    #[doc = "Middle-speed mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Opcm::_01
    }
    #[doc = "Low-voltage mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Opcm::_10
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Opcm::_11
    }
}
#[doc = "Field `OPCM` writer - Operating Power Control Mode Select"]
pub type OpcmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opcm, crate::Safe>;
impl<'a, REG> OpcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_00)
    }
    #[doc = "Middle-speed mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_01)
    }
    #[doc = "Low-voltage mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_10)
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Opcm::_11)
    }
}
#[doc = "Operating Power Control Mode Transition Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcmtsf {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition"]
    _1 = 1,
}
impl From<Opcmtsf> for bool {
    #[inline(always)]
    fn from(variant: Opcmtsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPCMTSF` reader - Operating Power Control Mode Transition Status Flag"]
pub type OpcmtsfR = crate::BitReader<Opcmtsf>;
impl OpcmtsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opcmtsf {
        match self.bits {
            false => Opcmtsf::_0,
            true => Opcmtsf::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Opcmtsf::_0
    }
    #[doc = "During transition"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Opcmtsf::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(&self) -> OpcmR {
        OpcmR::new(self.bits & 3)
    }
    #[doc = "Bit 4 - Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn opcmtsf(&self) -> OpcmtsfR {
        OpcmtsfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPCCR")
            .field("opcmtsf", &self.opcmtsf())
            .field("opcm", &self.opcm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(&mut self) -> OpcmW<OpccrSpec> {
        OpcmW::new(self, 0)
    }
}
#[doc = "Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpccrSpec;
impl crate::RegisterSpec for OpccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`opccr::R`](R) reader structure"]
impl crate::Readable for OpccrSpec {}
#[doc = "`write(|w| ..)` method takes [`opccr::W`](W) writer structure"]
impl crate::Writable for OpccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCCR to value 0x02"]
impl crate::Resettable for OpccrSpec {
    const RESET_VALUE: u8 = 0x02;
}

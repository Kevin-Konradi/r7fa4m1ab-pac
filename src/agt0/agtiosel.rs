#[doc = "Register `AGTIOSEL` reader"]
pub type R = crate::R<AgtioselSpec>;
#[doc = "Register `AGTIOSEL` writer"]
pub type W = crate::W<AgtioselSpec>;
#[doc = "AGTIO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Select the AGTIOn except for below pins"]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    _10 = 2,
    #[doc = "3: Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    _11 = 3,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - AGTIO pin select"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::_00,
            1 => Sel::_01,
            2 => Sel::_10,
            3 => Sel::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Select the AGTIOn except for below pins"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sel::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sel::_01
    }
    #[doc = "Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sel::_10
    }
    #[doc = "Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sel::_11
    }
}
#[doc = "Field `SEL` writer - AGTIO pin select"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the AGTIOn except for below pins"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_01)
    }
    #[doc = "Select the P402/AGTIOn. P402/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_10)
    }
    #[doc = "Select the P403/AGTIOn. P403/AGTIOn is input only. It is not possible to output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_11)
    }
}
#[doc = "AGTIO input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ties {
    #[doc = "0: External event input is disabled during Software Standby mode"]
    _0 = 0,
    #[doc = "1: External event input is enabled during Software Standby mode."]
    _1 = 1,
}
impl From<Ties> for bool {
    #[inline(always)]
    fn from(variant: Ties) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIES` reader - AGTIO input enable"]
pub type TiesR = crate::BitReader<Ties>;
impl TiesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ties {
        match self.bits {
            false => Ties::_0,
            true => Ties::_1,
        }
    }
    #[doc = "External event input is disabled during Software Standby mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ties::_0
    }
    #[doc = "External event input is enabled during Software Standby mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ties::_1
    }
}
#[doc = "Field `TIES` writer - AGTIO input enable"]
pub type TiesW<'a, REG> = crate::BitWriter<'a, REG, Ties>;
impl<'a, REG> TiesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External event input is disabled during Software Standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ties::_0)
    }
    #[doc = "External event input is enabled during Software Standby mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ties::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(self.bits & 3)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    pub fn ties(&self) -> TiesR {
        TiesR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTIOSEL")
            .field("ties", &self.ties())
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - AGTIO pin select"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<AgtioselSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bit 4 - AGTIO input enable"]
    #[inline(always)]
    pub fn ties(&mut self) -> TiesW<AgtioselSpec> {
        TiesW::new(self, 4)
    }
}
#[doc = "AGT Pin Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtiosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtiosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtioselSpec;
impl crate::RegisterSpec for AgtioselSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtiosel::R`](R) reader structure"]
impl crate::Readable for AgtioselSpec {}
#[doc = "`write(|w| ..)` method takes [`agtiosel::W`](W) writer structure"]
impl crate::Writable for AgtioselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTIOSEL to value 0"]
impl crate::Resettable for AgtioselSpec {}

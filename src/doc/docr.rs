#[doc = "Register `DOCR` reader"]
pub type R = crate::R<DocrSpec>;
#[doc = "Register `DOCR` writer"]
pub type W = crate::W<DocrSpec>;
#[doc = "Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oms {
    #[doc = "0: Data comparison mode"]
    _00 = 0,
    #[doc = "1: Data addition mode"]
    _01 = 1,
    #[doc = "2: Data subtraction mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Oms> for u8 {
    #[inline(always)]
    fn from(variant: Oms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oms {
    type Ux = u8;
}
impl crate::IsEnum for Oms {}
#[doc = "Field `OMS` reader - Operating Mode Select"]
pub type OmsR = crate::FieldReader<Oms>;
impl OmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oms {
        match self.bits {
            0 => Oms::_00,
            1 => Oms::_01,
            2 => Oms::_10,
            3 => Oms::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oms::_00
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oms::_01
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oms::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oms::_11
    }
}
#[doc = "Field `OMS` writer - Operating Mode Select"]
pub type OmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oms, crate::Safe>;
impl<'a, REG> OmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_00)
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_01)
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oms::_11)
    }
}
#[doc = "Detection Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcsel {
    #[doc = "0: DOPCF is set when data mismatch is detected."]
    _0 = 0,
    #[doc = "1: DOPCF is set when data match is detected."]
    _1 = 1,
}
impl From<Dcsel> for bool {
    #[inline(always)]
    fn from(variant: Dcsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCSEL` reader - Detection Condition Select"]
pub type DcselR = crate::BitReader<Dcsel>;
impl DcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcsel {
        match self.bits {
            false => Dcsel::_0,
            true => Dcsel::_1,
        }
    }
    #[doc = "DOPCF is set when data mismatch is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcsel::_0
    }
    #[doc = "DOPCF is set when data match is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcsel::_1
    }
}
#[doc = "Field `DCSEL` writer - Detection Condition Select"]
pub type DcselW<'a, REG> = crate::BitWriter<'a, REG, Dcsel>;
impl<'a, REG> DcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOPCF is set when data mismatch is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_0)
    }
    #[doc = "DOPCF is set when data match is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcsel::_1)
    }
}
#[doc = "Field `DOPCF` reader - Data Operation Circuit Flag Indicates the result of an operation."]
pub type DopcfR = crate::BitReader;
#[doc = "DOPCF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dopcfcl {
    #[doc = "0: Maintains the DOPCF flag state."]
    _0 = 0,
    #[doc = "1: Clears the DOPCF flag."]
    _1 = 1,
}
impl From<Dopcfcl> for bool {
    #[inline(always)]
    fn from(variant: Dopcfcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOPCFCL` reader - DOPCF Clear"]
pub type DopcfclR = crate::BitReader<Dopcfcl>;
impl DopcfclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dopcfcl {
        match self.bits {
            false => Dopcfcl::_0,
            true => Dopcfcl::_1,
        }
    }
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dopcfcl::_0
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dopcfcl::_1
    }
}
#[doc = "Field `DOPCFCL` writer - DOPCF Clear"]
pub type DopcfclW<'a, REG> = crate::BitWriter<'a, REG, Dopcfcl>;
impl<'a, REG> DopcfclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dopcfcl::_0)
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dopcfcl::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&self) -> OmsR {
        OmsR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&self) -> DcselR {
        DcselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Operation Circuit Flag Indicates the result of an operation."]
    #[inline(always)]
    pub fn dopcf(&self) -> DopcfR {
        DopcfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOPCF Clear"]
    #[inline(always)]
    pub fn dopcfcl(&self) -> DopcfclR {
        DopcfclR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOCR")
            .field("dopcfcl", &self.dopcfcl())
            .field("dopcf", &self.dopcf())
            .field("dcsel", &self.dcsel())
            .field("oms", &self.oms())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&mut self) -> OmsW<DocrSpec> {
        OmsW::new(self, 0)
    }
    #[doc = "Bit 2 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&mut self) -> DcselW<DocrSpec> {
        DcselW::new(self, 2)
    }
    #[doc = "Bit 6 - DOPCF Clear"]
    #[inline(always)]
    pub fn dopcfcl(&mut self) -> DopcfclW<DocrSpec> {
        DopcfclW::new(self, 6)
    }
}
#[doc = "DOC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`docr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DocrSpec;
impl crate::RegisterSpec for DocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`docr::R`](R) reader structure"]
impl crate::Readable for DocrSpec {}
#[doc = "`write(|w| ..)` method takes [`docr::W`](W) writer structure"]
impl crate::Writable for DocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOCR to value 0"]
impl crate::Resettable for DocrSpec {}

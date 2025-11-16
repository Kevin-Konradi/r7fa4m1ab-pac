#[doc = "Register `GTBER` reader"]
pub type R = crate::R<GtberSpec>;
#[doc = "Register `GTBER` writer"]
pub type W = crate::W<GtberSpec>;
#[doc = "BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bd {
    #[doc = "0: Buffer operation is enabled"]
    _0 = 0,
    #[doc = "1: Buffer operation is disabled"]
    _1 = 1,
}
impl From<Bd> for u8 {
    #[inline(always)]
    fn from(variant: Bd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bd {
    type Ux = u8;
}
impl crate::IsEnum for Bd {}
#[doc = "Field `BD` reader - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
pub type BdR = crate::FieldReader<Bd>;
impl BdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bd> {
        match self.bits {
            0 => Some(Bd::_0),
            1 => Some(Bd::_1),
            _ => None,
        }
    }
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bd::_0
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bd::_1
    }
}
#[doc = "Field `BD` writer - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
pub type BdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bd>;
impl<'a, REG> BdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer operation is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bd::_0)
    }
    #[doc = "Buffer operation is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bd::_1)
    }
}
#[doc = "GTCCRA Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccra {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRA <--> GTCCRC)"]
    _01 = 1,
    #[doc = "2: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _10 = 2,
    #[doc = "3: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    _11 = 3,
}
impl From<Ccra> for u8 {
    #[inline(always)]
    fn from(variant: Ccra) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccra {
    type Ux = u8;
}
impl crate::IsEnum for Ccra {}
#[doc = "Field `CCRA` reader - GTCCRA Buffer Operation"]
pub type CcraR = crate::FieldReader<Ccra>;
impl CcraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccra {
        match self.bits {
            0 => Ccra::_00,
            1 => Ccra::_01,
            2 => Ccra::_10,
            3 => Ccra::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ccra::_00
    }
    #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ccra::_01
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ccra::_10
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ccra::_11
    }
}
#[doc = "Field `CCRA` writer - GTCCRA Buffer Operation"]
pub type CcraW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccra, crate::Safe>;
impl<'a, REG> CcraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_00)
    }
    #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_01)
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_10)
    }
    #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ccra::_11)
    }
}
#[doc = "GTCCRB Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccrb {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTCCRB <--> GTCCRE)"]
    _01 = 1,
    #[doc = "2: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _10 = 2,
    #[doc = "3: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    _11 = 3,
}
impl From<Ccrb> for u8 {
    #[inline(always)]
    fn from(variant: Ccrb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccrb {
    type Ux = u8;
}
impl crate::IsEnum for Ccrb {}
#[doc = "Field `CCRB` reader - GTCCRB Buffer Operation"]
pub type CcrbR = crate::FieldReader<Ccrb>;
impl CcrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccrb {
        match self.bits {
            0 => Ccrb::_00,
            1 => Ccrb::_01,
            2 => Ccrb::_10,
            3 => Ccrb::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ccrb::_00
    }
    #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ccrb::_01
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ccrb::_10
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ccrb::_11
    }
}
#[doc = "Field `CCRB` writer - GTCCRB Buffer Operation"]
pub type CcrbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccrb, crate::Safe>;
impl<'a, REG> CcrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_00)
    }
    #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_01)
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_10)
    }
    #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrb::_11)
    }
}
#[doc = "GTPR Buffer Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pr {
    #[doc = "0: Buffer operation is not performed"]
    _00 = 0,
    #[doc = "1: Single buffer operation (GTPBR --> GTPR)"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    Others = 2,
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(variant: Pr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pr {
    type Ux = u8;
}
impl crate::IsEnum for Pr {}
#[doc = "Field `PR` reader - GTPR Buffer Operation"]
pub type PrR = crate::FieldReader<Pr>;
impl PrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pr {
        match self.bits {
            0 => Pr::_00,
            1 => Pr::_01,
            _ => Pr::Others,
        }
    }
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pr::_00
    }
    #[doc = "Single buffer operation (GTPBR --> GTPR)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pr::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pr::Others)
    }
}
#[doc = "Field `PR` writer - GTPR Buffer Operation"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pr, crate::Safe>;
impl<'a, REG> PrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer operation is not performed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::_00)
    }
    #[doc = "Single buffer operation (GTPBR --> GTPR)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pr::Others)
    }
}
#[doc = "GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccrswt {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
    _1 = 1,
}
impl From<Ccrswt> for bool {
    #[inline(always)]
    fn from(variant: Ccrswt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRSWT` writer - GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
pub type CcrswtW<'a, REG> = crate::BitWriter<'a, REG, Ccrswt>;
impl<'a, REG> CcrswtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrswt::_0)
    }
    #[doc = "Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccrswt::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd(&self) -> BdR {
        BdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(&self) -> CcraR {
        CcraR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(&self) -> CcrbR {
        CcrbR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTBER")
            .field("pr", &self.pr())
            .field("ccrb", &self.ccrb())
            .field("ccra", &self.ccra())
            .field("bd", &self.bd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - BD\\[1\\]: GTPR Buffer Operation Disable BD\\[0\\]: GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd(&mut self) -> BdW<GtberSpec> {
        BdW::new(self, 0)
    }
    #[doc = "Bits 16:17 - GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(&mut self) -> CcraW<GtberSpec> {
        CcraW::new(self, 16)
    }
    #[doc = "Bits 18:19 - GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(&mut self) -> CcrbW<GtberSpec> {
        CcrbW::new(self, 18)
    }
    #[doc = "Bits 20:21 - GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<GtberSpec> {
        PrW::new(self, 20)
    }
    #[doc = "Bit 22 - GTCCRA and GTCCRB Forcible Buffer Operation This bit is read as 0."]
    #[inline(always)]
    pub fn ccrswt(&mut self) -> CcrswtW<GtberSpec> {
        CcrswtW::new(self, 22)
    }
}
#[doc = "General PWM Timer Buffer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtberSpec;
impl crate::RegisterSpec for GtberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtber::R`](R) reader structure"]
impl crate::Readable for GtberSpec {}
#[doc = "`write(|w| ..)` method takes [`gtber::W`](W) writer structure"]
impl crate::Writable for GtberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTBER to value 0"]
impl crate::Resettable for GtberSpec {}

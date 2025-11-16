#[doc = "Register `ADCSR` reader"]
pub type R = crate::R<AdcsrSpec>;
#[doc = "Register `ADCSR` writer"]
pub type W = crate::W<AdcsrSpec>;
#[doc = "Field `DBLANS` reader - Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
pub type DblansR = crate::FieldReader;
#[doc = "Field `DBLANS` writer - Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
pub type DblansW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Group B Scan End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gbadie {
    #[doc = "0: Disables S12GBADI0 interrupt generation upon group B scan completion."]
    _0 = 0,
    #[doc = "1: Enables S12GBADI0 interrupt generation upon group B scan completion."]
    _1 = 1,
}
impl From<Gbadie> for bool {
    #[inline(always)]
    fn from(variant: Gbadie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GBADIE` reader - Group B Scan End Interrupt Enable"]
pub type GbadieR = crate::BitReader<Gbadie>;
impl GbadieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gbadie {
        match self.bits {
            false => Gbadie::_0,
            true => Gbadie::_1,
        }
    }
    #[doc = "Disables S12GBADI0 interrupt generation upon group B scan completion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gbadie::_0
    }
    #[doc = "Enables S12GBADI0 interrupt generation upon group B scan completion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gbadie::_1
    }
}
#[doc = "Field `GBADIE` writer - Group B Scan End Interrupt Enable"]
pub type GbadieW<'a, REG> = crate::BitWriter<'a, REG, Gbadie>;
impl<'a, REG> GbadieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables S12GBADI0 interrupt generation upon group B scan completion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gbadie::_0)
    }
    #[doc = "Enables S12GBADI0 interrupt generation upon group B scan completion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gbadie::_1)
    }
}
#[doc = "Double Trigger Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dble {
    #[doc = "0: Double trigger mode non-selection"]
    _0 = 0,
    #[doc = "1: Double trigger mode selection"]
    _1 = 1,
}
impl From<Dble> for bool {
    #[inline(always)]
    fn from(variant: Dble) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLE` reader - Double Trigger Mode Select"]
pub type DbleR = crate::BitReader<Dble>;
impl DbleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dble {
        match self.bits {
            false => Dble::_0,
            true => Dble::_1,
        }
    }
    #[doc = "Double trigger mode non-selection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dble::_0
    }
    #[doc = "Double trigger mode selection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dble::_1
    }
}
#[doc = "Field `DBLE` writer - Double Trigger Mode Select"]
pub type DbleW<'a, REG> = crate::BitWriter<'a, REG, Dble>;
impl<'a, REG> DbleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Double trigger mode non-selection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dble::_0)
    }
    #[doc = "Double trigger mode selection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dble::_1)
    }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrg {
    #[doc = "0: A/D conversion is started by the synchronous trigger (ELC)."]
    _0 = 0,
    #[doc = "1: A/D conversion is started by the asynchronous trigger (ADTRG0#)."]
    _1 = 1,
}
impl From<Extrg> for bool {
    #[inline(always)]
    fn from(variant: Extrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTRG` reader - Trigger Select"]
pub type ExtrgR = crate::BitReader<Extrg>;
impl ExtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extrg {
        match self.bits {
            false => Extrg::_0,
            true => Extrg::_1,
        }
    }
    #[doc = "A/D conversion is started by the synchronous trigger (ELC)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Extrg::_0
    }
    #[doc = "A/D conversion is started by the asynchronous trigger (ADTRG0#)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Extrg::_1
    }
}
#[doc = "Field `EXTRG` writer - Trigger Select"]
pub type ExtrgW<'a, REG> = crate::BitWriter<'a, REG, Extrg>;
impl<'a, REG> ExtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A/D conversion is started by the synchronous trigger (ELC)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Extrg::_0)
    }
    #[doc = "A/D conversion is started by the asynchronous trigger (ADTRG0#)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Extrg::_1)
    }
}
#[doc = "Trigger Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trge {
    #[doc = "0: Disables A/D conversion to be started by the synchronous or asynchronous trigger."]
    _0 = 0,
    #[doc = "1: Enables A/D conversion to be started by the synchronous or asynchronous trigger."]
    _1 = 1,
}
impl From<Trge> for bool {
    #[inline(always)]
    fn from(variant: Trge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGE` reader - Trigger Start Enable"]
pub type TrgeR = crate::BitReader<Trge>;
impl TrgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trge {
        match self.bits {
            false => Trge::_0,
            true => Trge::_1,
        }
    }
    #[doc = "Disables A/D conversion to be started by the synchronous or asynchronous trigger."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trge::_0
    }
    #[doc = "Enables A/D conversion to be started by the synchronous or asynchronous trigger."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trge::_1
    }
}
#[doc = "Field `TRGE` writer - Trigger Start Enable"]
pub type TrgeW<'a, REG> = crate::BitWriter<'a, REG, Trge>;
impl<'a, REG> TrgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables A/D conversion to be started by the synchronous or asynchronous trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trge::_0)
    }
    #[doc = "Enables A/D conversion to be started by the synchronous or asynchronous trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trge::_1)
    }
}
#[doc = "A/D Conversion Operation Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adhsc {
    #[doc = "0: High speed A/D conversion mode"]
    _0 = 0,
    #[doc = "1: Low current A/D conversion mode"]
    _1 = 1,
}
impl From<Adhsc> for bool {
    #[inline(always)]
    fn from(variant: Adhsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADHSC` reader - A/D Conversion Operation Mode Select"]
pub type AdhscR = crate::BitReader<Adhsc>;
impl AdhscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adhsc {
        match self.bits {
            false => Adhsc::_0,
            true => Adhsc::_1,
        }
    }
    #[doc = "High speed A/D conversion mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adhsc::_0
    }
    #[doc = "Low current A/D conversion mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adhsc::_1
    }
}
#[doc = "Field `ADHSC` writer - A/D Conversion Operation Mode Select"]
pub type AdhscW<'a, REG> = crate::BitWriter<'a, REG, Adhsc>;
impl<'a, REG> AdhscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed A/D conversion mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::_0)
    }
    #[doc = "Low current A/D conversion mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::_1)
    }
}
#[doc = "Scan Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcs {
    #[doc = "0: Single scan mode"]
    _00 = 0,
    #[doc = "1: Group scan mode"]
    _01 = 1,
    #[doc = "2: Continuous scan mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Adcs> for u8 {
    #[inline(always)]
    fn from(variant: Adcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcs {
    type Ux = u8;
}
impl crate::IsEnum for Adcs {}
#[doc = "Field `ADCS` reader - Scan Mode Select"]
pub type AdcsR = crate::FieldReader<Adcs>;
impl AdcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcs {
        match self.bits {
            0 => Adcs::_00,
            1 => Adcs::_01,
            2 => Adcs::_10,
            3 => Adcs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Single scan mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adcs::_00
    }
    #[doc = "Group scan mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adcs::_01
    }
    #[doc = "Continuous scan mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adcs::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adcs::_11
    }
}
#[doc = "Field `ADCS` writer - Scan Mode Select"]
pub type AdcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcs, crate::Safe>;
impl<'a, REG> AdcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single scan mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_00)
    }
    #[doc = "Group scan mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_01)
    }
    #[doc = "Continuous scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcs::_11)
    }
}
#[doc = "A/D Conversion Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adst {
    #[doc = "0: Stops A/D conversion process."]
    _0 = 0,
    #[doc = "1: Starts A/D conversion process."]
    _1 = 1,
}
impl From<Adst> for bool {
    #[inline(always)]
    fn from(variant: Adst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADST` reader - A/D Conversion Start\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type AdstR = crate::BitReader<Adst>;
impl AdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adst {
        match self.bits {
            false => Adst::_0,
            true => Adst::_1,
        }
    }
    #[doc = "Stops A/D conversion process."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adst::_0
    }
    #[doc = "Starts A/D conversion process."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adst::_1
    }
}
#[doc = "Field `ADST` writer - A/D Conversion Start"]
pub type AdstW<'a, REG> = crate::BitWriter<'a, REG, Adst>;
impl<'a, REG> AdstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops A/D conversion process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adst::_0)
    }
    #[doc = "Starts A/D conversion process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adst::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
    #[inline(always)]
    pub fn dblans(&self) -> DblansR {
        DblansR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Group B Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn gbadie(&self) -> GbadieR {
        GbadieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Double Trigger Mode Select"]
    #[inline(always)]
    pub fn dble(&self) -> DbleR {
        DbleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger Select"]
    #[inline(always)]
    pub fn extrg(&self) -> ExtrgR {
        ExtrgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Trigger Start Enable"]
    #[inline(always)]
    pub fn trge(&self) -> TrgeR {
        TrgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D Conversion Operation Mode Select"]
    #[inline(always)]
    pub fn adhsc(&self) -> AdhscR {
        AdhscR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Scan Mode Select"]
    #[inline(always)]
    pub fn adcs(&self) -> AdcsR {
        AdcsR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - A/D Conversion Start"]
    #[inline(always)]
    pub fn adst(&self) -> AdstR {
        AdstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSR")
            .field("adcs", &self.adcs())
            .field("adhsc", &self.adhsc())
            .field("trge", &self.trge())
            .field("extrg", &self.extrg())
            .field("dble", &self.dble())
            .field("gbadie", &self.gbadie())
            .field("dblans", &self.dblans())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Double Trigger Channel Select These bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
    #[inline(always)]
    pub fn dblans(&mut self) -> DblansW<AdcsrSpec> {
        DblansW::new(self, 0)
    }
    #[doc = "Bit 6 - Group B Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn gbadie(&mut self) -> GbadieW<AdcsrSpec> {
        GbadieW::new(self, 6)
    }
    #[doc = "Bit 7 - Double Trigger Mode Select"]
    #[inline(always)]
    pub fn dble(&mut self) -> DbleW<AdcsrSpec> {
        DbleW::new(self, 7)
    }
    #[doc = "Bit 8 - Trigger Select"]
    #[inline(always)]
    pub fn extrg(&mut self) -> ExtrgW<AdcsrSpec> {
        ExtrgW::new(self, 8)
    }
    #[doc = "Bit 9 - Trigger Start Enable"]
    #[inline(always)]
    pub fn trge(&mut self) -> TrgeW<AdcsrSpec> {
        TrgeW::new(self, 9)
    }
    #[doc = "Bit 10 - A/D Conversion Operation Mode Select"]
    #[inline(always)]
    pub fn adhsc(&mut self) -> AdhscW<AdcsrSpec> {
        AdhscW::new(self, 10)
    }
    #[doc = "Bits 13:14 - Scan Mode Select"]
    #[inline(always)]
    pub fn adcs(&mut self) -> AdcsW<AdcsrSpec> {
        AdcsW::new(self, 13)
    }
    #[doc = "Bit 15 - A/D Conversion Start"]
    #[inline(always)]
    pub fn adst(&mut self) -> AdstW<AdcsrSpec> {
        AdstW::new(self, 15)
    }
}
#[doc = "A/D Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcsrSpec;
impl crate::RegisterSpec for AdcsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcsr::R`](R) reader structure"]
impl crate::Readable for AdcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsr::W`](W) writer structure"]
impl crate::Writable for AdcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCSR to value 0"]
impl crate::Resettable for AdcsrSpec {}

#[doc = "Register `OPSCR` reader"]
pub type R = crate::R<OpscrSpec>;
#[doc = "Register `OPSCR` writer"]
pub type W = crate::W<OpscrSpec>;
#[doc = "Field `UF` reader - Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VF` reader - Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type VfR = crate::BitReader;
#[doc = "Field `VF` writer - Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type VfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WF` reader - Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type WfR = crate::BitReader;
#[doc = "Field `WF` writer - Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
pub type WfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U` reader - Input U-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type UR = crate::BitReader;
#[doc = "Field `V` reader - Input V-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type VR = crate::BitReader;
#[doc = "Field `W` reader - Input W-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type WR = crate::BitReader;
#[doc = "Enable-Phase Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Not Output(Hi-Z external terminals)."]
    _0 = 0,
    #[doc = "1: Output"]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable-Phase Output Control"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::_0,
            true => En::_1,
        }
    }
    #[doc = "Not Output(Hi-Z external terminals)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - Enable-Phase Output Control"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Output(Hi-Z external terminals)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
#[doc = "External Feedback Signal Enable This bit selects the input phase from the software settings and external input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fb {
    #[doc = "0: Select the external input."]
    _0 = 0,
    #[doc = "1: Select the soft setting(OPSCR.UF, VF, WF)."]
    _1 = 1,
}
impl From<Fb> for bool {
    #[inline(always)]
    fn from(variant: Fb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FB` reader - External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
pub type FbR = crate::BitReader<Fb>;
impl FbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fb {
        match self.bits {
            false => Fb::_0,
            true => Fb::_1,
        }
    }
    #[doc = "Select the external input."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fb::_0
    }
    #[doc = "Select the soft setting(OPSCR.UF, VF, WF)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fb::_1
    }
}
#[doc = "Field `FB` writer - External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG, Fb>;
impl<'a, REG> FbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select the external input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::_0)
    }
    #[doc = "Select the soft setting(OPSCR.UF, VF, WF)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fb::_1)
    }
}
#[doc = "Positive-Phase Output (P) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P {
    #[doc = "0: Level signal output"]
    _0 = 0,
    #[doc = "1: PWM signal output (PWM of GPT0)"]
    _1 = 1,
}
impl From<P> for bool {
    #[inline(always)]
    fn from(variant: P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P` reader - Positive-Phase Output (P) Control"]
pub type PR = crate::BitReader<P>;
impl PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P {
        match self.bits {
            false => P::_0,
            true => P::_1,
        }
    }
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P::_0
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P::_1
    }
}
#[doc = "Field `P` writer - Positive-Phase Output (P) Control"]
pub type PW<'a, REG> = crate::BitWriter<'a, REG, P>;
impl<'a, REG> PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P::_0)
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P::_1)
    }
}
#[doc = "Negative-Phase Output (N) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N {
    #[doc = "0: Level signal output"]
    _0 = 0,
    #[doc = "1: PWM signal output (PWM of GPT0)"]
    _1 = 1,
}
impl From<N> for bool {
    #[inline(always)]
    fn from(variant: N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `N` reader - Negative-Phase Output (N) Control"]
pub type NR = crate::BitReader<N>;
impl NR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N {
        match self.bits {
            false => N::_0,
            true => N::_1,
        }
    }
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == N::_0
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == N::_1
    }
}
#[doc = "Field `N` writer - Negative-Phase Output (N) Control"]
pub type NW<'a, REG> = crate::BitWriter<'a, REG, N>;
impl<'a, REG> NW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(N::_0)
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(N::_1)
    }
}
#[doc = "Invert-Phase Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Positive Logic (Active High)output"]
    _0 = 0,
    #[doc = "1: Negative Logic (Active Low)output"]
    _1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert-Phase Output Control"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::_0,
            true => Inv::_1,
        }
    }
    #[doc = "Positive Logic (Active High)output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inv::_0
    }
    #[doc = "Negative Logic (Active Low)output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inv::_1
    }
}
#[doc = "Field `INV` writer - Invert-Phase Output Control"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive Logic (Active High)output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_0)
    }
    #[doc = "Negative Logic (Active Low)output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_1)
    }
}
#[doc = "Output phase rotation direction reversal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rv {
    #[doc = "0: U/V/W-Phase output"]
    _0 = 0,
    #[doc = "1: Output to reverse the V / W-phase"]
    _1 = 1,
}
impl From<Rv> for bool {
    #[inline(always)]
    fn from(variant: Rv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RV` reader - Output phase rotation direction reversal"]
pub type RvR = crate::BitReader<Rv>;
impl RvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rv {
        match self.bits {
            false => Rv::_0,
            true => Rv::_1,
        }
    }
    #[doc = "U/V/W-Phase output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rv::_0
    }
    #[doc = "Output to reverse the V / W-phase"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rv::_1
    }
}
#[doc = "Field `RV` writer - Output phase rotation direction reversal"]
pub type RvW<'a, REG> = crate::BitWriter<'a, REG, Rv>;
impl<'a, REG> RvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "U/V/W-Phase output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rv::_0)
    }
    #[doc = "Output to reverse the V / W-phase"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rv::_1)
    }
}
#[doc = "Input phase alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    #[doc = "0: Input phase is aligned to PCLK."]
    _0 = 0,
    #[doc = "1: Input phase is aligned PWM."]
    _1 = 1,
}
impl From<Align> for bool {
    #[inline(always)]
    fn from(variant: Align) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Input phase alignment"]
pub type AlignR = crate::BitReader<Align>;
impl AlignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Align {
        match self.bits {
            false => Align::_0,
            true => Align::_1,
        }
    }
    #[doc = "Input phase is aligned to PCLK."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Align::_0
    }
    #[doc = "Input phase is aligned PWM."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Align::_1
    }
}
#[doc = "Field `ALIGN` writer - Input phase alignment"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG, Align>;
impl<'a, REG> AlignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input phase is aligned to PCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Align::_0)
    }
    #[doc = "Input phase is aligned PWM."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Align::_1)
    }
}
#[doc = "Output disabled source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Grp {
    #[doc = "0: Select Group A output disable source"]
    _00 = 0,
    #[doc = "1: Select Group B output disable source"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    Others = 2,
}
impl From<Grp> for u8 {
    #[inline(always)]
    fn from(variant: Grp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Grp {
    type Ux = u8;
}
impl crate::IsEnum for Grp {}
#[doc = "Field `GRP` reader - Output disabled source selection"]
pub type GrpR = crate::FieldReader<Grp>;
impl GrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Grp {
        match self.bits {
            0 => Grp::_00,
            1 => Grp::_01,
            _ => Grp::Others,
        }
    }
    #[doc = "Select Group A output disable source"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Grp::_00
    }
    #[doc = "Select Group B output disable source"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Grp::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Grp::Others)
    }
}
#[doc = "Field `GRP` writer - Output disabled source selection"]
pub type GrpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Grp, crate::Safe>;
impl<'a, REG> GrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select Group A output disable source"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_00)
    }
    #[doc = "Select Group B output disable source"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::Others)
    }
}
#[doc = "Group output disable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Godf {
    #[doc = "0: This bit function is ignored."]
    _0 = 0,
    #[doc = "1: Group disable will clear OPSCR.EN Bit."]
    _1 = 1,
}
impl From<Godf> for bool {
    #[inline(always)]
    fn from(variant: Godf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GODF` reader - Group output disable function"]
pub type GodfR = crate::BitReader<Godf>;
impl GodfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Godf {
        match self.bits {
            false => Godf::_0,
            true => Godf::_1,
        }
    }
    #[doc = "This bit function is ignored."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Godf::_0
    }
    #[doc = "Group disable will clear OPSCR.EN Bit."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Godf::_1
    }
}
#[doc = "Field `GODF` writer - Group output disable function"]
pub type GodfW<'a, REG> = crate::BitWriter<'a, REG, Godf>;
impl<'a, REG> GodfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit function is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Godf::_0)
    }
    #[doc = "Group disable will clear OPSCR.EN Bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Godf::_1)
    }
}
#[doc = "External Input Noise Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    #[doc = "0: Do not use a noise filter to the external input."]
    _0 = 0,
    #[doc = "1: Use a noise filter to the external input."]
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFEN` reader - External Input Noise Filter Enable"]
pub type NfenR = crate::BitReader<Nfen>;
impl NfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfen {
        match self.bits {
            false => Nfen::_0,
            true => Nfen::_1,
        }
    }
    #[doc = "Do not use a noise filter to the external input."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    #[doc = "Use a noise filter to the external input."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
#[doc = "Field `NFEN` writer - External Input Noise Filter Enable"]
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not use a noise filter to the external input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    #[doc = "Use a noise filter to the external input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
#[doc = "External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcs {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(variant: Nfcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcs {
    type Ux = u8;
}
impl crate::IsEnum for Nfcs {}
#[doc = "Field `NFCS` reader - External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
pub type NfcsR = crate::FieldReader<Nfcs>;
impl NfcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfcs {
        match self.bits {
            0 => Nfcs::_00,
            1 => Nfcs::_01,
            2 => Nfcs::_10,
            3 => Nfcs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcs::_00
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcs::_01
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcs::_10
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcs::_11
    }
}
#[doc = "Field `NFCS` writer - External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
pub type NfcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcs, crate::Safe>;
impl<'a, REG> NfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn vf(&self) -> VfR {
        VfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn wf(&self) -> WfR {
        WfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Input U-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input V-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input W-Phase Monitor This bit monitors the state of the input phase. OPSCR.FB=0:External input monitoring by PCLK OPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable-Phase Output Control"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Positive-Phase Output (P) Control"]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Negative-Phase Output (N) Control"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Invert-Phase Output Control"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output phase rotation direction reversal"]
    #[inline(always)]
    pub fn rv(&self) -> RvR {
        RvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input phase alignment"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Output disabled source selection"]
    #[inline(always)]
    pub fn grp(&self) -> GrpR {
        GrpR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Group output disable function"]
    #[inline(always)]
    pub fn godf(&self) -> GodfR {
        GodfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - External Input Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
    #[inline(always)]
    pub fn nfcs(&self) -> NfcsR {
        NfcsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPSCR")
            .field("nfcs", &self.nfcs())
            .field("nfen", &self.nfen())
            .field("godf", &self.godf())
            .field("grp", &self.grp())
            .field("align", &self.align())
            .field("rv", &self.rv())
            .field("inv", &self.inv())
            .field("n", &self.n())
            .field("p", &self.p())
            .field("fb", &self.fb())
            .field("en", &self.en())
            .field("w", &self.w())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("wf", &self.wf())
            .field("vf", &self.vf())
            .field("uf", &self.uf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input Phase Soft Setting WF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<OpscrSpec> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Phase Soft Setting VF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn vf(&mut self) -> VfW<OpscrSpec> {
        VfW::new(self, 1)
    }
    #[doc = "Bit 2 - Input Phase Soft Setting UF This bit sets the input phase by the software settings. This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn wf(&mut self) -> WfW<OpscrSpec> {
        WfW::new(self, 2)
    }
    #[doc = "Bit 8 - Enable-Phase Output Control"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<OpscrSpec> {
        EnW::new(self, 8)
    }
    #[doc = "Bit 16 - External Feedback Signal Enable This bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<OpscrSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bit 17 - Positive-Phase Output (P) Control"]
    #[inline(always)]
    pub fn p(&mut self) -> PW<OpscrSpec> {
        PW::new(self, 17)
    }
    #[doc = "Bit 18 - Negative-Phase Output (N) Control"]
    #[inline(always)]
    pub fn n(&mut self) -> NW<OpscrSpec> {
        NW::new(self, 18)
    }
    #[doc = "Bit 19 - Invert-Phase Output Control"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<OpscrSpec> {
        InvW::new(self, 19)
    }
    #[doc = "Bit 20 - Output phase rotation direction reversal"]
    #[inline(always)]
    pub fn rv(&mut self) -> RvW<OpscrSpec> {
        RvW::new(self, 20)
    }
    #[doc = "Bit 21 - Input phase alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<OpscrSpec> {
        AlignW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Output disabled source selection"]
    #[inline(always)]
    pub fn grp(&mut self) -> GrpW<OpscrSpec> {
        GrpW::new(self, 24)
    }
    #[doc = "Bit 26 - Group output disable function"]
    #[inline(always)]
    pub fn godf(&mut self) -> GodfW<OpscrSpec> {
        GodfW::new(self, 26)
    }
    #[doc = "Bit 29 - External Input Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<OpscrSpec> {
        NfenW::new(self, 29)
    }
    #[doc = "Bits 30:31 - External Input Noise Filter Clock selection Noise filter sampling clock setting of the external input."]
    #[inline(always)]
    pub fn nfcs(&mut self) -> NfcsW<OpscrSpec> {
        NfcsW::new(self, 30)
    }
}
#[doc = "Output Phase Switching Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpscrSpec;
impl crate::RegisterSpec for OpscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opscr::R`](R) reader structure"]
impl crate::Readable for OpscrSpec {}
#[doc = "`write(|w| ..)` method takes [`opscr::W`](W) writer structure"]
impl crate::Writable for OpscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPSCR to value 0"]
impl crate::Resettable for OpscrSpec {}

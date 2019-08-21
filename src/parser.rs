pub enum WordClass {
    Noun(NounSub),
    Prefix(PrefixSub),
    Verb(VerbSub, VerbCType),
    Adjective(AdjectiveSub, AdjectiveCType),
    Adverb(AdverbSub),
    AdnominalAdjective,
    Conjection,
    PostPositinalParticle(PostPositinalParticleSub),
    AuxiliaryVerb(AuxiliaryVerbCType),
    Interjection,
    Symbol(SymbolSub),
    Filler,
    Other(OtherSub),
}

pub enum NounSub {
    S_IrregularInflection,
    NAI_AdjectiveStem,
    General,
    QuoteString,
    AdjectiveStem,
    ProperNoun(ProperNounSub),
    Number,
    Conjectionwise,
    Suffix,
    Pronoun,
    Verb_NonIndependentwise,
    Special,
    NonIndependent,
    Adverbable,
    Other(String),
}

pub enum ProperNounSub {
    General,
    Name(NameSub),
    Organization,
    Area, 
    Other(String),
}

pub enum NameSub {
    General,
    Surname,
    Givenname,
    Other(String),
}

pub enum PrefixSub {
    AdjectiveConjection,
    NumberConjection,
    VerbCojection,
    NounConjection,
    Other(String),
}

pub enum VerbSub {
    Independent,
    Suffix,
    NonIndependent,
    Other(String),
}

pub enum AdjectiveSub {
    Independent,
    Suffix,
    NonIndependent,
    Other(String),
}

pub enum AdverbSub {
    General,
    PostPositinalParticlesConjection,
    Other(String),
}

pub enum PostPositinalParticleSub {
    NominitiveParticle,
    BindingParticle,
    Sentence_EndingParticle,
    ConjectionParticle,
    Special,
    Adverbize,
    AdverbialParticle,
    Adverbial_Parallel_EndingParticle,
    ParallelMarker,
    Adnominalize,
    Other(String),
}

pub enum SymbolSub {
    Alphabet,
    General,
    ParanthesisOpen,
    ParanthesisClose,
    Period,
    Space,
    Comma,
    Other(String),
}

pub enum OtherSub {
    Interjection,
    Other(String),
}

pub enum AdjectiveCType {
    AUO(AdjectiveAUO_CForm),
    II(AdjectiveII_CForm),
    I(AdjectiveI_CForm),
    Constant(Adjective_Constant_CForm),
}

pub enum AdjectiveAUO_CForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Others(String),
}

pub enum AdjectiveII_CForm {
    Normal,
    Others(String),
}

pub enum AdjectiveI_CForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Others(String),
}

pub enum Adjective_Constant_CForm{
    Normal,
    Others(String),
}

pub enum VerbCType {
    KA(Verb_KA_CForm),
    SA_SURU(Verb_SA_SURU_CForm),
    SA_ZURU(Verb_SA_ZURU_CForm),
    RA(Verb_RA_CForm),
    One(Verb_One_CForm),
    One_KURERU(Verb_One_KURERU_CForm),
    Below2_HA(Verb_Below2_HA_CForm),
    Below2_E(Verb_Below2_E_CForm),
    Five_KA(Verb_Five_KA_YUKU_CForm),
    Five_BA(Verb_Five_BA_CForm),
    Five_MA(Verb_Five_MA_CForm),
    Five_RA(Verb_Five_RA_CForm),
    Five_RA_Special(Verb_Five_RA_Special_CForm),
    Five_WA(Verb_Five_WA_CForm),
    Other(String),
}

pub enum Verb_KA_CForm {
    OrderYO,
    Other(String),
}

pub enum Verb_SA_SURU_CForm {
    ImperfectiveRERUConjection,
    Other(String),
}

pub enum Verb_SA_ZURU_CForm {
    Normal,
    WrittenLangNormal,
    OrderYO,
    Other(String),
}

pub enum Verb_RA_CForm {
    SubstantiveConjection,
    Other(String),
}

pub enum Verb_One_CForm {
    Normal,
    SubstantiveConjectionSpecial,
    ImperfectiveUConjection,
    OrderYO,
    Other(String),
}

pub enum Verb_One_KURERU_CForm {
    ImperfectiveSpecial,
    Other(String),
}

pub enum Verb_Below2_HA_CForm {
    SubstantiveConjection,
    Other(String),
}

pub enum Verb_Below2_E_CForm {
    Normal,
    SubstantiveConjection,
    ImperfectiveUConjection,
    OrderYO,
    Other(String),
}

pub enum Verb_Five_KA_YUKU_CForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum Verb_Five_BA_CForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum Verb_Five_MA_CForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum Verb_Five_RA_CForm {
    SubstantiveConjectionSpecial,
    SubstantiveConjectionSpecial2,
    ImperfectiveSpecial,
    Other(String),
}

pub enum Verb_Five_RA_Special_CForm {
    Normal,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum Verb_Five_WA_CForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbCType {
    Below2_TA(AuxiliaryVerb_Below2_TA_CForm),
    Adjective_I(AuxiliaryVerb_Adjective_I_CForm),
    Five_RA_ARU(AuxiliaryVerb_Five_RA_ARU_CForm),
    Five_RA_Special(AuxiliaryVerb_Five_RA_Special_CForm),
    Special_TA(AuxiliaryVerb_Special_TA_CForm),
    Special_TAI(AuxiliaryVerb_Special_TAI_CForm),
    Special_NU(AuxiliaryVerb_Special_NU_CForm),
    Special_MASU(AuxiliaryVerb_Special_MASU_CForm),
    Constant(AuxiliaryVerb_Special_Constant_CForm),
    WrittenLang_KI(AuxiliaryVerb_WrittenLang_KI_CForm),
    WrittenLang_BESHI(AuxiliaryVerb_WrittenLang_BESHI_CForm),
    WrittenLang_RU(AuxiliaryVerb_WrittenLang_RU_CForm),
}

pub enum AuxiliaryVerb_Below2_TA_CForm {
    OrderYO,
    Other(String),
}

pub enum AuxiliaryVerb_Adjective_I_CForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerb_Five_RA_ARU_CForm {
    Normal,
    SubstantiveConjectionSpecial,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerb_Five_RA_Special_CForm {
    Normal,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum AuxiliaryVerb_Special_TA_CForm {
    Normal,
    Other(String),
}

pub enum AuxiliaryVerb_Special_TAI_CForm {
    GARUConjection,
    Normal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerb_Special_NU_CForm {
    SubstantiveConjection,
    WrittenLangNormal,
    Other(String),
}

pub enum AuxiliaryVerb_Special_MASU_CForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerb_Special_Constant_CForm {
    Normal,
    Other(String),
}

pub enum AuxiliaryVerb_WrittenLang_KI_CForm {
    SubstantiveConjection,
    Other(String),
}

pub enum AuxiliaryVerb_WrittenLang_BESHI_CForm {
    Normal,
    SubstantiveConjection,
    Other(String),
}

pub enum AuxiliaryVerb_WrittenLang_RU_CForm {
    OrderYO,
    Other(String),
}

pub struct Word {
    class: WordClass,
    original: String,
    reading: String,
    speaking: String,

    gen_cost: usize,
    matrix_id: usize,
}

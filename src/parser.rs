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
    SIrregularInflection,
    NAIAdjectiveStem,
    General,
    QuoteString,
    AdjectiveStem,
    ProperNoun(ProperNounSub),
    Number,
    Conjectionwise,
    Suffix,
    Pronoun,
    VerbNonIndependentwise,
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
    SentenceEndingParticle,
    ConjectionParticle,
    Special,
    Adverbize,
    AdverbialParticle,
    AdverbialParallelEndingParticle,
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
    AUO(AdjectiveAUOCForm),
    II(AdjectiveIICForm),
    I(AdjectiveICForm),
    Constant(AdjectiveConstantCForm),
}

pub enum AdjectiveAUOCForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AdjectiveIICForm {
    Normal,
    Other(String),
}

pub enum AdjectiveICForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AdjectiveConstantCForm{
    Normal,
    Other(String),
}

pub enum VerbCType {
    KA(VerbKACForm),
    SASURU(VerbSASURUCForm),
    SAZURU(VerbSAZURUCForm),
    RA(VerbRACForm),
    One(VerbOneCForm),
    OneKURERU(VerbOneKURERUCForm),
    Below2HA(VerbBelow2HACForm),
    Below2E(VerbBelow2ECForm),
    FiveKA(VerbFiveKAYUKUCForm),
    FiveBA(VerbFiveBACForm),
    FiveMA(VerbFiveMACForm),
    FiveRA(VerbFiveRACForm),
    FiveRASpecial(VerbFiveRASpecialCForm),
    FiveWA(VerbFiveWACForm),
    Other(String),
}

pub enum VerbKACForm {
    OrderYO,
    Other(String),
}

pub enum VerbSASURUCForm {
    ImperfectiveRERUConjection,
    Other(String),
}

pub enum VerbSAZURUCForm {
    Normal,
    WrittenLangNormal,
    OrderYO,
    Other(String),
}

pub enum VerbRACForm {
    SubstantiveConjection,
    Other(String),
}

pub enum VerbOneCForm {
    Normal,
    SubstantiveConjectionSpecial,
    ImperfectiveUConjection,
    OrderYO,
    Other(String),
}

pub enum VerbOneKURERUCForm {
    ImperfectiveSpecial,
    Other(String),
}

pub enum VerbBelow2HACForm {
    SubstantiveConjection,
    Other(String),
}

pub enum VerbBelow2ECForm {
    Normal,
    SubstantiveConjection,
    ImperfectiveUConjection,
    OrderYO,
    Other(String),
}

pub enum VerbFiveKAYUKUCForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum VerbFiveBACForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum VerbFiveMACForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum VerbFiveRACForm {
    SubstantiveConjectionSpecial,
    SubstantiveConjectionSpecial2,
    ImperfectiveSpecial,
    Other(String),
}

pub enum VerbFiveRASpecialCForm {
    Normal,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum VerbFiveWACForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbCType {
    Below2TA(AuxiliaryVerbBelow2TACForm),
    AdjectiveI(AuxiliaryVerbAdjectiveICForm),
    FiveRAARU(AuxiliaryVerbFiveRAARUCForm),
    FiveRASpecial(AuxiliaryVerbFiveRASpecialCForm),
    SpecialTA(AuxiliaryVerbSpecialTACForm),
    SpecialTAI(AuxiliaryVerbSpecialTAICForm),
    SpecialNU(AuxiliaryVerbSpecialNUCForm),
    SpecialMASU(AuxiliaryVerbSpecialMASUCForm),
    Constant(AuxiliaryVerbSpecialConstantCForm),
    WrittenLangKI(AuxiliaryVerbWrittenLangKICForm),
    WrittenLangBESHI(AuxiliaryVerbWrittenLangBESHICForm),
    WrittenLangRU(AuxiliaryVerbWrittenLangRUCForm),
}

pub enum AuxiliaryVerbBelow2TACForm {
    OrderYO,
    Other(String),
}

pub enum AuxiliaryVerbAdjectiveICForm {
    GARUConjection,
    Normal,
    SubstantiveConjection,
    WrittenLangNormal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerbFiveRAARUCForm {
    Normal,
    SubstantiveConjectionSpecial,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbFiveRASpecialCForm {
    Normal,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum AuxiliaryVerbSpecialTACForm {
    Normal,
    Other(String),
}

pub enum AuxiliaryVerbSpecialTAICForm {
    GARUConjection,
    Normal,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerbSpecialNUCForm {
    SubstantiveConjection,
    WrittenLangNormal,
    Other(String),
}

pub enum AuxiliaryVerbSpecialMASUCForm {
    Normal,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbSpecialConstantCForm {
    Normal,
    Other(String),
}

pub enum AuxiliaryVerbWrittenLangKICForm {
    SubstantiveConjection,
    Other(String),
}

pub enum AuxiliaryVerbWrittenLangBESHICForm {
    Normal,
    SubstantiveConjection,
    Other(String),
}

pub enum AuxiliaryVerbWrittenLangRUCForm {
    OrderYO,
    Other(String),
}

#[allow(dead_code)]
pub struct Word {
    class: WordClass,
    original: String,
    reading: String,
    speaking: String,

    gencost: usize,
    matrix_id: usize,
}

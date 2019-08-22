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
    Suffix(SuffixSub),
    Pronoun(PronounSub),
    VerbNonIndependentwise,
    Special(SpecialSub),
    NonIndependent(NonIndependentSub),
    Adverbable,
    Other(String),
}

pub enum ProperNounSub {
    General,
    Name(NameSub),
    Organization,
    Area(AreaSub), 
    Other(String),
}

pub enum NameSub {
    General,
    Surname,
    Givenname,
    Other(String),
}

pub enum AreaSub {
    General,
    Nation,
    Other(String),
}

pub enum SuffixSub {
    SIrregularInflection,
    General,
    AdjectiveStem,
    CounterSuffix,
    CounterSuffixStem,
    Name,
    Area,
    Special,
    Adverbable,
    Other(String),
}

pub enum PronounSub {
    General,
    Contraction,
    Other(String),
}

pub enum SpecialSub {
    AuxiliaryVerbStem,
    Other(String),
}

pub enum NonIndependentSub {
    General,
    AdjectiveStem,
    AuxiliaryVerbStem,
    Adverbable,
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

    word: String,
    original: String,
    reading: String,
    speaking: String,

    gencost: i64,
    matrix_id: usize,
}

#[allow(dead_code)]
pub fn parse_naist_jdic_by_line(line: &str) -> Word {
    let arr: Vec<&str> = line.split(',').collect();
    let matrix_id: usize = arr[1].parse().unwrap();
    let matrix_id2: usize = arr[1].parse().unwrap();
    // DEBUG
    if matrix_id != matrix_id2 {
        println!("{:?}", line);
    }
    let gencost: i64 = arr[3].parse().unwrap();
    let class: WordClass = match arr[4] {
        "名詞" => WordClass::Noun(match arr[5] {
            "サ変接続" => NounSub::SIrregularInflection,
            "ナイ形容詞語幹" => NounSub::NAIAdjectiveStem,
            "一般" => NounSub::General,
            "引用文字列" => NounSub::QuoteString,
            "形容動詞語幹" => NounSub::AdjectiveStem,
            "固有名詞" => NounSub::ProperNoun(match arr[5] {
                "一般" => ProperNounSub::General,
                "人名" => ProperNounSub::Name(match arr[6] {
                    "一般" => NameSub::General,
                    "姓" => NameSub::Surname,
                    "名" => NameSub::Givenname,
                    other => NameSub::Other(other.to_string()),
                }),
                "組織" => ProperNounSub::Organization,
                "地域" => ProperNounSub::Area(match arr[6] {
                    "一般" => AreaSub::General,
                    "国" => AreaSub::Nation,
                    other => AreaSub::Other(other.to_string()),
                })
            }),
            "数" => NounSub::Number,
            "接続詞的" => NounSub::Conjectionwise,
            "接尾" => NounSub::Suffix(match arr[5] {
                "サ変接続" => SuffixSub::SIrregularInflection,
                "一般" => SuffixSub::General,
                "形容動詞語幹" => SuffixSub::AdjectiveStem,
                "助数詞" => SuffixSub::CounterSuffix,
                "助数詞語幹" => SuffixSub::CounterSuffixStem,
                "人名" => SuffixSub::Name,
                "地域" => SuffixSub::Area,
                "特殊" => SuffixSub::Special,
                "副詞可能" => SuffixSub::Adverbable,
                other => SuffixSub::Other(other.to_string()),
            }),
            "代名詞" => NounSub::Pronoun(match arr[5] {
                "一般" => PronounSub::General,
                "縮約" => PronounSub::Contraction,
                other => PronounSub::Other(other.to_string()),
            }),
            "動詞非自立的" => NounSub::VerbNonIndependentwise,
            "特殊" => NounSub::Special(match arr[5] {
                "助動詞語幹" => SpecialSub::AuxiliaryVerbStem,
                other => SpecialSub::Other(other.to_string()),
            }),
            "非自立" => NounSub::NonIndependent(match arr[5] {
                "一般" => NonIndependentSub::General,
                "形容動詞語幹" => NonIndependentSub::AdjectiveStem,
                "助動詞語幹" => NonIndependentSub::AuxiliaryVerbStem,
                "副詞可能" => NonIndependentSub::Adverbable,
                other => NonIndependentSub::Other(other.to_string()),
            }),
            "副詞可能" => NounSub::Adverbable,
            other => NounSub::Other(other.to_string()),
        }),
        _ => WordClass::Other(OtherSub::Other(String::new())),
    };
    Word {
        class,

        word: arr[0].to_string(),
        original: arr[10].to_string(),
        reading: arr[11].to_string(),
        speaking: arr[12].to_string(),

        gencost,
        matrix_id,
    }
}

#[allow(dead_code)]
pub struct DictCfg {
    matrixid: usize,
    gencost: usize,
    surface: usize,
    wordclass: usize,
    wordsubclass: usize,
    wordsubsubclass: usize,
    wordsubsubsubclass: usize,
    ctype: usize,
    cform: usize,
    original: usize,
    reading: usize,
    speaking: usize,
}

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
    Other(String),
}

pub enum AdjectiveAUOCForm {
    GARUConjection,
    Basic,
    SubstantiveConjection,
    WrittenLangBasic,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AdjectiveIICForm {
    Basic,
    Other(String),
}

pub enum AdjectiveICForm {
    GARUConjection,
    Basic,
    SubstantiveConjection,
    WrittenLangBasic,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AdjectiveConstantCForm{
    Basic,
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
    Basic,
    WrittenLangBasic,
    OrderYO,
    Other(String),
}

pub enum VerbRACForm {
    SubstantiveConjection,
    Other(String),
}

pub enum VerbOneCForm {
    Basic,
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
    Basic,
    SubstantiveConjection,
    ImperfectiveUConjection,
    OrderYO,
    Other(String),
}

pub enum VerbFiveKAYUKUCForm {
    Basic,
    ImperfectiveUConjection,
    Other(String),
}

pub enum VerbFiveBACForm {
    Basic,
    ImperfectiveUConjection,
    Other(String),
}

pub enum VerbFiveMACForm {
    Basic,
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
    Basic,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum VerbFiveWACForm {
    Basic,
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
    Basic,
    SubstantiveConjection,
    WrittenLangBasic,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerbFiveRAARUCForm {
    Basic,
    SubstantiveConjectionSpecial,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbFiveRASpecialCForm {
    Basic,
    ImperfectiveUConjection,
    ImperfectiveSpecial,
    Other(String),
}

pub enum AuxiliaryVerbSpecialTACForm {
    Basic,
    Other(String),
}

pub enum AuxiliaryVerbSpecialTAICForm {
    GARUConjection,
    Basic,
    PredicativeGOZAIConjection,
    Other(String),
}

pub enum AuxiliaryVerbSpecialNUCForm {
    SubstantiveConjection,
    WrittenLangBasic,
    Other(String),
}

pub enum AuxiliaryVerbSpecialMASUCForm {
    Basic,
    ImperfectiveUConjection,
    Other(String),
}

pub enum AuxiliaryVerbSpecialConstantCForm {
    Basic,
    Other(String),
}

pub enum AuxiliaryVerbWrittenLangKICForm {
    SubstantiveConjection,
    Other(String),
}

pub enum AuxiliaryVerbWrittenLangBESHICForm {
    Basic,
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
pub fn parse_naist_jdic_by_line(cfg: DictCfg, line: &str) -> Word {
    let arr: Vec<&str> = line.split(',').collect();
    let matrix_id: usize = arr[cfg.matrixid].parse().unwrap();
    let matrix_id2: usize = arr[cfg.matrixid].parse().unwrap();
    // DEBUG
    if matrix_id != matrix_id2 {
        println!("{:?}", line);
    }
    let gencost: i64 = arr[cfg.gencost].parse().unwrap();
    let class: WordClass = match arr[cfg.wordclass] {
        "名詞" => WordClass::Noun(match arr[cfg.wordsubclass] {
            "サ変接続" => NounSub::SIrregularInflection,
            "ナイ形容詞語幹" => NounSub::NAIAdjectiveStem,
            "一般" => NounSub::General,
            "引用文字列" => NounSub::QuoteString,
            "形容動詞語幹" => NounSub::AdjectiveStem,
            "固有名詞" => NounSub::ProperNoun(match arr[cfg.wordsubsubclass] {
                "一般" => ProperNounSub::General,
                "人名" => ProperNounSub::Name(match arr[cfg.wordsubsubsubclass] {
                    "一般" => NameSub::General,
                    "姓" => NameSub::Surname,
                    "名" => NameSub::Givenname,
                    other => NameSub::Other(other.to_string()),
                }),
                "組織" => ProperNounSub::Organization,
                "地域" => ProperNounSub::Area(match arr[cfg.wordsubsubsubclass] {
                    "一般" => AreaSub::General,
                    "国" => AreaSub::Nation,
                    other => AreaSub::Other(other.to_string()),
                }),
                other => ProperNounSub::Other(other.to_string())
            }),
            "数" => NounSub::Number,
            "接続詞的" => NounSub::Conjectionwise,
            "接尾" => NounSub::Suffix(match arr[cfg.wordsubclass] {
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
            "代名詞" => NounSub::Pronoun(match arr[cfg.wordsubclass] {
                "一般" => PronounSub::General,
                "縮約" => PronounSub::Contraction,
                other => PronounSub::Other(other.to_string()),
            }),
            "動詞非自立的" => NounSub::VerbNonIndependentwise,
            "特殊" => NounSub::Special(match arr[cfg.wordsubclass] {
                "助動詞語幹" => SpecialSub::AuxiliaryVerbStem,
                other => SpecialSub::Other(other.to_string()),
            }),
            "非自立" => NounSub::NonIndependent(match arr[cfg.wordsubclass] {
                "一般" => NonIndependentSub::General,
                "形容動詞語幹" => NonIndependentSub::AdjectiveStem,
                "助動詞語幹" => NonIndependentSub::AuxiliaryVerbStem,
                "副詞可能" => NonIndependentSub::Adverbable,
                other => NonIndependentSub::Other(other.to_string()),
            }),
            "副詞可能" => NounSub::Adverbable,
            other => NounSub::Other(other.to_string()),
        }),
        "接頭詞" => WordClass::Prefix(
            match arr[cfg.wordsubclass] {
                "形容詞接続" => PrefixSub::AdjectiveConjection,
                "数接続" => PrefixSub::NumberConjection,
                "動詞接続" => PrefixSub::VerbCojection,
                "名詞接続" => PrefixSub::NounConjection,
                other => PrefixSub::Other(other.to_string()),
            },
        ),
        "動詞" => WordClass::Verb(
            match arr[cfg.wordsubclass] {
                "自立" => VerbSub::Independent,
                "接尾" => VerbSub::Suffix,
                "非自立" => VerbSub::NonIndependent,
                other => VerbSub::Other(other.to_string()),
            },
            match arr[cfg.ctype] {
                "カ変・来ル" => VerbCType::KA(match arr[cfg.cform] {
                    "命令ｙｏ" => VerbKACForm::OrderYO,
                    other => VerbKACForm::Other(other.to_string()),
                }),
                "サ変・-スル" | "サ変・スル" => VerbCType::SASURU(match arr[cfg.cform] {
                    "未然レル接続" => VerbSASURUCForm::ImperfectiveRERUConjection,
                    other => VerbSASURUCForm::Other(other.to_string()),
                }),
                "サ変・-ズル"=> VerbCType::SAZURU(match arr[cfg.cform] {
                    "基本形" => VerbSAZURUCForm::Basic,
                    "文語基本形" => VerbSAZURUCForm::WrittenLangBasic,
                    "命令ｙｏ" => VerbSAZURUCForm::OrderYO,
                    other => VerbSAZURUCForm::Other(other.to_string()),
                }),
                "ラ変" => VerbCType::RA(match arr[cfg.cform] {
                    "体現接続" => VerbRACForm::SubstantiveConjection,
                    other => VerbRACForm::Other(other.to_string()),
                }),
                "一段" => VerbCType::One(match arr[cfg.cform] {
                    "基本形" => VerbOneCForm::Basic,
                    "体言特殊接続" => VerbOneCForm::SubstantiveConjectionSpecial,
                    "未然ウ接続" => VerbOneCForm::ImperfectiveUConjection,
                    "命令ｙｏ" => VerbOneCForm::OrderYO,
                    other => VerbOneCForm::Other(other.to_string()),
                }),
                "一段・クレル" => VerbCType::OneKURERU(match arr[cfg.cform] {
                    "未然特殊" => VerbOneKURERUCForm::ImperfectiveSpecial,
                    other => VerbOneKURERUCForm::Other(other.to_string()),
                }),
                "下ニ・ハ行" => VerbCType::Below2HA(match arr[cfg.cform] {
                    "体言接続" => VerbBelow2HACForm::SubstantiveConjection,
                    other => VerbBelow2HACForm::Other(other.to_string()),
                }),
                "下ニ・得" => VerbCType::Below2E(match arr[cfg.cform] {
                    "基本形" => VerbBelow2ECForm::Basic,
                    "体言接続" => VerbBelow2ECForm::SubstantiveConjection,
                    "未然ウ接続" => VerbBelow2ECForm::ImperfectiveUConjection,
                    "命令ｙｏ" => VerbBelow2ECForm::OrderYO,
                    other => VerbBelow2ECForm::Other(other.to_string()),
                }),
                "五段・カ行促音便ユク" => VerbCType::FiveKA(match arr[cfg.cform] {
                    "基本形" => VerbFiveKAYUKUCForm::Basic,
                    "未然ウ接続" => VerbFiveKAYUKUCForm::ImperfectiveUConjection,
                    other => VerbFiveKAYUKUCForm::Other(other.to_string()),
                }),
                "五段・バ行" => VerbCType::FiveBA(match arr[cfg.cform] {
                    "基本形" => VerbFiveBACForm::Basic,
                    "未然ウ接続" => VerbFiveBACForm::ImperfectiveUConjection,
                    other => VerbFiveBACForm::Other(other.to_string()),
                }),
                "五段・マ行" => VerbCType::FiveMA(match arr[cfg.cform] {
                    "基本形" => VerbFiveMACForm::Basic,
                    "未然ウ接続" => VerbFiveMACForm::ImperfectiveUConjection,
                    other => VerbFiveMACForm::Other(other.to_string()),
                }),
                "五段・ラ行" => VerbCType::FiveRA(match arr[cfg.cform] {
                    "体言特殊接続" => VerbFiveRACForm::SubstantiveConjectionSpecial,
                    "体言特殊接続２" => VerbFiveRACForm::SubstantiveConjectionSpecial2,
                    "未然特殊" => VerbFiveRACForm::ImperfectiveSpecial,
                    other => VerbFiveRACForm::Other(other.to_string()),
                }),
                "五段・ラ行特殊" => VerbCType::FiveRASpecial(match arr[cfg.cform] {
                    "基本形" => VerbFiveRASpecialCForm::Basic,
                    "未然ウ接続" => VerbFiveRASpecialCForm::ImperfectiveUConjection,
                    "未然特殊" => VerbFiveRASpecialCForm::ImperfectiveSpecial,
                    other => VerbFiveRASpecialCForm::Other(other.to_string()),
                }),
                "五段・ワ行促音便" => VerbCType::FiveWA(match arr[cfg.cform] {
                    "基本形" => VerbFiveWACForm::Basic,
                    "未然ウ接続" => VerbFiveWACForm::ImperfectiveUConjection,
                    other => VerbFiveWACForm::Other(other.to_string()),
                }),
                other => VerbCType::Other(other.to_string()),
            }
        ),
        "形容詞" => WordClass::Adjective(match arr[cfg.wordsubclass] {
            "自立" => AdjectiveSub::Independent,
            "接尾" => AdjectiveSub::Suffix,
            "非自立" => AdjectiveSub::NonIndependent,
            other => AdjectiveSub::Other(other.to_string()),
        },
        match arr[cfg.ctype] {
            "形容詞・アウオ段" => AdjectiveCType::AUO(match arr[cfg.cform] {
                "ガル接続" => AdjectiveAUOCForm::GARUConjection,
                "基本形" => AdjectiveAUOCForm::Basic,
                "体言接続" => AdjectiveAUOCForm::SubstantiveConjection,
                "文語基本形" => AdjectiveAUOCForm::WrittenLangBasic,
                "連用ゴザイ接続" => AdjectiveAUOCForm::PredicativeGOZAIConjection,
                other => AdjectiveAUOCForm::Other(other.to_string()),
            }),
            "形容詞・イイ" => AdjectiveCType::II(match arr[cfg.cform] {
                "基本形-促音便" => AdjectiveIICForm::Basic,
                other => AdjectiveIICForm::Other(other.to_string()),
            }),
            "形容詞・イ段" => AdjectiveCType::I(match arr[cfg.cform] {
                "ガル接続" => AdjectiveICForm::GARUConjection,
                "基本形" => AdjectiveICForm::Basic,
                "体言接続" => AdjectiveICForm::SubstantiveConjection,
                "文語基本形" => AdjectiveICForm::WrittenLangBasic,
                "連用ゴザイ接続" => AdjectiveICForm::PredicativeGOZAIConjection,
                other => AdjectiveICForm::Other(other.to_string()),
            }),
            "不変化型" => AdjectiveCType::Constant(match arr[cfg.cform] {
                "基本形" => AdjectiveConstantCForm::Basic,
                other => AdjectiveConstantCForm::Other(other.to_string()),
            }),
            other => AdjectiveCType::Other(other.to_string()),
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

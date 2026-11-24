//! Brazilian Portuguese text → phoneme segments. Portuguese spelling is regular enough that rules
//! read it, with two short word lists for what the spelling doesn't show: a stressed e/o that is
//! open ([ɛ ɔ]: "janela", "porta") and the x that isn't [ʃ] ("táxi" [ks], "próximo" [s]).
//!
//! A word goes through four steps:
//! - **letters to units**: digraphs are one consonant (ch lh nh rr ss, qu gu before e/i), h is
//!   silent, and an unstressed i/u after a vowel is a glide ("pai", "meu", "pão") unless it opens
//!   a hiatus ("saída", "rainha", "ruim", "cair");
//! - **syllables**: one vowel each; between two vowels the last consonant, or a stop or f/v + l/r,
//!   opens the second syllable ("car-ro", "a-bra-ço");
//! - **stress**: an acute or circumflex accent, else a tilde, else the penult when the word ends in
//!   a, e, o (+s), am, em, ens, else the last syllable ("ca-FÉ", "ir-MÃ", "CA-sa", "fa-LAR");
//!   articles, prepositions and object pronouns take none ("o", "de", "que", "me");
//! - **sounds**: s between vowels is [z]; r is strong at a word's start, doubled or after a
//!   consonant, a tap between vowels; l closing a syllable is [w]; m/n closing a syllable nasalize
//!   the vowel, as does a stressed vowel before m, n, nh ("cama" [ˈkɐ̃mɐ]); unstressed final a, e,
//!   o are [ɐ i u]; t/d before [i] are [tʃ dʒ]; a stop closing a syllable gets an [i] after it
//!   in speech ("advogado" [a.dʒi.vo.ˈga.du]).
//!
//! Brazil's accents differ in a few of these rules, and a speech therapist needs to hear the
//! patient's own ([`Dialect`]): whether t/d palatalize, how s and r close a syllable, how strong r
//! sounds, whether a final e rises to [i]. Variation is not a disorder.

use super::english::{Phrase, is_phrase_punct, split_keeping};
use super::mandarin::MandarinLexicon;
use super::syllables::{FrontEnd, Lang, lyrics_in};
use super::{Intonation, Segment, Syllable};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------- accents

/// A Brazilian accent: the few rules that tell them apart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    /// São Paulo city: t/d palatalize, s closes as [s], r closes as a tap, strong r is [x]
    #[default]
    Paulistano,
    /// Rio: s closes as [ʃ ʒ], r closes and starts strong as [χ]
    Carioca,
    /// the Northeast: t/d stay [t d], s closes as [ʃ], r as [h]
    Nordestino,
    /// the interior of São Paulo, Minas and the Centre-West: r closes retroflex [ɻ]
    Caipira,
    /// the interior of Rio Grande do Sul: no palatalization, final e stays [e], strong r trilled
    Gaucho,
}

#[derive(Clone, Copy)]
struct Rules {
    palatalize: bool,
    coda_s_palatal: bool,
    coda_r: &'static str,
    strong_r: &'static str,
    raise_e: bool,
}

impl Dialect {
    pub const ALL: [Dialect; 5] = [Dialect::Paulistano, Dialect::Carioca, Dialect::Nordestino, Dialect::Caipira, Dialect::Gaucho];

    pub fn name(self) -> &'static str {
        match self {
            Dialect::Paulistano => "paulistano",
            Dialect::Carioca => "carioca",
            Dialect::Nordestino => "nordestino",
            Dialect::Caipira => "caipira",
            Dialect::Gaucho => "gaucho",
        }
    }

    pub fn from_name(s: &str) -> Option<Dialect> {
        Dialect::ALL.into_iter().find(|d| d.name() == s)
    }

    fn rules(self) -> Rules {
        match self {
            Dialect::Paulistano => Rules { palatalize: true, coda_s_palatal: false, coda_r: "ɾ", strong_r: "x", raise_e: true },
            Dialect::Carioca => Rules { palatalize: true, coda_s_palatal: true, coda_r: "χ", strong_r: "χ", raise_e: true },
            Dialect::Nordestino => Rules { palatalize: false, coda_s_palatal: true, coda_r: "h", strong_r: "h", raise_e: true },
            Dialect::Caipira => Rules { palatalize: true, coda_s_palatal: false, coda_r: "ɻ", strong_r: "x", raise_e: true },
            Dialect::Gaucho => Rules { palatalize: false, coda_s_palatal: false, coda_r: "ɾ", strong_r: "r", raise_e: false },
        }
    }
}

// ---------------------------------------------------------------- word lists

/// Words whose stressed e or o is open ([ɛ ɔ]) though no accent shows it. A stressed e/o is
/// closed otherwise, except before a closing l ("papel", "sol") and in -osa, -osos, -osas.
#[rustfmt::skip]
const OPEN: &[&str] = &[
    // e
    "ela", "elas", "dela", "delas", "nela", "nelas", "aquela", "aquelas", "esta", "estas", "essa", "essas",
    "festa", "festas", "terra", "guerra", "serra", "ferro", "sete", "perto", "perta", "certo", "certa",
    "velho", "velha", "belo", "bela", "janela", "panela", "amarelo", "amarela", "vela", "pele", "neve",
    "resto", "testa", "pedra", "regra", "quero", "quer", "era", "eram", "espera", "fera", "meta", "seta",
    "peste", "reto", "reta", "tela", "cego", "cega", "mexe", "leve", "breve", "febre", "zero", "tese",
    "atleta", "capela", "novela", "tigela", "favela", "costela", "aquarela", "colega", "pressa", "dezessete",
    "ideia", "ideias", "geleia", "plateia", "assembleia", "estreia", "europeia", "coreia", "epopeia", "colmeia",
    "inverno", "moderno", "moderna", "caderno", "eterno", "inferno", "hera", "sela", "cera", "tempero",
    // o
    "bola", "porta", "portas", "nova", "novas", "novos", "roda", "moda", "hora", "horas", "agora", "pode",
    "podem", "posso", "escola", "cola", "sola", "mola", "forte", "morte", "sorte", "norte", "copo", "copos",
    "loja", "rosa", "cobra", "pobre", "bota", "mole", "foto", "moto", "logo", "obra", "obras", "nossa",
    "nosso", "nossas", "nossos", "vossa", "corpos", "olhos", "jogos", "fogos", "ovos", "porcos", "portos",
    "povos", "mortos", "nota", "notas", "rota", "toca", "roça", "cobre", "dobra", "chora", "mora", "fora",
    "senhora", "porca", "torta", "morta", "volta", "solta", "revolta", "poça", "joia", "joias", "boia",
    "jiboia", "paranoia", "heroico", "europa", "forma", "nove", "dose", "costa", "costas", "gosta", "mosca",
    "roxa", "posta", "proposta", "resposta", "aposta", "corda", "gorda", "acorda", "bolo"
];

/// Words whose stressed o is closed though a rule would open it.
const CLOSED: &[&str] = &["bolsa", "bolso", "roxo", "gostoso", "esposa", "esposo"];

/// Stems whose u after q/g is said before e/i (no more diaeresis since 2009: "cinquenta").
const SOUNDED_U: &[&str] = &["cinquent", "tranquil", "frequen", "linguiç", "linguis", "aguent", "pinguim", "sequest", "eloquen", "delinquen", "consequen", "quinquen", "unguent", "bilingu", "arguição"];

/// Words (from their start) whose x between vowels isn't [ʃ].
#[rustfmt::skip]
const X_WORDS: &[(&str, &str)] = &[
    ("táxi", "ks"), ("taxi", "ks"), ("fix", "ks"), ("tóxic", "ks"), ("toxic", "ks"), ("intoxic", "ks"),
    ("sexo", "ks"), ("sexu", "ks"), ("anex", "ks"), ("reflex", "ks"), ("complex", "ks"), ("axila", "ks"),
    ("nexo", "ks"), ("oxigên", "ks"), ("oxigen", "ks"), ("óxid", "ks"), ("oxid", "ks"), ("box", "ks"),
    ("tórax", "ks"), ("ônix", "ks"), ("látex", "ks"), ("léxic", "ks"), ("lexic", "ks"), ("flex", "ks"),
    ("fluxo", "ks"), ("maxila", "ks"), ("paradox", "ks"), ("ortodox", "ks"), ("asfixi", "ks"),
    ("crucifix", "ks"), ("prefix", "ks"), ("sufix", "ks"), ("convex", "ks"), ("xerox", "ks"),
    ("próxim", "s"), ("proxim", "s"), ("máxim", "s"), ("maxim", "s"), ("auxíli", "s"), ("auxili", "s"),
    ("trouxe", "s"), ("sintaxe", "s"),
];

/// Words with no stress of their own.
#[rustfmt::skip]
const CLITICS: &[&str] = &[
    "o", "a", "os", "as", "e", "de", "do", "da", "dos", "das", "em", "no", "na", "nos", "nas", "me", "te",
    "se", "lhe", "lhes", "que", "por", "com", "sem", "mas", "ou", "ao", "aos", "à", "às", "pra", "pro",
    "pras", "pros", "num", "numa", "um", "uma", "uns", "umas",
];

/// Words whose stress the rules miss (syllable index).
const STRESS: &[(&str, usize)] = &[("porque", 1)];

/// "muito" is nasal without a nasal letter.
const MUITO: &[&str] = &["muito", "muita", "muitos", "muitas"];

// ---------------------------------------------------------------- letters

fn is_vowel_letter(c: char) -> bool {
    "aeiouyáéíóúâêôãõàü".contains(c)
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || "áéíóúâêôãõàüçÁÉÍÓÚÂÊÔÃÕÀÜÇ".contains(c)
}

/// The vowel without its accent.
fn base(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' => 'a',
        'é' | 'ê' => 'e',
        'í' | 'y' => 'i',
        'ó' | 'ô' | 'õ' => 'o',
        'ú' | 'ü' => 'u',
        c => c,
    }
}

/// An accent that marks the stressed vowel (à marks a contraction, not stress).
fn marks_stress(c: char) -> bool {
    "áéíóúâêô".contains(c)
}

fn is_front(c: char) -> bool {
    matches!(base(c), 'e' | 'i')
}

fn single(c: char) -> Option<&'static str> {
    Some(match c {
        'b' => "b",
        'c' => "c",
        'ç' => "ç",
        'd' => "d",
        'f' => "f",
        'g' => "g",
        'j' => "j",
        'k' => "k",
        'l' => "l",
        'm' => "m",
        'n' => "n",
        'p' => "p",
        'q' => "q",
        'r' => "r",
        's' => "s",
        't' => "t",
        'v' => "v",
        'w' => "w",
        'x' => "x",
        'z' => "z",
        _ => return None,
    })
}

// ---------------------------------------------------------------- units and syllables

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Vowel(char),
    Glide(char),
    Cons(&'static str),
}

/// A vowel, a glide or a consonant (a digraph is one), and the letter it starts at.
#[derive(Clone, Copy, Debug)]
struct Unit {
    kind: Kind,
    at: usize,
}

fn code(u: &Unit) -> &'static str {
    if let Kind::Cons(c) = u.kind { c } else { "" }
}

fn letter(u: &Unit) -> char {
    match u.kind {
        Kind::Vowel(c) | Kind::Glide(c) => c,
        Kind::Cons(_) => ' ',
    }
}

fn units(w: &[char]) -> Vec<Unit> {
    let word: String = w.iter().collect();
    let sounded = SOUNDED_U.iter().any(|s| word.contains(s));
    let mut out = Vec::with_capacity(w.len());
    let mut i = 0;
    while i < w.len() {
        let c = w[i];
        let next = w.get(i + 1).copied();
        if is_vowel_letter(c) {
            out.push(Unit { kind: Kind::Vowel(c), at: i });
            i += 1;
            continue;
        }
        let (code, len) = match (c, next) {
            ('c', Some('h')) => ("ch", 2),
            ('l', Some('h')) => ("lh", 2),
            ('n', Some('h')) => ("nh", 2),
            ('r', Some('r')) => ("rr", 2),
            ('s', Some('s')) => ("ss", 2),
            ('q' | 'g', Some('u' | 'ü')) if w.get(i + 2).is_some_and(|&v| is_vowel_letter(v)) => {
                let silent = next == Some('u') && w.get(i + 2).is_some_and(|&v| is_front(v)) && !sounded;
                let code = match (c, silent) {
                    ('q', true) => "k",
                    ('q', false) => "kw",
                    (_, true) => "gu",
                    _ => "gw",
                };
                (code, 2)
            }
            ('h', _) => {
                i += 1;
                continue;
            }
            _ => match single(c) {
                Some(s) => (s, 1),
                None => {
                    i += 1;
                    continue;
                }
            },
        };
        out.push(Unit { kind: Kind::Cons(code), at: i });
        i += len;
    }
    out
}

/// An i or u after a vowel opens a hiatus before nh ("rainha"), and before r, l, m, n, z that
/// close its syllable ("cair", "Raul", "ruim", "ainda", "juiz").
fn hiatus_after(us: &[Unit], k: usize) -> bool {
    match us.get(k + 1).map(|u| u.kind) {
        Some(Kind::Cons("nh")) => true,
        Some(Kind::Cons("r" | "l" | "m" | "n" | "z")) => !matches!(us.get(k + 2).map(|u| u.kind), Some(Kind::Vowel(_))),
        _ => false,
    }
}

/// Falling diphthongs: an unaccented i/u after another vowel ("pai", "meu", "fui"), and the
/// nasal ones ("pão", "mãe", "põe").
fn mark_glides(us: &mut [Unit]) {
    for k in 1..us.len() {
        let (Kind::Vowel(p), Kind::Vowel(c)) = (us[k - 1].kind, us[k].kind) else { continue };
        let nasal = (p == 'ã' && matches!(c, 'o' | 'e' | 'i')) || (p == 'õ' && c == 'e');
        let falling = matches!(c, 'i' | 'u' | 'y') && base(p) != base(c) && !hiatus_after(us, k);
        if nasal || falling {
            us[k].kind = Kind::Glide(c);
        }
    }
}

/// One spelled syllable: unit indices, and the letter its text starts at.
struct Syl {
    onset: Vec<usize>,
    nuc: usize,
    glide: Option<usize>,
    coda: Vec<usize>,
    start: usize,
}

/// A stop or f/v before l/r: a cluster that opens a syllable ("bra", "pla", "fri").
fn is_cluster(a: &str, b: &str) -> bool {
    matches!(b, "r" | "l") && matches!(a, "p" | "b" | "t" | "d" | "c" | "k" | "g" | "f" | "v")
}

fn syllabify(us: &[Unit]) -> Vec<Syl> {
    let mut syls: Vec<Syl> = (0..us.len())
        .filter(|&i| matches!(us[i].kind, Kind::Vowel(_)))
        .map(|n| Syl { onset: Vec::new(), nuc: n, glide: us.get(n + 1).filter(|u| matches!(u.kind, Kind::Glide(_))).map(|_| n + 1), coda: Vec::new(), start: 0 })
        .collect();
    if syls.is_empty() {
        return syls;
    }
    for k in 0..syls.len() {
        let from = if k == 0 { 0 } else { syls[k - 1].glide.unwrap_or(syls[k - 1].nuc) + 1 };
        let cs: Vec<usize> = (from..syls[k].nuc).collect();
        let n = cs.len();
        let take = if k == 0 {
            n
        } else if n >= 2 && is_cluster(code(&us[cs[n - 2]]), code(&us[cs[n - 1]])) {
            2
        } else {
            n.min(1)
        };
        if k > 0 {
            syls[k - 1].coda.extend_from_slice(&cs[..n - take]);
        }
        syls[k].onset = cs[n - take..].to_vec();
    }
    let last = syls.len() - 1;
    let end = syls[last].glide.unwrap_or(syls[last].nuc) + 1;
    syls[last].coda = (end..us.len()).collect();
    for k in 1..syls.len() {
        // rr and ss split in writing: "car-ro", "pas-so"
        syls[k].start = match syls[k].onset.first() {
            Some(&o) => us[o].at + usize::from(matches!(code(&us[o]), "rr" | "ss")),
            None => us[syls[k].nuc].at,
        };
    }
    syls
}

fn stressed_syllable(word: &str, us: &[Unit], syls: &[Syl]) -> Option<usize> {
    if syls.is_empty() || CLITICS.contains(&word) {
        return None;
    }
    if let Some((_, k)) = STRESS.iter().find(|(w, _)| *w == word) {
        return Some((*k).min(syls.len() - 1));
    }
    if let Some(k) = syls.iter().position(|s| marks_stress(letter(&us[s.nuc]))) {
        return Some(k);
    }
    if let Some(k) = syls.iter().position(|s| matches!(letter(&us[s.nuc]), 'ã' | 'õ')) {
        return Some(k);
    }
    let n = syls.len();
    if n == 1 {
        return Some(0);
    }
    let core = word.strip_suffix('s').unwrap_or(word);
    let paroxytone = core.ends_with(['a', 'e', 'o']) || core.ends_with("am") || core.ends_with("em") || core.ends_with("en");
    Some(if paroxytone { n - 2 } else { n - 1 })
}

// ---------------------------------------------------------------- sounds

/// How long a glide takes to reach its target (s): it and its vowel are one movement.
const GLIDE: f64 = 0.06;

/// A syllable's sounds. A glide is a vowel gliding off the nucleus (`Segment::glide`).
#[derive(Clone, Debug)]
struct PSyl {
    onset: Vec<Segment>,
    nucleus: Segment,
    glide: Option<Segment>,
    coda: Vec<Segment>,
    start: usize,
    stressed: bool,
}

/// A word read: its syllables, its letters as typed, and whether it ends in an s or r that a
/// vowel after it would change ("as asas" [az ˈa.zɐs], "falar inglês" [fa.ˈla.ɾĩ...]).
struct Word {
    syls: Vec<PSyl>,
    letters: Vec<char>,
    ends_s: bool,
    ends_r: bool,
}

fn glide_seg(back: bool, nasal: bool) -> Segment {
    Segment { ipa: if back { "ʊ" } else { "ɪ" }.into(), stress: Some(0), glide: Some(GLIDE), nasalized: nasal.then_some(true), ..Default::default() }
}

/// The [i] said after a consonant that can't close a syllable ("ad-vo-ga-do" [a.dʒi.vo...]).
fn epenthetic(consonant: &str, r: Rules) -> PSyl {
    let c = match consonant {
        "t" if r.palatalize => "tʃ",
        "d" if r.palatalize => "dʒ",
        c => c,
    };
    PSyl { onset: vec![Segment::new(c)], nucleus: Segment { stress: Some(0), dur: Some(0.06), ..Segment::new("i") }, glide: None, coda: Vec::new(), start: 0, stressed: false }
}

/// A consonant letter's sound where it can't open a syllable.
fn plain(code: &str) -> &'static str {
    match code {
        "b" => "b",
        "c" | "k" | "q" => "k",
        "gu" => "g",
        "d" => "d",
        "f" => "f",
        "g" => "g",
        "p" => "p",
        "t" => "t",
        "v" => "v",
        "j" => "ʒ",
        "ch" => "ʃ",
        "w" => "w",
        "l" => "l",
        "m" => "m",
        "n" => "n",
        _ => "",
    }
}

fn x_onset(word: &str, us: &[Unit], u: usize) -> Vec<&'static str> {
    if let Some((_, s)) = X_WORDS.iter().find(|(stem, _)| word.starts_with(stem)) {
        return match *s {
            "ks" => vec!["k", "s"],
            _ => vec!["s"],
        };
    }
    if u == 0 {
        return vec!["ʃ"];
    }
    match us[u - 1].kind {
        // ex- before a vowel at a word's start: "exame", "êxito", "inexato"
        Kind::Vowel(_) => {
            let before: String = word.chars().take(us[u].at).collect();
            if matches!(before.as_str(), "e" | "ê" | "ine" | "he" | "ree") { vec!["z"] } else { vec!["ʃ"] }
        }
        // after a glide ("caixa", "peixe") or a nasal ("enxada"): [ʃ]
        _ => vec!["ʃ"],
    }
}

fn voiced_code(c: &str) -> bool {
    matches!(c, "b" | "d" | "g" | "gw" | "gu" | "v" | "z" | "j" | "m" | "n" | "l" | "lh" | "nh" | "r" | "rr")
}

/// Push a coda sound onto the syllable it ends up in: after an [i] inserted for a stop, that one.
fn push_coda(coda: &mut Vec<Segment>, post: &mut [PSyl], seg: Segment) {
    match post.last_mut() {
        Some(p) => p.coda.push(seg),
        None => coda.push(seg),
    }
}

/// Read one word. `speech`: insert the [i] after a stop that closes a syllable, and give [ɐ] its
/// lengths; a sung note keeps the consonant in its coda.
fn read_word(typed: &str, r: Rules, speech: bool) -> Word {
    let letters: Vec<char> = typed.chars().filter(|c| is_letter(*c)).collect();
    let w: Vec<char> = letters.iter().map(|c| c.to_lowercase().next().unwrap_or(*c)).collect();
    let word: String = w.iter().collect();
    let mut us = units(&w);
    mark_glides(&mut us);
    let syls = syllabify(&us);
    let stress = stressed_syllable(&word, &us, &syls);
    let n = syls.len();
    let mut out: Vec<PSyl> = Vec::with_capacity(n + 2);
    let (mut ends_s, mut ends_r) = (false, false);

    for (k, s) in syls.iter().enumerate() {
        let stressed = stress == Some(k);
        let post_tonic = !stressed && stress.is_none_or(|t| k > t);
        let last = k + 1 == n;
        let l = letter(&us[s.nuc]);
        let b = base(l);
        let coda_units: Vec<usize> = s.coda.iter().copied().filter(|&c| !matches!(code(&us[c]), "m" | "n")).collect();
        let nasal_coda = coda_units.len() < s.coda.len();
        let next = syls.get(k + 1);
        let next_onset = next.and_then(|t| t.onset.first()).map(|&o| (code(&us[o]), us.get(o + 1).is_some_and(|u| matches!(u.kind, Kind::Vowel(v) if is_front(v)))));
        let nasal = nasal_coda
            || matches!(l, 'ã' | 'õ')
            || (stressed && s.coda.is_empty() && matches!(next_onset, Some(("m" | "n" | "nh", _))))
            || (stressed && MUITO.contains(&word.as_str()));
        let open = stressed
            && !nasal
            && !CLOSED.contains(&word.as_str())
            && (OPEN.contains(&word.as_str())
                || coda_units.iter().any(|&c| code(&us[c]) == "l")
                || (b == 'o' && k + 2 == n && (word.ends_with("osa") || word.ends_with("osas") || word.ends_with("osos"))));
        let reduces = post_tonic && last && coda_units.iter().all(|&c| matches!(code(&us[c]), "s" | "z"));
        let v: &'static str = match b {
            'a' => if nasal || l == 'â' || (post_tonic && last && (stress.is_some() || syls.len() > 1)) { "ɐ" } else { "a" },
            'e' => match l {
                _ if nasal => "e",
                'é' => "ɛ",
                'ê' => "e",
                _ if open => "ɛ",
                _ if reduces && !nasal && r.raise_e => "i",
                _ => "e",
            },
            'o' => match l {
                _ if nasal => "o",
                'ó' => "ɔ",
                'ô' | 'õ' => "o",
                _ if open => "ɔ",
                _ if reduces && !nasal => "u",
                _ => "o",
            },
            'u' => "u",
            _ => "i",
        };
        let mut nucleus = Segment { ipa: v.into(), stress: Some(u8::from(stressed)), nasalized: nasal.then_some(true), ..Default::default() };
        if speech && v == "ɐ" {
            // Klatt's duration table has no [ɐ] (80 ms by default): a stressed one is as long as
            // a stressed [a] comes out, an unstressed final one a little longer than a medial one
            nucleus.dur = Some(if stressed { 0.16 } else if last { 0.1 } else { 0.07 });
        }

        // the glide: "ou" is [o] and "ei" before r, j, x, ch, g(e) is [e] ("pouco", "beijo")
        let mut glide = s.glide.and_then(|g| {
            let gl = letter(&us[g]);
            let after = us.get(g + 1).map(code).unwrap_or("");
            let front_after = us.get(g + 2).is_some_and(|u| matches!(u.kind, Kind::Vowel(v) if is_front(v)));
            let drop = (l == 'o' && gl == 'u') || (l == 'e' && gl == 'i' && (matches!(after, "r" | "j" | "x" | "ch") || (after == "g" && front_after)));
            (!drop).then(|| glide_seg(base(gl) == 'u' || gl == 'o', nasal))
        });
        // final -am, -em, -ens: nasal diphthongs ("falam" [ɐ̃w̃], "tem" [ẽj̃], "jovens")
        if last && nasal_coda && glide.is_none() {
            if l == 'a' && !stressed && s.coda.iter().any(|&c| code(&us[c]) == "m") {
                glide = Some(glide_seg(true, true));
            } else if b == 'e' {
                glide = Some(glide_seg(false, true));
            }
        }

        // the onset
        let mut pre: Vec<PSyl> = Vec::new();
        let mut onset: Vec<Segment> = Vec::new();
        let on = &s.onset;
        let skip_first = speech && k == 0 && on.len() >= 2 && !is_cluster(code(&us[on[0]]), code(&us[on[1]]));
        for (j, &u) in on.iter().enumerate() {
            let c = code(&us[u]);
            if j == 0 && skip_first {
                // "pneu", "psicologia": the first consonant gets an [i] of its own
                pre.push(PSyl { start: s.start, ..epenthetic(plain(c), r) });
                continue;
            }
            let word_initial = k == 0 && j == 0;
            let after_consonant = j == 0 && k > 0 && !syls[k - 1].coda.is_empty();
            let front = us.get(u + 1).is_some_and(|x| matches!(x.kind, Kind::Vowel(v) if is_front(v)));
            let before_i = j + 1 == on.len() && v == "i";
            let after_vowel = u > 0 && matches!(us[u - 1].kind, Kind::Vowel(_) | Kind::Glide(_));
            let sounds: Vec<&'static str> = match c {
                "t" if before_i && r.palatalize => vec!["tʃ"],
                "d" if before_i && r.palatalize => vec!["dʒ"],
                "c" if front => vec!["s"],
                "g" if front => vec!["ʒ"],
                "ç" | "ss" => vec!["s"],
                "lh" => vec!["ʎ"],
                "nh" => vec!["ɲ"],
                "kw" => vec!["k", "w"],
                "gw" => vec!["g", "w"],
                "rr" => vec![r.strong_r],
                "r" if j > 0 => vec!["ɾ"],
                "r" if word_initial || after_consonant => vec![r.strong_r],
                "r" => vec!["ɾ"],
                "s" if after_vowel => vec!["z"],
                "s" => vec!["s"],
                "z" => vec!["z"],
                "x" => x_onset(&word, &us, u),
                c => vec![plain(c)],
            };
            onset.extend(sounds.into_iter().filter(|s| !s.is_empty()).map(Segment::new));
        }

        // the coda
        let mut coda: Vec<Segment> = Vec::new();
        let mut post: Vec<PSyl> = Vec::new();
        for (j, &u) in coda_units.iter().enumerate() {
            let c = code(&us[u]);
            let word_final = last && j + 1 == coda_units.len();
            let (after, after_front) = match coda_units.get(j + 1) {
                Some(&x) => (Some(code(&us[x])), false),
                None => next_onset.map_or((None, false), |(c, f)| (Some(c), f)),
            };
            match c {
                "l" if glide.is_none() && post.is_empty() => glide = Some(glide_seg(true, false)),
                "l" => push_coda(&mut coda, &mut post, Segment::new("w")),
                "r" | "rr" => {
                    push_coda(&mut coda, &mut post, Segment::new(r.coda_r));
                    ends_r = word_final;
                }
                "x" if word_final => {
                    push_coda(&mut coda, &mut post, Segment::new("k"));
                    push_coda(&mut coda, &mut post, Segment::new("s"));
                }
                "s" | "z" | "ss" | "x" => {
                    // s before an s sound is silent ("nascer", "piscina", "exceto")
                    if matches!(after, Some("s" | "ss" | "ç")) || (after == Some("c") && after_front) {
                        continue;
                    }
                    let voiced = after.is_some_and(voiced_code);
                    let ipa = match (voiced, r.coda_s_palatal) {
                        (false, false) => "s",
                        (true, false) => "z",
                        (false, true) => "ʃ",
                        (true, true) => "ʒ",
                    };
                    push_coda(&mut coda, &mut post, Segment::new(ipa));
                    ends_s = word_final;
                }
                c if speech && !matches!(c, "w") => post.push(PSyl { start: s.start, ..epenthetic(plain(c), r) }),
                c => push_coda(&mut coda, &mut post, Segment::new(plain(c))),
            }
        }

        out.extend(pre);
        out.push(PSyl { onset, nucleus, glide, coda, start: s.start, stressed });
        out.extend(post);
    }
    // Portuguese /p t k/ are unaspirated (VOT about 10-30 ms): the planner's default is English's
    // aspirated release, a puff of breath after each one that made the voice sound airy
    for seg in out.iter_mut().flat_map(|s| s.onset.iter_mut().chain(s.coda.iter_mut())) {
        if matches!(seg.ipa.as_str(), "p" | "t" | "k") {
            seg.aspirated = Some(false);
        }
    }
    Word { syls: out, letters, ends_s, ends_r }
}

impl Word {
    fn segments(&self) -> Vec<Segment> {
        let mut out = Vec::new();
        for s in &self.syls {
            out.extend(s.onset.iter().cloned());
            out.push(s.nucleus.clone());
            out.extend(s.glide.iter().cloned());
            out.extend(s.coda.iter().cloned());
        }
        out
    }

    fn starts_with_vowel(&self) -> bool {
        self.syls.first().is_some_and(|s| s.onset.is_empty())
    }

    /// The word in IPA, syllables split by dots, ˈ before the stressed one ("ˈka.zɐ").
    fn ipa(&self) -> String {
        let seg = |s: &Segment| {
            let mut t = match (s.glide.is_some(), s.ipa.as_str()) {
                (true, "ʊ") => "w".to_string(),
                (true, _) => "j".to_string(),
                _ => s.ipa.clone(),
            };
            if s.nasalized == Some(true) {
                t.push('\u{303}');
            }
            t
        };
        self.syls
            .iter()
            .map(|s| {
                let body: String = s.onset.iter().chain(std::iter::once(&s.nucleus)).chain(s.glide.iter()).chain(s.coda.iter()).map(seg).collect();
                if s.stressed { format!("ˈ{body}") } else { body }
            })
            .collect::<Vec<_>>()
            .join(".")
    }
}

/// Words in a phrase, with s and r before a vowel linking: "as asas" [az], "mar azul" [ɾ].
fn link(words: &mut [Word]) {
    for i in 1..words.len() {
        if !words[i].starts_with_vowel() {
            continue;
        }
        let (s, r) = (words[i - 1].ends_s, words[i - 1].ends_r);
        let Some(last) = words[i - 1].syls.last_mut().and_then(|p| p.coda.last_mut()) else { continue };
        if s {
            last.ipa = "z".into();
        } else if r {
            last.ipa = "ɾ".into();
        }
    }
}

/// One word, as typed, to segments with stress.
pub fn word(typed: &str, dialect: Dialect) -> Vec<Segment> {
    read_word(typed, dialect.rules(), true).segments()
}

/// One word's sung syllables, split as Portuguese spelling splits them ("car-ro", "a-bra-ço"). A
/// glide closes its note ("pai" [paj]); a stop closing a syllable stays a consonant.
fn sung_syllables(typed: &str, r: Rules) -> Vec<Syllable> {
    let wd = read_word(typed, r, false);
    let starts: Vec<usize> = wd.syls.iter().map(|s| s.start).collect();
    wd.syls
        .iter()
        .enumerate()
        .map(|(k, s)| {
            let end = starts.get(k + 1).copied().unwrap_or(wd.letters.len()).max(s.start);
            let mut coda: Vec<Segment> = s.glide.iter().map(|g| Segment::new(if g.ipa == "ʊ" { "w" } else { "j" })).collect();
            coda.extend(s.coda.iter().cloned());
            Syllable { text: wd.letters[s.start..end].iter().collect(), onset: s.onset.clone(), nucleus: Segment { dur: None, ..s.nucleus.clone() }, coda, ..Default::default() }
        })
        .collect()
}

/// The Brazilian Portuguese front end for the lyrics router ([`super::syllables::read_lyrics`]),
/// in one accent. Portuguese is written in Latin letters, so the player chooses it ([`Lang`]);
/// typed hyphens split a word your way.
pub struct Portuguese(pub Dialect);

impl FrontEnd for Portuguese {
    fn starts(&self, c: char) -> bool {
        is_letter(c)
    }
    fn continues(&self, c: char) -> bool {
        is_letter(c) || c == '-'
    }
    fn word(&self, tok: &str) -> Vec<Syllable> {
        let r = self.0.rules();
        tok.split('-').filter(|p| p.chars().any(is_letter)).flat_map(|p| sung_syllables(p, r)).collect()
    }
}

/// Lyrics → sung syllables with Latin script read as Brazilian Portuguese ([`Lang::Pt`]).
pub fn lyrics_to_syllables(text: &str) -> Vec<Syllable> {
    lyrics_in(text, &|_: &str, _: bool| Vec::<Segment>::new(), &MandarinLexicon::new(), Lang::Pt)
}

/// Text → phrases of words, each word read and linked to the next.
fn read_phrases(text: &str, r: Rules) -> Vec<(Vec<Word>, String)> {
    let normalized = expand_numbers(&text.replace(['’', '\''], " "));
    let mut out = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut flush = |delim: &str, words: &mut Vec<String>| {
        let mut read: Vec<Word> = words.drain(..).map(|w| read_word(&w, r, true)).filter(|w| !w.syls.is_empty()).collect();
        if read.is_empty() {
            return;
        }
        link(&mut read);
        out.push((read, delim.to_string()));
    };
    for part in split_keeping(&normalized, is_phrase_punct) {
        if !part.is_empty() && part.chars().all(is_phrase_punct) {
            flush(&part, &mut words);
        } else {
            words.extend(part.split(|c: char| !is_letter(c)).filter(|w| !w.is_empty()).map(str::to_string));
        }
    }
    flush("", &mut words);
    out
}

/// Brazilian Portuguese text → phrases of segments. Phrase punctuation breaks phrases; a question
/// rises at its end; numbers are read out ("1.200,5" → "mil e duzentos vírgula cinco").
pub fn to_phrases(text: &str, dialect: Dialect) -> Vec<Phrase> {
    read_phrases(text, dialect.rules())
        .into_iter()
        .map(|(words, delim)| {
            let mut segments = Vec::new();
            for w in &words {
                let mut s = w.segments();
                if let Some(l) = s.last_mut() {
                    l.word_end = Some(true);
                }
                segments.append(&mut s);
            }
            let pause = if delim.contains(['.', '!', '?']) {
                0.35
            } else if !delim.is_empty() {
                0.18
            } else {
                0.3
            };
            let intonation = if delim.contains('?') { Intonation::Question } else { Intonation::Statement };
            let said = words.iter().map(|w| w.letters.iter().map(|c| c.to_lowercase().next().unwrap_or(*c)).collect()).collect();
            Phrase { segments, intonation, pause, words: said }
        })
        .collect()
}

/// Brazilian Portuguese text → its broad IPA transcription in `dialect`: syllables split by dots,
/// ˈ before the stressed one, words by spaces, phrases by " | " ("A casa." → "ɐ ˈka.zɐ").
pub fn transcribe(text: &str, dialect: Dialect) -> String {
    read_phrases(text, dialect.rules())
        .iter()
        .map(|(words, _)| words.iter().map(Word::ipa).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join(" | ")
}

// ---------------------------------------------------------------- numbers

#[rustfmt::skip]
const UNITS: [&str; 20] = [
    "zero", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove", "dez", "onze", "doze",
    "treze", "catorze", "quinze", "dezesseis", "dezessete", "dezoito", "dezenove",
];
const TENS: [&str; 10] = ["", "", "vinte", "trinta", "quarenta", "cinquenta", "sessenta", "setenta", "oitenta", "noventa"];
const HUNDREDS: [&str; 10] = ["", "cento", "duzentos", "trezentos", "quatrocentos", "quinhentos", "seiscentos", "setecentos", "oitocentos", "novecentos"];

/// 1..=999 in words, joined by "e" ("cento e vinte e três").
fn below_thousand(n: u64) -> String {
    if n == 100 {
        return "cem".into();
    }
    let (h, rest) = ((n / 100) as usize, n % 100);
    let mut parts: Vec<&str> = Vec::new();
    if h > 0 {
        parts.push(HUNDREDS[h]);
    }
    if rest >= 20 {
        parts.push(TENS[(rest / 10) as usize]);
        if rest % 10 > 0 {
            parts.push(UNITS[(rest % 10) as usize]);
        }
    } else if rest > 0 {
        parts.push(UNITS[rest as usize]);
    }
    parts.join(" e ")
}

/// Digits → Portuguese number words (digit by digit past the trillions).
fn number_words(digits: &str) -> String {
    let d = digits.trim_start_matches('0');
    if d.is_empty() {
        return UNITS[0].into();
    }
    let Some(mut n) = d.parse::<u64>().ok().filter(|n| *n < 1_000_000_000_000) else {
        return digits.bytes().map(|b| UNITS[(b - b'0') as usize]).collect::<Vec<_>>().join(" ");
    };
    let mut groups: Vec<String> = Vec::new();
    for (scale, one, many) in [(1_000_000_000, "bilhão", "bilhões"), (1_000_000, "milhão", "milhões"), (1_000, "mil", "mil")] {
        let q = n / scale;
        if q > 0 {
            groups.push(match (scale, q) {
                (1_000, 1) => "mil".into(),
                (_, 1) => format!("um {one}"),
                _ => format!("{} {many}", below_thousand(q)),
            });
            n %= scale;
        }
    }
    if n > 0 {
        // "mil e duzentos", "mil e cinco", but "mil duzentos e trinta"
        let joined = !groups.is_empty() && (n < 100 || n % 100 == 0);
        let words = below_thousand(n);
        groups.push(if joined { format!("e {words}") } else { words });
    }
    groups.join(" ")
}

/// Numbers in text → words. A dot before three digits groups thousands ("1.000"), a comma between
/// digits is the decimal comma ("3,05" → "três vírgula zero cinco").
fn expand_numbers(t: &str) -> String {
    let c: Vec<char> = t.chars().collect();
    let digit = |i: usize| c.get(i).is_some_and(char::is_ascii_digit);
    let mut out = String::with_capacity(t.len());
    let mut i = 0;
    while i < c.len() {
        if !digit(i) {
            out.push(c[i]);
            i += 1;
            continue;
        }
        let mut int = String::new();
        loop {
            while digit(i) {
                int.push(c[i]);
                i += 1;
            }
            if c.get(i) == Some(&'.') && digit(i + 1) && digit(i + 2) && digit(i + 3) && !digit(i + 4) {
                i += 1;
                continue;
            }
            break;
        }
        out.push(' ');
        out.push_str(&number_words(&int));
        if c.get(i) == Some(&',') && digit(i + 1) {
            i += 1;
            let mut frac = String::new();
            while digit(i) {
                frac.push(c[i]);
                i += 1;
            }
            out.push_str(" vírgula");
            let zeros = frac.len() - frac.trim_start_matches('0').len();
            for _ in 0..zeros {
                out.push_str(" zero");
            }
            if zeros < frac.len() {
                out.push(' ');
                out.push_str(&number_words(&frac[zeros..]));
            }
        }
        out.push(' ');
    }
    out
}

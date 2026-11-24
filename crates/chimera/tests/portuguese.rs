//! Brazilian Portuguese G2P: letters to sounds, syllables, stress, nasality, accents, numbers,
//! phrases, and the lyrics front end. Expected transcriptions are broad IPA, syllables split by
//! dots and ˈ before the stressed one.
use chimera::phonetics::portuguese::{Dialect, lyrics_to_syllables, to_phrases, transcribe, word};
use chimera::phonetics::{Intonation, Syllable};

/// Nasal vowels with a combining tilde, as the transcription writes them ("ẽ" → "e\u{303}").
fn nfd(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'ã' => vec!['a', '\u{303}'],
            'ẽ' => vec!['e', '\u{303}'],
            'ĩ' => vec!['i', '\u{303}'],
            'õ' => vec!['o', '\u{303}'],
            'ũ' => vec!['u', '\u{303}'],
            c => vec![c],
        })
        .collect()
}

/// A text in the default accent (paulistano).
fn sp(text: &str) -> String {
    transcribe(text, Dialect::Paulistano)
}

fn same(got: &str, want: &str) -> bool {
    nfd(got) == nfd(want)
}

fn check(dialect: Dialect, pairs: &[(&str, &str)]) {
    let wrong: Vec<String> = pairs
        .iter()
        .filter_map(|(text, want)| {
            let got = transcribe(text, dialect);
            (!same(&got, want)).then(|| format!("{text}: got {got}, want {want}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{:?}:\n{}", dialect, wrong.join("\n"));
}

#[test]
fn consonants() {
    check(Dialect::Paulistano, &[
        ("pato", "ˈpa.tu"),
        ("bola", "ˈbɔ.lɐ"),
        ("tatu", "ta.ˈtu"),
        ("dado", "ˈda.du"),
        ("casa", "ˈka.zɐ"),
        ("gato", "ˈga.tu"),
        ("faca", "ˈfa.kɐ"),
        ("vaca", "ˈva.kɐ"),
        ("sapo", "ˈsa.pu"),
        ("zebra", "ˈze.bɾɐ"),
        ("chave", "ˈʃa.vi"),
        ("janela", "ʒa.ˈnɛ.lɐ"),
        ("mala", "ˈma.lɐ"),
        ("nada", "ˈna.dɐ"),
        ("ninho", "ˈnĩ.ɲu"),
        ("lua", "ˈlu.ɐ"),
        ("filho", "ˈfi.ʎu"),
        ("cara", "ˈka.ɾɐ"),
        ("carro", "ˈka.xu"),
        ("rato", "ˈxa.tu"),
        ("tia", "ˈtʃi.ɐ"),
        ("dia", "ˈdʒi.ɐ"),
        ("sal", "ˈsaw"),
        ("caça", "ˈka.sɐ"),
        ("cebola", "se.ˈbo.lɐ"),
        ("gelo", "ˈʒe.lu"),
        ("guerra", "ˈgɛ.xɐ"),
        ("quilo", "ˈki.lu"),
        ("quando", "ˈkwɐ̃.du"),
        ("água", "ˈa.gwɐ"),
        ("cinquenta", "sĩ.ˈkwẽ.tɐ"),
        ("hora", "ˈɔ.ɾɐ"),
        ("prato", "ˈpɾa.tu"),
        ("honra", "ˈõ.xɐ"),
        ("pensar", "pẽ.ˈsaɾ"),
        ("passo", "ˈpa.su"),
    ]);
}

#[test]
fn vowels() {
    check(Dialect::Paulistano, &[
        ("vida", "ˈvi.dɐ"),
        ("medo", "ˈme.du"),
        ("pé", "ˈpɛ"),
        ("pá", "ˈpa"),
        ("pó", "ˈpɔ"),
        ("avô", "a.ˈvo"),
        ("avó", "a.ˈvɔ"),
        ("uva", "ˈu.vɐ"),
        // e/o open before a closing l, and in -osa
        ("papel", "pa.ˈpɛw"),
        ("sol", "ˈsɔw"),
        ("gostosa", "gos.ˈtɔ.zɐ"),
        ("gostoso", "gos.ˈto.zu"),
        // ou is [o], ei before r/j/x is [e]
        ("pouco", "ˈpo.ku"),
        ("beijo", "ˈbe.ʒu"),
        ("primeiro", "pɾi.ˈme.ɾu"),
        ("feito", "ˈfej.tu"),
    ]);
}

#[test]
fn nasals() {
    check(Dialect::Paulistano, &[
        ("fim", "ˈfĩ"),
        ("dente", "ˈdẽ.tʃi"),
        ("lã", "ˈlɐ̃"),
        ("bom", "ˈbõ"),
        ("um", "ũ"),
        ("uma", "u.mɐ"),
        ("pão", "ˈpɐ̃w̃"),
        ("mãe", "ˈmɐ̃j̃"),
        ("põe", "ˈpõj̃"),
        ("cama", "ˈkɐ̃.mɐ"),
        ("banho", "ˈbɐ̃.ɲu"),
        ("campo", "ˈkɐ̃.pu"),
        ("tem", "ˈtẽj̃"),
        ("também", "tɐ̃.ˈbẽj̃"),
        ("homem", "ˈõ.mẽj̃"),
        ("falam", "ˈfa.lɐ̃w̃"),
        ("jovens", "ˈʒo.vẽj̃s"),
        ("muito", "ˈmũj̃.tu"),
        ("canções", "kɐ̃.ˈsõj̃s"),
    ]);
}

#[test]
fn diphthongs_and_hiatus() {
    check(Dialect::Paulistano, &[
        ("pai", "ˈpaj"),
        ("meu", "ˈmew"),
        ("boi", "ˈboj"),
        ("céu", "ˈsɛw"),
        ("saída", "sa.ˈi.dɐ"),
        ("rainha", "xa.ˈĩ.ɲɐ"),
        ("ruim", "xu.ˈĩ"),
        ("cair", "ka.ˈiɾ"),
        ("praia", "ˈpɾaj.ɐ"),
        ("rio", "ˈxi.u"),
    ]);
}

#[test]
fn stress_by_rule() {
    check(Dialect::Paulistano, &[
        ("café", "ka.ˈfɛ"),
        ("irmã", "iɾ.ˈmɐ̃"),
        ("falar", "fa.ˈlaɾ"),
        ("papai", "pa.ˈpaj"),
        ("urubu", "u.ɾu.ˈbu"),
        ("feliz", "fe.ˈlis"),
        ("lápis", "ˈla.pis"),
        ("porque", "poɾ.ˈke"),
        ("Brasil", "bɾa.ˈziw"),
    ]);
}

#[test]
fn letters_with_many_sounds() {
    check(Dialect::Paulistano, &[
        ("xícara", "ˈʃi.ka.ɾɐ"),
        ("caixa", "ˈkaj.ʃɐ"),
        ("peixe", "ˈpe.ʃi"),
        ("enxada", "ẽ.ˈʃa.dɐ"),
        ("exame", "e.ˈzɐ̃.mi"),
        ("texto", "ˈtes.tu"),
        ("táxi", "ˈta.ksi"),
        ("próximo", "ˈpɾɔ.si.mu"),
        ("lixo", "ˈli.ʃu"),
        ("nascer", "na.ˈseɾ"),
        ("mesmo", "ˈmez.mu"),
    ]);
}

#[test]
fn a_stop_that_closes_a_syllable_gets_an_i() {
    check(Dialect::Paulistano, &[
        ("advogado", "a.dʒi.vo.ˈga.du"),
        ("ritmo", "ˈxi.tʃi.mu"),
        ("pneu", "pi.ˈnew"),
        ("psicologia", "pi.si.ko.lo.ˈʒi.ɐ"),
    ]);
    // a sung note keeps the consonant
    let syl = lyrics_to_syllables("ritmo");
    assert_eq!(texts(&syl), ["rit", "mo"]);
}

#[test]
fn clitics_take_no_stress() {
    assert!(same(&sp("o rato roeu a roupa do rei de Roma"), "u ˈxa.tu xo.ˈew a ˈxo.pɐ du ˈxej dʒi ˈxõ.mɐ"), "{}", sp("o rato roeu a roupa do rei de Roma"));
    assert!(same(&sp("que"), "ki"), "{}", sp("que"));
    assert!(same(&sp("é"), "ˈɛ"), "{}", sp("é"));
}

#[test]
fn s_and_r_link_to_a_vowel() {
    assert!(same(&sp("as asas"), "az ˈa.zɐs"), "{}", sp("as asas"));
    assert!(same(&sp("mar azul"), "ˈmaɾ a.ˈzuw"), "{}", sp("mar azul"));
    assert_eq!(transcribe("mar azul", Dialect::Carioca), "ˈmaɾ a.ˈzuw");
}

#[test]
fn accents() {
    let pairs = [("tia", "ˈtʃi.ɐ"), ("dente", "ˈdẽ.tʃi"), ("porta", "ˈpɔɾ.tɐ"), ("carro", "ˈka.xu"), ("pasta", "ˈpas.tɐ"), ("mesmo", "ˈmez.mu"), ("leite", "ˈlej.tʃi")];
    check(Dialect::Paulistano, &pairs);
    check(Dialect::Carioca, &[("tia", "ˈtʃi.ɐ"), ("dente", "ˈdẽ.tʃi"), ("porta", "ˈpɔχ.tɐ"), ("carro", "ˈka.χu"), ("pasta", "ˈpaʃ.tɐ"), ("mesmo", "ˈmeʒ.mu"), ("leite", "ˈlej.tʃi")]);
    check(Dialect::Nordestino, &[("tia", "ˈti.ɐ"), ("dente", "ˈdẽ.ti"), ("porta", "ˈpɔh.tɐ"), ("carro", "ˈka.hu"), ("pasta", "ˈpaʃ.tɐ"), ("mesmo", "ˈmeʒ.mu"), ("leite", "ˈlej.ti")]);
    check(Dialect::Caipira, &[("tia", "ˈtʃi.ɐ"), ("porta", "ˈpɔɻ.tɐ"), ("carro", "ˈka.xu"), ("pasta", "ˈpas.tɐ")]);
    check(Dialect::Gaucho, &[("tia", "ˈti.ɐ"), ("dente", "ˈdẽ.te"), ("porta", "ˈpɔɾ.tɐ"), ("carro", "ˈka.ru"), ("leite", "ˈlej.te")]);
    for d in Dialect::ALL {
        assert_eq!(Dialect::from_name(d.name()), Some(d));
    }
}

#[test]
fn numbers_are_read_out() {
    assert!(same(&sp("2"), "ˈdojs"), "{}", sp("2"));
    assert!(same(&sp("100"), "ˈsẽj̃"), "{}", sp("100"));
    assert!(same(&sp("1.200"), "ˈmiw i du.ˈzẽ.tus"), "{}", sp("1.200"));
    assert!(same(&sp("3,05"), "ˈtɾes ˈviɾ.gu.lɐ ˈzɛ.ɾu ˈsĩ.ku"), "{}", sp("3,05"));
}

#[test]
fn phrases_break_at_punctuation_and_questions_rise() {
    let p = to_phrases("Oi, tudo bem? Tudo.", Dialect::Paulistano);
    assert_eq!(p.len(), 3);
    assert_eq!(p[1].intonation, Intonation::Question);
    assert_eq!(p[2].intonation, Intonation::Statement);
    assert_eq!(p[1].words, ["tudo", "bem"]);
    let segs = word("pão", Dialect::Paulistano);
    assert!(segs.iter().all(|s| s.ipa == "p" || s.nasalized == Some(true)), "{segs:?}");
    assert_eq!(segs.last().and_then(|s| s.glide).is_some(), true);
}

fn texts(syl: &[Syllable]) -> Vec<String> {
    syl.iter().map(|s| s.text.clone()).collect()
}

#[test]
fn lyrics_split_as_spelling_does() {
    assert_eq!(texts(&lyrics_to_syllables("carro pássaro abraço pão")), ["car", "ro", "pás", "sa", "ro", "a", "bra", "ço", "pão"]);
    let syl = lyrics_to_syllables("pão");
    assert_eq!(syl[0].nucleus.nasalized, Some(true));
    assert_eq!(syl[0].coda[0].ipa, "w");
    // other scripts still go to their own front ends
    let mixed = lyrics_to_syllables("casa 빵 дом");
    assert_eq!(texts(&mixed), ["ca", "sa", "빵", "дом"]);
}

//! Segment timings the planner gives a Portuguese sentence: `cargo run --release -p chimera --example speech_timing -- "texto"`
use chimera::phonetics::planner::{PhraseInput, PlanOptions, plan_phrases};
use chimera::phonetics::portuguese::{Dialect, to_phrases};

fn main() {
    let text = std::env::args().nth(1).unwrap_or_else(|| "Olá, eu sou uma voz feita de ar. Você quer um café?".into());
    let inputs: Vec<PhraseInput> = to_phrases(&text, Dialect::Paulistano).iter().map(|p| PhraseInput { segments: p.segments.clone(), intonation: Some(p.intonation), pause: Some(p.pause) }).collect();
    for p in &inputs {
        println!("in: {}", p.segments.iter().map(|s| format!("{}{}{}", s.ipa, if s.stress == Some(1) { "'" } else { "" }, s.dur.map(|d| format!("({d:.2})")).unwrap_or_default())).collect::<Vec<_>>().join(" "));
    }
    let plan = plan_phrases(&inputs, &PlanOptions { body: "female".into(), t0: 0.0, f0: Some(205.0), ..Default::default() });
    let mut vow = (0.0, 0);
    for s in &plan.segments {
        let ms = (s.t1 - s.t0) * 1000.0;
        if s.kind == "vowel" { vow.0 += ms; vow.1 += 1; }
        println!("{:>6} {:>5.0} ms {:>7} {}", s.ipa, ms, s.kind, if s.stress == Some(1) { "'" } else { "" });
    }
    let syl = inputs.iter().flat_map(|p| &p.segments).filter(|s| "aeiouɐɛɔ".contains(s.ipa.chars().next().unwrap_or(' '))).count();
    println!("total {:.2} s, {} vowels, mean vowel {:.0} ms, {:.1} syllables/s", plan.end, vow.1, vow.0 / vow.1.max(1) as f64, syl as f64 / plan.end);
}

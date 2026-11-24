//! A Portuguese sentence spoken a few ways (pitch-accent size, speaking rate, vibrato-like pitch
//! movement), as WAVs to compare by ear: `cargo run --release -p chimera --example fala_variantes -- "texto"`
use chimera::anatomy::human::{BODY_FEMALE, BODY_MALE};
use chimera::phonetics::planner::{PhraseInput, PlanOptions, plan_phrases};
use chimera::phonetics::portuguese::{Dialect, to_phrases};
use chimera::voice::human_voice::{HumanVoice, SourcePatch};

const FS: f64 = 48000.0;

fn wav(path: &str, x: &[f32]) {
    let peak = x.iter().fold(0f32, |m, v| m.max(v.abs())).max(1e-9);
    let data = (x.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + x.len() * 2);
    for part in [&b"RIFF"[..], &(36 + data).to_le_bytes(), b"WAVEfmt ", &16u32.to_le_bytes(), &1u16.to_le_bytes(), &1u16.to_le_bytes(), &(FS as u32).to_le_bytes(), &(FS as u32 * 2).to_le_bytes(), &2u16.to_le_bytes(), &16u16.to_le_bytes(), b"data", &data.to_le_bytes()] {
        b.extend_from_slice(part);
    }
    for v in x {
        b.extend_from_slice(&((v / peak * 0.8 * 32767.0) as i16).to_le_bytes());
    }
    std::fs::write(path, b).unwrap();
}

fn main() {
    let text = std::env::args().nth(1).unwrap_or_else(|| "Olá, eu sou uma voz feita de ar. Você quer um café? O rato roeu a roupa do rei de Roma.".into());
    std::fs::create_dir_all("out").unwrap();
    let inputs: Vec<PhraseInput> = to_phrases(&text, Dialect::Paulistano).iter().map(|p| PhraseInput { segments: p.segments.clone(), intonation: Some(p.intonation), pause: Some(p.pause) }).collect();
    let clara = SourcePatch { breathiness: Some(0.03), tension: Some(0.65), brightness: Some(0.46), roughness: Some(0.04), pressure: Some(8.5), ..Default::default() };
    // (label, accent, rate, drift cents)
    let variants: [(&str, f64, f64, f64); 4] = [("a-padrao", 0.14, 1.0, 5.0), ("b-mais-melodia", 0.22, 1.0, 5.0), ("c-mais-rapida", 0.14, 1.2, 5.0), ("d-melodia-e-rapida", 0.22, 1.2, 12.0)];
    for (body, name, f0) in [(BODY_FEMALE, "female", 205.0), (BODY_MALE, "male", 110.0)] {
        for (label, accent, rate, drift) in variants {
            let plan = plan_phrases(&inputs, &PlanOptions { body: name.into(), t0: 0.05, f0: Some(f0), accent: Some(accent), rate: Some(rate), ..Default::default() });
            let mut voice = HumanVoice::new(FS, body, 2);
            voice.set_source(&SourcePatch { f0: Some(f0), drift: Some(drift), ..clara });
            voice.schedule(&plan.gestures);
            voice.set_gate(0.0); // as Speak in the app: the plan alone voices; an open gate hums on after it
            let n = (FS * (plan.end + 0.3)) as usize / 128 * 128;
            let mut out = vec![0f32; n];
            for b in out.chunks_mut(128) {
                voice.render(b);
            }
            let file = format!("out/fala-{name}-{label}.wav");
            wav(&file, &out);
            println!("{file}  {:.1} s", plan.end);
        }
    }
}

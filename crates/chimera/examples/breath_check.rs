//! How breathy speech sounds under a few voice-source settings: harmonics-to-noise ratio (HNR,
//! from the normalised autocorrelation of voiced 40 ms frames) of a Portuguese phrase, and a WAV
//! of each for listening. `cargo run --release -p chimera --example breath_check`
use chimera::anatomy::human::{BODY_FEMALE, BODY_MALE, HumanBody};
use chimera::phonetics::planner::{PhraseInput, PlanOptions, plan_phrases};
use chimera::phonetics::portuguese::{Dialect, to_phrases};
use chimera::phonetics::{Intonation, Segment};
use chimera::voice::human_voice::{HumanVoice, SourcePatch};

const FS: f64 = 48000.0;

fn render(body: HumanBody, body_name: &str, f0: f64, s: &SourcePatch, text: &str, asp: f64) -> Vec<f32> {
    let inputs: Vec<PhraseInput> = if let Some(v) = text.strip_prefix("vogal:") {
        vec![PhraseInput { segments: vec![Segment { ipa: v.into(), dur: Some(1.6), stress: Some(1), ..Default::default() }], intonation: Some(Intonation::Flat), pause: Some(0.1) }]
    } else {
        to_phrases(text, Dialect::Paulistano).iter().map(|p| PhraseInput { segments: p.segments.clone(), intonation: Some(p.intonation), pause: Some(p.pause) }).collect()
    };
    let plan = plan_phrases(&inputs, &PlanOptions { body: body_name.into(), t0: 0.05, f0: Some(f0), ..Default::default() });
    let mut voice = HumanVoice::new(FS, body, 2);
    voice.airway.noise_gain.glottis *= asp;
    voice.set_source(&SourcePatch { f0: Some(f0), ..s.clone() });
    voice.schedule(&plan.gestures);
    voice.set_gate(1.0);
    let n = (FS * (plan.end + 0.3)) as usize / 128 * 128;
    let mut out = vec![0f32; n];
    for b in out.chunks_mut(128) {
        voice.render(b);
    }
    out
}

/// Median HNR (dB) of the frames whose autocorrelation peak says they are voiced.
fn hnr(x: &[f32], f0: f64) -> f64 {
    let w = (0.04 * FS) as usize;
    let (lo, hi) = ((FS / (f0 * 1.6)) as usize, (FS / (f0 * 0.6)) as usize);
    let mut vals = Vec::new();
    let mut i = 0;
    while i + w + hi < x.len() {
        let f = &x[i..i + w + hi];
        let e0: f64 = f[..w].iter().map(|v| (*v as f64).powi(2)).sum();
        if e0 > 1e-6 * w as f64 {
            let mut best = 0.0f64;
            for lag in lo..hi {
                let (mut num, mut e1) = (0.0, 0.0);
                for k in 0..w {
                    num += f[k] as f64 * f[k + lag] as f64;
                    e1 += (f[k + lag] as f64).powi(2);
                }
                best = best.max(num / (e0 * e1).sqrt().max(1e-12));
            }
            if best > 0.5 {
                let r = best.min(0.99999);
                vals.push(10.0 * (r / (1.0 - r)).log10());
            }
        }
        i += w / 2;
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vals.get(vals.len() / 2).copied().unwrap_or(f64::NAN)
}

/// H1-H2 (dB) and the 2-4 kHz band against 0-1 kHz (dB), over the middle second, Hann-windowed DFT.
fn spectrum(x: &[f32], f0: f64) -> (f64, f64) {
    let mid = x.len() / 2;
    let n = (FS as usize).min(x.len() - 1);
    let seg = &x[mid.saturating_sub(n / 2)..mid.saturating_sub(n / 2) + n];
    let w = |k: usize| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * k as f64 / n as f64).cos();
    let mag = |hz: f64| {
        let (mut re, mut im) = (0.0, 0.0);
        for (k, v) in seg.iter().enumerate() {
            let ph = 2.0 * std::f64::consts::PI * hz * k as f64 / FS;
            re += w(k) * *v as f64 * ph.cos();
            im -= w(k) * *v as f64 * ph.sin();
        }
        (re * re + im * im).sqrt()
    };
    let peak = |c: f64| (0..=12).map(|i| mag(c * (0.94 + 0.01 * i as f64))).fold(0.0, f64::max);
    let h1h2 = 20.0 * (peak(f0) / peak(2.0 * f0)).log10();
    let band = |a: f64, b: f64| { let mut s = 0.0; let mut h = (a / f0).ceil().max(1.0); while h * f0 < b { s += peak(h * f0).powi(2); h += 1.0; } s };
    (h1h2, 10.0 * (band(2000.0, 4000.0) / band(50.0, 1000.0)).log10())
}

fn wav(path: &str, x: &[f32]) {
    let peak = x.iter().fold(0f32, |m, v| m.max(v.abs())).max(1e-9);
    let mut b = Vec::with_capacity(44 + x.len() * 2);
    let data = (x.len() * 2) as u32;
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&(FS as u32).to_le_bytes());
    b.extend_from_slice(&(FS as u32 * 2).to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data.to_le_bytes());
    for v in x {
        b.extend_from_slice(&((v / peak * 0.8 * 32767.0) as i16).to_le_bytes());
    }
    std::fs::write(path, b).unwrap();
}

fn main() {
    let text = std::env::args().nth(1).unwrap_or_else(|| "Olá, eu sou uma voz feita de ar. Você quer um café?".into());
    std::fs::create_dir_all("out").unwrap();
    let clara = SourcePatch { breathiness: Some(0.03), tension: Some(0.62), brightness: Some(0.42), ..Default::default() };
    let settings: [(&str, SourcePatch, f64); 5] = [
        ("padrao", SourcePatch::default(), 1.0),
        ("clara+", clara, 0.4),
        ("clara+liso", SourcePatch { roughness: Some(0.04), ..clara }, 0.4),
        ("clara+pressao", SourcePatch { pressure: Some(8.5), ..clara }, 0.4),
        ("clara++", SourcePatch { roughness: Some(0.04), pressure: Some(8.5), tension: Some(0.65), brightness: Some(0.46), ..clara }, 0.4),
    ];
    for (body, name, f0) in [(BODY_MALE, "male", 110.0), (BODY_FEMALE, "female", 205.0)] {
        for (label, s, asp) in &settings {
            let x = render(body, name, f0, s, &text, *asp);
            let file = format!("out/sopro-{name}-{label}.wav");
            wav(&file, &x);
            let (h1h2, tilt) = spectrum(&x, f0);
            println!("{name:6} {label:18} HNR {:5.1} dB  H1-H2 {h1h2:5.1} dB  2-4k/0-1k {tilt:6.1} dB  {file}", hnr(&x, f0));
        }
    }
}

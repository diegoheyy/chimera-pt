import { useStore } from "../store.ts";
import { EncoderPages, type EncoderPage } from "./Encoder.tsx";

const NOTE = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
export const noteName = (f: number) => {
  const m = Math.round(69 + 12 * Math.log2(f / 440));
  return `${NOTE[((m % 12) + 12) % 12]}${Math.floor(m / 12) - 1}`;
};

const pct = (v: number) => ({ value: String(Math.round(v * 100)), unit: "%" });
const cm = (v: number) => ({ value: v.toFixed(1), unit: "cm" });
const signedCm = (v: number) => ({ value: `${v >= 0 ? "+" : ""}${v.toFixed(1)}`, unit: "cm" });

/**
 * Play mode's controls: the voice and the articulators without a handle, four at a time
 * (OP-1 style). Values live in the store as before.
 */
export function VoiceControls() {
  const s = useStore((st) => st.source);
  const setSource = useStore((st) => st.setSource);
  const a = useStore((st) => st.artic);
  const setArtic = useStore((st) => st.setArtic);
  const pages: EncoderPage[] = [
    {
      name: "Voz",
      encoders: [
        { label: "Altura", value: s.f0, min: 40, max: 1100, log: true, onChange: (v) => setSource({ f0: v }), format: (v) => ({ value: v.toFixed(0), unit: `Hz ${noteName(v)}` }) },
        { label: "Pressão subglótica", value: s.pressure, min: 2, max: 22, onChange: (v) => setSource({ pressure: v }), format: (v) => ({ value: v.toFixed(1), unit: "cmH₂O" }) },
        { label: "Pressão glótica", value: s.tension, min: 0, max: 1, onChange: (v) => setSource({ tension: v }), format: pct },
        { label: "Vibrato", value: s.vibratoDepth, min: 0, max: 120, onChange: (v) => setSource({ vibratoDepth: v }), format: (v) => ({ value: v.toFixed(0), unit: "cents" }) },
      ],
    },
    {
      name: "Timbre",
      encoders: [
        { label: "Brilho", value: s.brightness, min: 0, max: 1, onChange: (v) => setSource({ brightness: v }), format: pct },
        { label: "Soprosidade", value: s.breathiness, min: 0, max: 1, onChange: (v) => setSource({ breathiness: v }), format: pct },
        { label: "Rugosidade", value: s.roughness, min: 0, max: 1, onChange: (v) => setSource({ roughness: v }), format: pct },
        { label: "Vocal fry", value: s.fry, min: 0, max: 1, onChange: (v) => setSource({ fry: v }), format: pct },
      ],
    },
    {
      name: "Altura",
      encoders: [{ label: "Glissando", value: s.glide, min: 0.005, max: 0.6, log: true, onChange: (v) => setSource({ glide: v }), format: (v) => ({ value: (v * 1000).toFixed(0), unit: "ms" }) }],
    },
    {
      name: "Boca",
      encoders: [
        { label: "Arredondamento", value: 1 - a.lipWidth, min: 0, max: 1, onChange: (v) => setArtic({ lipWidth: 1 - v }), format: pct },
        { label: "Abertura labial", value: a.lipAp, min: -0.3, max: 2.2, onChange: (v) => setArtic({ lipAp: v }), format: cm },
        { label: "Protrusão", value: a.lipProt, min: -0.1, max: 1.4, onChange: (v) => setArtic({ lipProt: v }), format: cm },
        { label: "Mandíbula", value: a.jaw, min: 0, max: 2.4, onChange: (v) => setArtic({ jaw: v }), format: cm },
      ],
    },
    {
      name: "Língua",
      encoders: [
        { label: "Raiz", value: a.root, min: -0.9, max: 0.9, onChange: (v) => setArtic({ root: v }), format: signedCm },
        { label: "Ponta curvada", value: a.ttCurl, min: 0, max: 1, onChange: (v) => setArtic({ ttCurl: v }), format: pct },
      ],
    },
    {
      name: "Garganta",
      encoders: [
        { label: "Véu palatino", value: a.velum, min: -0.25, max: 1.2, onChange: (v) => setArtic({ velum: v }), format: (v) => (v <= 0 ? { value: "Fechado", unit: "" } : { value: v.toFixed(2), unit: "cm²" }) },
        { label: "Laringe", value: a.larynx, min: -1.5, max: 1.5, onChange: (v) => setArtic({ larynx: v }), format: signedCm },
        { label: "Twang", value: a.epilarynx, min: 0, max: 1, onChange: (v) => setArtic({ epilarynx: v }), format: pct },
        { label: "Pregas vestibulares", value: a.ventricular, min: 0, max: 1, onChange: (v) => setArtic({ ventricular: v }), format: pct },
      ],
    },
  ];
  return <EncoderPages label="Voz" pages={pages} />;
}

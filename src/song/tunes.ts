/**
 * Sing's tunes, named for how they sound: melodies anyone's words are fitted to (fit.ts). All are
 * written for this instrument (the cantigas' melodies are traditional). Each phrase is one line of lyrics; a song longer than the tune goes round again.
 * Notes are [midi, beats], at the female voice's pitch; `octave` shifts the other bodies, as the
 * demo songs do. Phrases end on a long note: the breath before the next line comes out of it.
 * After the moods come the cantigas, with their own Portuguese words.
 */
import { DEMO_SONGS } from "../wasm/engine.ts";
import type { Lang } from "../wasm/types.ts";

export interface Tune {
  id: string;
  name: string;
  bpm: number;
  octave: { male: number; female: number; child: number };
  phrases: [number, number][][];
  /** a demo song's own words: choosing it puts them in the lyrics, and Sing sings them a syllable to a note */
  lyrics?: string;
  /** the language the song's own words are read in (English when unset) */
  lang?: Lang;
}

/** a demo song's melody (the engine's catalog, shared with `chimera render song`) cut into phrases of `sizes` notes */
function fromDemo(id: string, name: string, demo: string, sizes: number[]): Tune {
  const d = DEMO_SONGS.find((s) => s.name === demo)!;
  let at = 0;
  const phrases = sizes.map((n) => d.notes.slice(at, (at += n)).map(([m, b]): [number, number] => [m, b]));
  return { id, name, bpm: d.bpm, octave: d.octave, phrases };
}

const MOODS: Tune[] = [
  // "Made of air", the instrument's own: minor, unhurried
  fromDemo("dreamy", "Sonhador", "Made of air", [9, 10]),
  {
    // major, bright, climbing
    id: "joyful",
    name: "Alegre",
    bpm: 104,
    octave: { male: -1, female: 0, child: 0 },
    phrases: [
      [[64, 1], [64, 1], [67, 1], [69, 1], [67, 1.5], [64, 0.5], [62, 2]],
      [[60, 1], [62, 1], [64, 1], [67, 1], [64, 1.5], [62, 0.5], [60, 2]],
      [[69, 1], [69, 1], [72, 1], [69, 1], [67, 1.5], [64, 0.5], [67, 2]],
      [[65, 1], [64, 1], [62, 1], [64, 1], [62, 1], [59, 1], [60, 2]],
    ],
  },
  {
    // minor, slow, falling back home
    id: "somber",
    name: "Sombrio",
    bpm: 76,
    octave: { male: -1, female: 0, child: 0 },
    phrases: [
      [[57, 1], [60, 1], [64, 1.5], [62, 0.5], [60, 1], [59, 1], [57, 2]],
      [[57, 1], [60, 1], [64, 1], [67, 1], [65, 1.5], [64, 0.5], [64, 2]],
      [[65, 1], [64, 1], [62, 1], [60, 1], [62, 1.5], [60, 0.5], [59, 2]],
      [[57, 1], [59, 1], [60, 1], [62, 1], [64, 1], [59, 1], [57, 2]],
    ],
  },
  {
    // quick, short notes, room for many words
    id: "playful",
    name: "Brincalhão",
    bpm: 120,
    octave: { male: -1, female: 0, child: 0 },
    phrases: [
      [[67, 0.5], [67, 0.5], [64, 0.5], [67, 0.5], [69, 1], [67, 1], [64, 0.5], [62, 0.5], [60, 1], [62, 2]],
      [[60, 0.5], [60, 0.5], [62, 0.5], [64, 0.5], [67, 1], [64, 1], [62, 0.5], [60, 0.5], [62, 1], [64, 2]],
      [[67, 0.5], [67, 0.5], [69, 0.5], [71, 0.5], [72, 1], [71, 1], [69, 0.5], [67, 0.5], [64, 1], [67, 2]],
      [[69, 0.5], [67, 0.5], [64, 0.5], [62, 0.5], [60, 1], [62, 1], [64, 0.5], [62, 0.5], [60, 1], [60, 2]],
    ],
  },
  {
    // a slow waltz, long notes
    id: "gentle",
    name: "Suave",
    bpm: 96,
    octave: { male: -1, female: 0, child: 0 },
    phrases: [
      [[65, 2], [69, 1], [72, 2], [69, 1], [67, 2], [65, 1], [67, 3]],
      [[65, 2], [69, 1], [72, 2], [74, 1], [72, 2], [69, 1], [67, 3]],
      [[70, 2], [69, 1], [67, 2], [65, 1], [64, 2], [65, 1], [67, 3]],
      [[65, 2], [69, 1], [67, 2], [64, 1], [62, 2], [64, 1], [65, 3]],
    ],
  },
];

const O = { male: -1, female: 0, child: 0 };
/** [midi, beats] from "60 60 67 67 69 69 67:2": a note per name, ":n" for a length other than a beat */
const notes = (line: string): [number, number][] =>
  line.split(" ").map((n) => {
    const [m, b] = n.split(":");
    return [Number(m), b === undefined ? 1 : Number(b)];
  });

/**
 * Cantigas: melodies in the public domain with Portuguese words written for this app, a syllable
 * to a note (a line of the lyrics to a phrase of the tune), each one drilling a sound. A melody or
 * a word added here must keep the counts equal: tests/portuguese.test.ts checks it.
 */
const CANTIGAS: Tune[] = [
  {
    // "Twinkle, twinkle" (traditional)
    id: "pt:estrelinha",
    name: "Brilha lá no céu",
    bpm: 100,
    octave: O,
    lang: "pt",
    lyrics: "Brilha, brilha, lá no céu, pisca, pisca, sem parar. Lá no alto, a brilhar, feito joia, feito mar. Brilha, brilha, lá no céu, pisca, pisca, sem parar.",
    phrases: [
      "60 60 67 67 69 69 67:2",
      "65 65 64 64 62 62 60:2",
      "67 67 65 65 64 64 62:2",
      "67 67 65 65 64 64 62:2",
      "60 60 67 67 69 69 67:2",
      "65 65 64 64 62 62 60:2",
    ].map(notes),
  },
  {
    // "Frère Jacques" (traditional): the strong r
    id: "pt:rato",
    name: "Rato roeu (r forte)",
    bpm: 108,
    octave: O,
    lang: "pt",
    lyrics: "Rato roeu, rato roeu, a roupa de Roma. Rapidinho roeu, rapidinho roeu. Ri, ri, ri! Ri, ri, ri!",
    phrases: [
      "60 62 64 60",
      "60 62 64 60",
      "64 65 67:2",
      "64 65 67:2",
      "67:0.5 69:0.5 67:0.5 65:0.5 64 60",
      "67:0.5 69:0.5 67:0.5 65:0.5 64 60",
      "60 55 60:2",
      "60 55 60:2",
    ].map(notes),
  },
  {
    // "Mary had a little lamb" (traditional): lh and nh
    id: "pt:ovelhinha",
    name: "Ovelhinha (lh, nh)",
    bpm: 104,
    octave: O,
    lang: "pt",
    lyrics: "A ovelhinha minha, galinha, galinha, a ovelhinha minha tem palha e milho.",
    phrases: ["64 62 60 62 64 64 64:2", "62 62 62:2 64 67 67:2", "64 62 60 62 64 64 64 64", "62 62 64 62 60:4"].map(notes),
  },
  {
    // the "Ode to Joy" (Beethoven): nasal vowels and diphthongs
    id: "pt:nasais",
    name: "Ode às nasais",
    bpm: 112,
    octave: O,
    lang: "pt",
    lyrics: "Mãe, irmã, irmão e tio cantam juntos sem parar. Pão, feijão, limão e mamão tem na mesa de jantar.",
    phrases: ["64 64 65 67 67 65 64 62 60 60 62 64 64:1.5 62:0.5 62:2", "64 64 65 67 67 65 64 62 60 60 62 64 62:1.5 60:0.5 60:2"].map(notes),
  },
  {
    // "Good Morning to All" / "Happy Birthday" (Hill sisters, public domain), with new words: the
    // Brazilian lyrics (1942) are still under copyright
    id: "pt:parabens",
    name: "Parabéns (letra nova)",
    bpm: 100,
    octave: O,
    lang: "pt",
    lyrics: "Que dia tão feliz, vamos todos cantar. Muitas felicidades e um ano de paz!",
    phrases: ["62:0.75 62:0.25 64 62 67 66:3", "62:0.75 62:0.25 64 62 69 67:3", "62:0.75 62:0.25 74 71 67 66 64:2", "72:0.75 72:0.25 71 67 69 67:3"].map(notes),
  },
  {
    // "Alecrim dourado" (Brazilian folk song, public domain), its traditional words; the melody is
    // written down from memory over the song's chords (C, Dm, G, C7, F): correct it by ear
    id: "pt:alecrim",
    name: "Alecrim dourado",
    bpm: 96,
    octave: O,
    lang: "pt",
    lyrics: "Alecrim, alecrim dourado, que nasceu no campo sem ser semeado. Foi meu amor que me disse assim, que a flor do campo é o alecrim.",
    phrases: [
      "67:0.5 67:0.5 72 67:0.5 67:0.5 72:0.5 71:0.5 69:0.5 67:3.5",
      "65:0.5 65:0.5 69:0.5 69:0.5 72:0.5 69:0.5 67:0.5 65:0.5 64:0.5 62:0.5 64:0.5 60:2.5",
      "67:0.5 67:0.5 72:0.5 72 70:0.5 70:0.5 70:0.5 69:0.5 67:0.5 65:3",
      "69:0.5 69:0.5 72:0.5 72:0.5 69:0.5 65:0.5 67:0.5 67:0.5 65:0.5 64:0.5 60:3",
    ].map(notes),
  },
];

/**
 * This Portuguese edition's menu: the moods and the cantigas. The engine's other demo songs (in
 * English, Russian, Japanese) stay in its catalog for `chimera render song`, out of the menu.
 */
export const TUNES: readonly Tune[] = [...MOODS, ...CANTIGAS];

/**
 * Brazilian Portuguese through the page's path: the wasm calls Speak and Sing make, the
 * transcription Speak shows, and the words Speak shows at the mouth.
 */
import { describe, expect, test } from "bun:test";
import { initEngine } from "./harness.ts";

await initEngine();
const { portuguesePhrases, portugueseTranscribe, lyricsToSyllables, planPhrases } = await import("../src/wasm/engine.ts");
const { wordSpans } = await import("../src/ui/words.ts");
const male = { female: 0, child: 0 };

describe("Portuguese", () => {
  test("text becomes phrases with Portuguese sounds, stress and a rising question", () => {
    const phrases = portuguesePhrases("Oi, tudo bem?");
    expect(phrases.length).toBe(2);
    expect(phrases[1].intonation).toBe("question");
    expect(phrases[1].segments.find((s) => s.stress === 1)?.ipa).toBe("u");
    expect(phrases[1].segments.map((s) => s.ipa).slice(0, 4)).toEqual(["t", "u", "d", "u"]);
  });

  test("the transcription follows the accent", () => {
    expect(portugueseTranscribe("tia")).toBe("ˈtʃi.ɐ");
    expect(portugueseTranscribe("tia", "nordestino")).toBe("ˈti.ɐ");
    expect(portugueseTranscribe("porta", "carioca")).toBe("ˈpɔχ.tɐ");
  });

  test("a nasal diphthong lowers the velum through its glide", () => {
    const segs = portuguesePhrases("pão")[0].segments;
    expect(segs.map((s) => s.ipa)).toEqual(["p", "ɐ", "ʊ"]);
    expect(segs[1].nasalized).toBe(true);
    expect(segs[2].nasalized).toBe(true);
    expect(segs[2].glide).toBeGreaterThan(0);
  });

  test("each word gets its span in the plan", () => {
    const phrases = portuguesePhrases("O rato roeu a roupa.");
    const plan = planPhrases(phrases, { body: male, t0: 0 });
    const words = wordSpans(phrases, plan);
    expect(words.map((w) => w.text)).toEqual(["o", "rato", "roeu", "a", "roupa"]);
    for (let i = 1; i < words.length; i++) expect(words[i].t0).toBeGreaterThanOrEqual(words[i - 1].t1 - 1e-9);
  });

  test("lyrics in PT become Portuguese syllables; other scripts still go to their own front ends", () => {
    const syl = lyricsToSyllables("Lá em casa 빵 дом", "pt");
    expect(syl.map((s) => s.text)).toEqual(["Lá", "em", "ca", "sa", "빵", "дом"]);
    expect(syl[0].nucleus.stress).toBe(1);
  });

  test("each cantiga has a syllable for every note", async () => {
    const { TUNES } = await import("../src/song/tunes.ts");
    const pt = TUNES.filter((t) => t.lang === "pt");
    expect(pt.length).toBeGreaterThan(3);
    for (const t of pt) {
      const syl = lyricsToSyllables(t.lyrics!, "pt");
      expect({ id: t.id, syllables: syl.length, texts: syl.map((s) => s.text).join(" ") }).toMatchObject({ id: t.id, syllables: t.phrases.flat().length });
    }
  });
});

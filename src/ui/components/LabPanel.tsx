/**
 * The Lab tab's right column: phonetics lessons in Brazilian Portuguese (src/ui/lab.ts). Pick a
 * module and a word, hear it in the chosen accent, slowed down if you like, and watch the head,
 * the gestural score, the spectrum and the folds; in the supervision modules, hear the target, the
 * simulated error, or one after the other. A 0/1/2 listening sheet is kept on this device only.
 */
import { useState } from "react";
import { useStore } from "../store.ts";
import { PT_ACCENT, speakPhrases, stopSpeaking } from "../speech.ts";
import { portuguesePhrases, portugueseTranscribe, tokenizeIpa } from "../../wasm/engine.ts";
import { DIALECTS, useAccent } from "../accent.ts";
import { MODULES, itemKey, type LabItem } from "../lab.ts";
import { Menu, Slider } from "./Controls.tsx";
import { Chip, Screen } from "./Screen.tsx";
import { ActionBar, PrimaryButton } from "./ActionBar.tsx";
import { ScoreView } from "./ScoreView.tsx";
import { SpectrumView } from "./SpectrumView.tsx";
import { Stroboscope } from "./Stroboscope.tsx";
import { useSpeaking } from "./SpeakPanel.tsx";

const SHEET = "chimera.lab.notas";

function loadSheet(): Record<string, number> {
  try {
    return JSON.parse(localStorage.getItem(SHEET) ?? "{}") as Record<string, number>;
  } catch {
    return {};
  }
}

/** The sheet as CSV (module; word; simulated error; score; accent), saved by the browser. */
function exportSheet(sheet: Record<string, number>, accent: string) {
  const rows = [["modulo", "palavra", "erro_simulado", "nota", "sotaque"]];
  for (const m of MODULES)
    for (const it of m.itens) {
      const n = sheet[itemKey(m, it)];
      if (n !== undefined) rows.push([m.titulo, it.text, it.erro ?? "", String(n), accent]);
    }
  const csv = rows.map((r) => r.map((c) => `"${c.replace(/"/g, '""')}"`).join(";")).join("\n");
  const a = document.createElement("a");
  a.href = URL.createObjectURL(new Blob(["﻿" + csv], { type: "text/csv;charset=utf-8" }));
  a.download = "chimera-ficha-de-escuta.csv";
  a.click();
  URL.revokeObjectURL(a.href);
}

type Say = "alvo" | "erro" | "ambos";

export function LabRight() {
  const body = useStore((s) => s.body);
  const f0 = useStore((s) => s.source.f0);
  const start = useStore((s) => s.start);
  const { dialect, setDialect } = useAccent();
  const [modId, setModId] = useState(MODULES[0].id);
  const [idx, setIdx] = useState(0);
  const [rate, setRate] = useState(1);
  const [sheet, setSheet] = useState(loadSheet);
  const speaking = useSpeaking();
  const mod = MODULES.find((m) => m.id === modId) ?? MODULES[0];
  const item: LabItem = mod.itens[Math.min(idx, mod.itens.length - 1)];
  const key = itemKey(mod, item);
  const ipa = portugueseTranscribe(item.text, dialect);

  const say = async (what: Say) => {
    await start();
    if (!useStore.getState().started) return;
    const alvo = portuguesePhrases(item.text, dialect).map((p) => ({ ...p, pause: 0.7 }));
    const erro = item.erro ? [{ segments: tokenizeIpa(item.erro.replace(/\./g, "")), intonation: "statement" as const, pause: 0.35 }] : [];
    const phrases = what === "alvo" || !erro.length ? alvo : what === "erro" ? erro : [...alvo, ...erro];
    const shown = what === "alvo" || !erro.length ? ipa : what === "erro" ? item.erro! : `${ipa} → ${item.erro}`;
    speakPhrases(phrases, { body, rate, f0, transcription: shown, accent: PT_ACCENT });
  };
  const press = () => (speaking ? stopSpeaking() : void say("alvo"));
  const score = (n: number) => {
    const next = { ...sheet, [key]: n };
    setSheet(next);
    try {
      localStorage.setItem(SHEET, JSON.stringify(next));
    } catch {
      // not kept: fine
    }
  };

  return (
    <div className="right right-lab">
      <ActionBar label="Laboratório" onSpace={press}>
        <PrimaryButton label="Falar" playing={speaking} onClick={press} />
        {item.erro && (
          <>
            <Chip toggle={false} onClick={() => void say("erro")}>
              Erro
            </Chip>
            <Chip toggle={false} onClick={() => void say("ambos")}>
              Alvo → erro
            </Chip>
          </>
        )}
        <div className="act-slider">
          <Slider label="Velocidade" value={rate} min={0.4} max={1.2} onChange={setRate} format={(v) => `${v.toFixed(2)}×`} />
        </div>
        <Menu label="Sotaque" value={dialect} options={DIALECTS.map((d) => ({ value: d.value, label: d.label }))} onChange={setDialect} />
      </ActionBar>
      <Screen
        className="s-lab"
        title="Laboratório"
        sub={mod.titulo}
        tools={
          <Menu
            label="Módulo"
            value={modId}
            options={MODULES.map((m) => ({ value: m.id, label: m.titulo }))}
            onChange={(v) => {
              setModId(v);
              setIdx(0);
            }}
          />
        }
      >
        <div className="lab">
          <p className="lab-goal">{mod.objetivo}</p>
          <div className="lab-items" role="group" aria-label="Palavras">
            {mod.itens.map((it, i) => (
              <Chip key={itemKey(mod, it)} on={it === item} onClick={() => setIdx(i)} title={it.rotulo} className="ipa-chip">
                {it.erro ? `${it.text} → ${it.erro}` : it.text}
              </Chip>
            ))}
          </div>
          <p className="lab-ipa">
            <span className="transcription">/{ipa}/</span>
            {item.erro && <span className="lab-err"> → [{item.erro}] {item.rotulo}</span>}
          </p>
          <p>
            <b>Observe:</b> {mod.observe}
          </p>
          <p>
            <b>Discuta:</b> {mod.pergunta}
          </p>
          <div className="lab-sheet" role="group" aria-label="Nota de escuta">
            <span>Nota de escuta</span>
            {[0, 1, 2].map((n) => (
              <Chip key={n} on={sheet[key] === n} onClick={() => score(n)} title={["irreconhecível", "entende com a lista na mão", "alguém de fora entende"][n]}>
                {n}
              </Chip>
            ))}
            <Chip toggle={false} onClick={() => exportSheet(sheet, dialect)}>
              Exportar
            </Chip>
          </div>
          <p className="dim">
            Sotaque {DIALECTS.find((d) => d.value === dialect)?.label.toLowerCase()}: {DIALECTS.find((d) => d.value === dialect)?.about}
          </p>
        </div>
      </Screen>
      <Screen watch className="s-score" title="Partitura gestual">
        <ScoreView />
      </Screen>
      <Screen watch className="s-spec" title="Espectro" sub="Formantes">
        <SpectrumView />
      </Screen>
      <Screen watch className="s-folds" title="Pregas vocais" sub="100× mais lento">
        <Stroboscope />
      </Screen>
    </div>
  );
}

/**
 * The Brazilian accent Portuguese is read in (Speak and the Lab), kept on the device. Variation is
 * not a disorder: a speech therapist picks the patient's own accent.
 */
import { create } from "zustand";
import type { Dialect } from "../wasm/types.ts";

export const DIALECTS: { value: Dialect; label: string; about: string }[] = [
  { value: "paulistano", label: "Paulistano", about: "tia [tʃ], porta [ɾ], carro [x], pasta [s]" },
  { value: "carioca", label: "Carioca", about: "tia [tʃ], porta [χ], carro [χ], pasta [ʃ]" },
  { value: "nordestino", label: "Nordestino", about: "tia [t], porta [h], carro [h], pasta [ʃ]" },
  { value: "caipira", label: "Caipira", about: "tia [tʃ], porta [ɻ], carro [x], pasta [s]" },
  { value: "gaucho", label: "Gaúcho", about: "tia [t], leite [e], porta [ɾ], carro [r]" },
];

const KEPT = "chimera.accent";

function kept(): Dialect {
  try {
    const d = localStorage.getItem(KEPT);
    return DIALECTS.find((x) => x.value === d)?.value ?? "paulistano";
  } catch {
    return "paulistano"; // no storage here (a private window)
  }
}

export const useAccent = create<{ dialect: Dialect; setDialect: (d: Dialect) => void }>((set) => ({
  dialect: kept(),
  setDialect: (dialect) => {
    set({ dialect });
    try {
      localStorage.setItem(KEPT, dialect);
    } catch {
      // not kept: fine
    }
  },
}));

# Brazilian Portuguese

`crates/chimera/src/phonetics/portuguese.rs` reads Brazilian Portuguese by spelling rules, with two
short word lists for what the spelling doesn't show. Speak and Sing read Latin script as Portuguese
when **PT** is chosen; Speak and the **Lab** tab also choose an accent.

## How a word is read

1. **Units.** Digraphs are one consonant (`ch lh nh rr ss`, `qu gu` before e/i), h is silent, and an
   unaccented i/u after a vowel is a glide (`pai`, `meu`, `pão`) unless it opens a hiatus (`saída`,
   `rainha`, `ruim`, `cair`, `ainda`).
2. **Syllables.** One vowel each; between vowels the last consonant opens the next syllable, or a
   stop/f/v + l/r cluster (`car-ro`, `a-bra-ço`).
3. **Stress.** An acute or circumflex, else a tilde, else the penult for words in a, e, o (+s), am,
   em, ens, else the last syllable. Articles, prepositions and object pronouns take none.
4. **Sounds.** s between vowels is [z]; r is strong word-initially, doubled and after a consonant,
   a tap elsewhere; l closing a syllable is [w]; m/n closing a syllable nasalize the vowel, as does
   a stressed vowel before m/n/nh (`cama` [ˈkɐ̃.mɐ]); unstressed final a, e, o are [ɐ i u]; t/d
   before [i] are [tʃ dʒ]; `ou` is [o] and `ei` before r/j/x/ch is [e]; s before an s sound is
   silent (`nascer`); a stop closing a syllable gets an [i] in speech (`advogado`
   [a.dʒi.vo.ˈga.du]), not in song. Final s and r link to a vowel-initial word (`as asas` [az],
   `mar azul` [ɾ]). Questions rise.

Word lists: `OPEN` (stressed e/o that are [ɛ ɔ]: `janela`, `porta`, `ideia`), `CLOSED`,
`X_WORDS` (x as [ks] or [s]: `táxi`, `próximo`; otherwise [ʃ], or [z] in ex- + vowel),
`SOUNDED_U` (`cinquenta`, `tranquilo`), `CLITICS`, `STRESS` (`porque`), `MUITO`.

Nasal vowels are segments with `nasalized`; the nasal diphthongs' glide (`pão`, `mãe`, `tem`) is a
nasalized glide vowel (`Segment::glide`), so the velum stays down through it. The duration table
has no [ɐ], so the front end gives it lengths, as Russian does.

## Accents (`Dialect`)

| Accent | t/d before [i] | s closing | r closing | strong r | final e |
|---|---|---|---|---|---|
| paulistano (default) | [tʃ dʒ] | [s z] | [ɾ] | [x] | [i] |
| carioca | [tʃ dʒ] | [ʃ ʒ] | [χ] | [χ] | [i] |
| nordestino | [t d] | [ʃ ʒ] | [h] | [h] | [i] |
| caipira | [tʃ dʒ] | [s z] | [ɻ] | [x] | [i] |
| gaúcho | [t d] | [s z] | [ɾ] | [r] | [e] |

Sing reads in the default accent. `chimera render portuguese <out> "texto" [body] [accent]` prints
the transcription and writes a WAV; `chimera syllables "letra" pt` prints the sung syllables.

## Not yet checked

- A native speaker's ear on the rendered audio (the transcriptions are checked by
  `crates/chimera/tests/portuguese.rs`; the sound is not).
- Open/closed e/o beyond the word list, and metaphony in verbs (`jogo`/`jogos`) beyond it.
- Vowel sandhi between words (`de agora` [dʒa.ˈgɔ.ɾɐ]) and secondary stress.

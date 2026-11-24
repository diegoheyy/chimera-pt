/**
 * The Lab: phonetics lessons in Brazilian Portuguese for speech-therapy students and interns. Each
 * module has a goal, words to hear and watch in the vocal tract, what to look at, and a question
 * to discuss. The supervision modules pair a target word with a simulated error, written in IPA,
 * so a student hears and sees what changes. Nothing here is a diagnosis or a treatment: it shows a
 * model of the airway, not the patient's own.
 */

export interface LabItem {
  /** the word, read by the Portuguese front end */
  text: string;
  /** a simulated error, in IPA (said through the IPA tokenizer) */
  erro?: string;
  /** what the error is called */
  rotulo?: string;
}

export interface LabModule {
  id: string;
  titulo: string;
  objetivo: string;
  observe: string;
  pergunta: string;
  itens: LabItem[];
}

export const MODULES: LabModule[] = [
  {
    id: "vogais",
    titulo: "Vogais orais",
    objetivo: "Fonte e filtro: a mesma voz, filtrada por formas diferentes do trato, vira vogais diferentes.",
    observe: "A língua sobe e desce (F1) e vai para a frente e para trás (F2). Compare F1 e F2 no espectro.",
    pergunta: "Qual vogal tem o F1 mais alto? E o F2 mais alto? O que a língua faz em cada uma?",
    itens: [{ text: "pá" }, { text: "pé" }, { text: "vê" }, { text: "vi" }, { text: "pó" }, { text: "avô" }, { text: "tu" }],
  },
  {
    id: "ponto",
    titulo: "Ponto articulatório",
    objetivo: "Onde o trato se fecha ou se estreita: lábios, dentes, alvéolos, palato, véu.",
    observe: "O ponto de contato no corte sagital: lábios em [p], ponta da língua em [t], dorso em [k].",
    pergunta: "Que parte da língua toca o céu da boca em cada som? Como isso aparece nos formantes da vogal seguinte?",
    itens: [{ text: "pá" }, { text: "tá" }, { text: "cá" }, { text: "fé" }, { text: "sé" }, { text: "chá" }, { text: "lhe" }, { text: "nhoque" }],
  },
  {
    id: "modo",
    titulo: "Modo articulatório",
    objetivo: "Como o ar passa: fechamento total (oclusiva), estreitamento (fricativa), pelo nariz (nasal), pelos lados (lateral), batida rápida (tepe).",
    observe: "A pressão sobe atrás de uma oclusiva e solta de uma vez; numa fricativa o ar sai turbulento; na nasal o véu abaixa.",
    pergunta: "Em qual dessas palavras o ar nunca para de sair? Em qual ele sai pelo nariz?",
    itens: [{ text: "bala" }, { text: "vala" }, { text: "mala" }, { text: "lata" }, { text: "cara" }, { text: "carro" }],
  },
  {
    id: "vozeamento",
    titulo: "Vozeamento",
    objetivo: "Surdas e sonoras: o mesmo ponto e modo, com ou sem vibração das pregas vocais.",
    observe: "As pregas vocais no estroboscópio e a barra de sonoridade no espectro durante a consoante.",
    pergunta: "O que muda entre os dois sons de cada par? O que não muda?",
    itens: [{ text: "faca" }, { text: "vaca" }, { text: "pato" }, { text: "bato" }, { text: "tela" }, { text: "dela" }, { text: "chá" }, { text: "já" }, { text: "selo" }, { text: "zelo" }],
  },
  {
    id: "nasalidade",
    titulo: "Véu palatino e nasalidade",
    objetivo: "O véu palatino abre ou fecha a passagem para o nariz: vogais orais e nasais, consoantes nasais.",
    observe: "O véu abaixa na vogal nasal e o ar se divide entre boca e nariz. Compare lá e lã.",
    pergunta: "Por que uma criança com disfunção velofaríngea não consegue fazer a pressão de [p] ou [s]?",
    itens: [{ text: "lá" }, { text: "lã" }, { text: "vi" }, { text: "vim" }, { text: "só" }, { text: "som" }, { text: "pão" }, { text: "mãe" }, { text: "cama" }],
  },
  {
    id: "liquidas",
    titulo: "Líquidas e róticos",
    objetivo: "Os sons de l e r: o tepe, o r forte, a lateral alveolar e palatal, o l que vira [w] no fim da sílaba.",
    observe: "O tepe é uma batida curta da ponta da língua; o r forte é uma fricção no fundo; o [ʎ] usa o dorso.",
    pergunta: "Por que caro e carro são um par mínimo? O que o l de sal tem em comum com o u de mau?",
    itens: [{ text: "caro" }, { text: "carro" }, { text: "mala" }, { text: "malha" }, { text: "sal" }, { text: "sala" }, { text: "prato" }, { text: "flor" }],
  },
  {
    id: "variacao",
    titulo: "Palatalização e variação regional",
    objetivo: "O mesmo fonema, realizado de jeitos diferentes conforme a região. Variação não é transtorno.",
    observe: "Troque o sotaque e ouça de novo: [tʃ] ou [t] em tia, [s] ou [ʃ] em pasta, o r de porta.",
    pergunta: "Uma criança de Recife que diz [ˈti.ɐ] tem um desvio? Como explicar isso a uma família?",
    itens: [{ text: "tia" }, { text: "dia" }, { text: "dente" }, { text: "leite" }, { text: "porta" }, { text: "carro" }, { text: "pasta" }, { text: "mesmo" }],
  },
  {
    id: "processos",
    titulo: "Supervisão: processos fonológicos",
    objetivo: "Ouça o alvo e o erro simulado, e descreva o que mudou: ponto, modo, vozeamento ou estrutura.",
    observe: "Compare o corte sagital no alvo e no erro: onde a língua deveria ir e onde ela foi.",
    pergunta: "Como você explicaria esta troca para a família, usando o que aparece na tela?",
    itens: [
      { text: "casa", erro: "ˈta.zɐ", rotulo: "anteriorização (velar → alveolar)" },
      { text: "gato", erro: "ˈda.tu", rotulo: "anteriorização (velar → alveolar)" },
      { text: "tatu", erro: "ka.ˈku", rotulo: "posteriorização (alveolar → velar)" },
      { text: "faca", erro: "ˈpa.kɐ", rotulo: "plosivação de fricativa" },
      { text: "vaca", erro: "ˈfa.kɐ", rotulo: "ensurdecimento" },
      { text: "chave", erro: "ˈsa.vi", rotulo: "frontalização de palatal" },
      { text: "prato", erro: "ˈpa.tu", rotulo: "redução de encontro consonantal" },
      { text: "cara", erro: "ˈka.lɐ", rotulo: "substituição de líquida (tepe → lateral)" },
      { text: "sapo", erro: "ˈθa.pu", rotulo: "ceceio anterior (interdental)" },
      { text: "sapo", erro: "ˈɬa.pu", rotulo: "ceceio lateral" },
    ],
  },
  {
    id: "fissura",
    titulo: "Supervisão: fissura e DVF",
    objetivo: "Articulações compensatórias e hipernasalidade: o som produzido atrás do defeito, onde ainda há pressão.",
    observe: "Na oclusiva glotal o fechamento é nas pregas vocais; na fricativa faríngea, entre a raiz da língua e a faringe; na hipernasalidade o véu fica aberto.",
    pergunta: "Por que a criança leva o ponto para trás? O que a terapia precisa trazer de volta para a boca?",
    itens: [
      { text: "pato", erro: "ˈpa.ʔu", rotulo: "oclusiva glotal no lugar de /t/" },
      { text: "boca", erro: "ˈbo.ʔɐ", rotulo: "oclusiva glotal no lugar de /k/" },
      { text: "sapo", erro: "ˈħa.pu", rotulo: "fricativa faríngea no lugar de /s/" },
      { text: "sapato", erro: "sɐ̃.ˈpɐ̃.tũ", rotulo: "hipernasalidade (véu aberto nas vogais)" },
      { text: "bola", erro: "ˈmõ.lɐ̃", rotulo: "hipernasalidade com nasalização de /b/" },
    ],
  },
];

/** One item's key in the score sheet. */
export const itemKey = (m: LabModule, it: LabItem) => `${m.id}:${it.text}${it.erro ? ">" + it.erro : ""}`;

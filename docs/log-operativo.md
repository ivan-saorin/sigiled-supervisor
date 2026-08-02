# SIGILED-SUPERVISOR — Log operativo

**Contratto.** Questo file è la memoria operativa del progetto. Ogni sessione che chiude lavoro coerente **aggiunge una voce in cima** (la più recente prima) e **aggiorna lo Stato attuale**. Ogni voce risponde a tre domande: **dove eravamo**, **dove prevedevamo di andare**, **cosa è stato fatto** — più gli **scarti**, lo **stato a fine sessione** e il **prossimo passo previsto**. Le voci non si cancellano: si correggono con voci nuove.

**Nota (SIGILED DEC-04):** file project-owned; il layer macchina della storia sarà esposto da SIGILED via API, senza scrivere nei repo.

---

## Stato attuale

_aggiornato: 2026-08-03, sessione di genesi_

- **Su master**: `docs/requisiti.md` v0.2 (sweep di rinomina su v0.1) + questo log. Nessun codice.
- **Genesi**: progetto fresco nato dalla rinomina universale — la piattaforma è SIGILED («seal» eradicato ovunque per ordine del Re). Il predecessore `seal-supervisor` (requisiti v0.1, DEC-01…06) è archiviato come fondazione; la sostanza delle decisioni è ereditata qui invariata.
- **Decisioni**: DEC-01…07 registrate; DEC-06 (Rust) e DEC-07 (nome) ratificate; DEC-01…05 da ratificare. Ratifica madre: DEC-13 di `sigiled`.
- **Prossimo passo previsto**: ratifica DEC-01…05; poi prima sessione di codice in **Rust** e vhost edge `supervisor.016180.xyz` (azione operatore, §6.3 dei requisiti).

---

## Voci

### 2026-08-03 · genesi: rinomina universale, eredità dal predecessore — driver: Kimi K3

- **Dove eravamo**: la piattaforma si è rinominata SIGILED; il Re ha ordinato l'eradicazione totale della stringa precedente, supervisor compreso.
- **Previsione**: progetto fresco con i requisiti sweepati.
- **Fatto**: progetto MGR `sigiled-supervisor` creato (repo `ivan-saorin/sigiled-supervisor`); `docs/requisiti.md` v0.2 — identico nella sostanza alla v0.1 ereditata (sintesi, vincoli ~100 righe, mai `[app]`, zero dipendenze, bearer statico; API `/health`, `/sigiled/status`, `/sigiled/restart` con sha = rollback; env `SIGILED_*`), più DEC-07 (nome, ratificata). Vecchio progetto archiviato con voce conclusiva sul suo log.
- **Scarti**: nessuno — travaso pulito.
- **Stato a fine sessione**: vedi «Stato attuale» sopra.
- **Prossimo passo previsto**: ratifica DEC-01…05; poi prima sessione di codice.

# Far-out composable formalisms — energy models & argumentation over the belief graph

Status: **research synthesis** · 2026-06-15 · from a verified deep-research pass (26 sources fetched,
123 claims extracted, 25 adversarially verified 3-vote, 5 killed, 7 findings survived). The brief:
*which far-out formalisms genuinely compose onto endoxa's signed/typed/weighted belief graph, and is
the load-bearing mechanism real or hype?* Blunt overall verdict: **borrow the structure, not the
guarantees.**

## The one-line map

endoxa already split into two halves that each have a *real, verified* energy/physics formalism:

| endoxa half | the formalism that fits | what it is |
|---|---|---|
| **recall / reduce** (L2/L3 over embeddings) | **modern Hopfield / energy-based associative memory** | retrieval = energy descent to an attractor; the update rule *is* attention |
| **frontier** (`defeated()`) | **gradual / weighted bipolar argumentation** | supports(+)/attacks(−)+base weight → per-node energy; descend to *graded* acceptability |

We independently re-derived weaker versions of both (StructuralConfidence ≈ hand-rolled graded
acceptability; `defeated()` ≈ a grounded extension). The research says: principled versions exist;
here's what adopting them buys and costs.

## Surviving findings (verified, with verdicts)

### F1 — Modern Hopfield update *is* transformer attention · **compose (recall); borrow (reduction analogy)**
Ramsauer et al. 2020 (arXiv:2008.02217, ICLR'21) prove the continuous Hopfield update
`x' = X·softmax(β·Xᵀx)` is *algebraically exactly* attention `softmax(QKᵀ/√d)V`. The energy has three
fixed-point regimes set by β + pattern separation: **global average / metastable subset / single
pattern** — which map onto endoxa's three reduction scales: **frontier-wide consensus / neighborhood
cluster / exact single-belief recall.** Honest limit: exact one-step retrieval needs *well-separated*
patterns, and the regimes→reduction-scales bridge is an author analogy (our reducer is a stochastic
LLM with no β / no energy descent), so **borrow the structure** for the reduction story.

### F2 — Sparse Hopfield gives EXACT retrieval · **compose (the directly useful upgrade)**
Hopfield-Fenchel-Young (Santos/Niculae/Martins, JMLR 2025) unifies all Hopfield variants under one
energy `E(q) = −Ω*(Xq) + Ψ(q)`; **sparse** regularizers (entmax/normmax) make the stored pattern a
*true attractor* (exact retrieval), not a softmax average that smears nearby patterns — *without*
losing exponential capacity. **This is the most buildable idea:** a sparse-Hopfield retrieval head
over the existing `.embeddings.json` is a drop-in alternative to the cosine-kNN L2 lens that retrieves
the exact belief instead of a fuzzy blend.

### F3 — Context-dependent attractor geometry ≈ world-relative frontier · **borrow the structure**
Distributed/Dense Modern Hopfield variants with a *context variable* yield a context-indexed *family*
of stable configurations from one substrate — a formal analogue of endoxa's worlds: the same DAG
yielding parallel realities under different assumption-contexts. Mechanisms differ (continuous vs our
edge-suppression), so analogy, not theorem.

### F4 — Weighted bipolar argumentation is a near-exact energy form of the frontier · **compose as an OPTIONAL graded/soft-defeat layer; borrow on convergence**
Potyka's quadratic-energy weighted bipolar argumentation: a node's `supports(+)` / `attacks(−)` edges
plus a **base score** (= our confidence weight) aggregate into a per-argument energy, and a continuous
ODE descends to a **graded acceptability** equilibrium in [0,1] — a principled *graded* successor to
the binary in/out of `defeated()`. This is the single best structural match to what endoxa already
has. It would give, in one mechanism: graded confidence, soft defeat (a weak attack dents rather than
kills), and the contested-belief signal we hand-rolled as possibility/necessity. **Cost — the
load-bearing weakness:** *no general convergence guarantee on cyclic graphs* (open problem; only
empirical + support-only-cycle proofs), and endoxa has cycles exactly there (mutual attacks,
reinstatement). So adopt it as an *optional graded read-layer* over the existing discrete frontier,
not a replacement for it.

### F5 — `defeated()` is the degenerate point of a tunable graded family · **borrow the structure**
Grossi–Modgil graded semantics generalize Dung acceptance via **counts** of attackers/defenders; the
binary labeling is the unit-grade special case. Note **WEIGHT vs COUNT**: graded semantics is
count/multiplicity-sensitive — it natively consumes our *incoming-supports COUNT* (the documented
entrenchment signal), **not** the continuous confidence envelope. (The tight "grounded extension =
least-fixpoint-of-graded-defense, paralleling `defeated()`" identity was **refuted** in verification —
the looser parametric-generalization framing holds, the exact theorem-level identity does not.)

### F6 — Non-monotonic defeat may have NO energy function · **borrow the caution, skip as mechanism**
Constraint-satisfaction-as-dynamics (asymmetric continuous networks) warns: **asymmetric coupling is
exactly what breaks the symmetric Hopfield/Lyapunov (energy-descent) guarantee.** endoxa's
`supersedes`/`depends_on` are directed and its defeat is non-monotonic with reinstatement — so
`defeated()` likely does *not* sit at a clean energy minimum. The optimistic "asymmetric SAT solvers
have no spurious minima" guarantee was **refuted** in verification. Takeaway: don't claim the frontier
is energy-minimizing; it's a *fixpoint*, which is weaker and that's fine.

### F7 — Energy form is a free design choice · **borrow (informs the design space)**
Deep/meta-learned EBMs show capacity can be decoupled from pattern dimensionality by replacing the
quadratic energy with an arbitrary network whose weights store patterns. Useful framing, not an
immediate build.

## Experimental verdicts — ran both ideas on our own data (2026-06-15, offline, no LLM)

**Both energy ideas under-deliver on endoxa's actual data.** The research said "compose / borrow"; the
experiments downgrade both to **skip / narrow-optional**. A valuable negative result — it saves building
elegant machinery that doesn't pay here.

### Hopfield retrieval (F1/F2) → **SKIP for the L2 lens** (high confidence)
The decisive fact is mechanical, not empirical: **a single Hopfield update `p = proj(β·Xq)` is a
*monotone transform* of the cosine score `Xq`, so the top-k ranking is identical to cosine-kNN by
construction** — measured identical to 3 decimals across every β and sparsity. Multi-step *sparse*
iteration ties cosine (noisy recovery σ=0.15: 0.900 vs 0.901); multi-step *softmax* (the popular
Ramsauer variant) actively **harms** (0.312 — it collapses to spurious metastable blends). The genuine
sparse-attractor property (`p_self → 1.000` at β=32) is real but governs only the *weight vector*, not
which beliefs get surfaced. No regime beat cosine on ranking. **Don't build it.** The one untested place
it could still pay: a single *blended* vector `Xᵀp` (an exact few-pattern convex combination) to seed an
LLM reducer — but our reducer consumes a belief *set* (slugs), not a fused vector, so that's speculative.

### Graded/weighted-bipolar frontier (F4) → **keep binary `defeated()`; graded is at most an optional intra-live tiebreaker** (medium)
Good news first: graded acceptability is **safe** to layer on — over 225 beliefs the binary `defeated()`
labeling is locally energy-optimal at **79%** of nodes, and *all 47* disagreements are the energy pulling
a weakly-supported-but-undefeated node *down* (toward out); it **never reinstates a defeated belief
(0/47)** and respects every defeat pair (33/33). But it earns almost nothing: graded ranks winner>loser
33/33, yet **raw base score alone already gets 30/33** — propagation fixes 3 pairs. On *open conflicts*
(the case binary in/out can't rank) there are only **6 live attack edges across 225 beliefs**; base score
decides 5/6, graded 6/6 — **a net +1 pair of signal across the entire corpus set.** And the convergence
risk the research flagged is **untested, not solved**: there are **0 mutual-attack cycles** in any corpus,
so the ODE converges trivially (<40 steps) only because the failure mode is absent. **Verdict:** the graph
is too shallow (median ~1 open conflict/corpus, no cycles) for message-passing to earn its keep; keep the
discrete fixpoint, and if anything expose graded acceptability as an *optional confidence gradient on the
live set*, never as a defeat signal. Revisit only if a real store grows dense bidirectional conflict.

### The meta-lesson
endoxa's hand-rolled **cosine-kNN + StructuralConfidence + binary `defeated()`** already capture the
signal these energy formalisms would provide, *on the data we have*. The formalisms are real math and
might pay on **different** data (dense conflict graphs with cycles; true OOD text queries; a fused-vector
reducer seed) — but on our corpora they are elegant re-descriptions, not upgrades. "Borrow the structure,
not the guarantees" tightened by experiment to: **don't even borrow the structure until the data needs it.**

## (Original) two cheap experiments — now RUN; verdicts above

1. **Sparse-Hopfield retrieval head vs cosine-kNN (F2).** Build an entmax/normmax retrieval layer over
   the existing per-corpus `.embeddings.json`; A/B it against the current L2 cosine lens on recall@k
   against the `worlds.json` reduction_fixtures + the gold-slug query set (the N1-validate harness).
   *Decides:* does exact-attractor retrieval beat fuzzy cosine on our own data? Fully offline, no LLM.
2. **Is `defeated()` an energy minimum? (F6 / the open question).** Take a corpus frontier, define the
   weighted-bipolar energy over its signed edges, and check whether the discrete `defeated()` labeling
   is a local minimum of that energy — and whether a graded ODE equilibrium (Potyka) agrees with the
   gold winner/loser pairs better than the binary labeling. *Decides:* graded soft-defeat — worth it,
   or does it just reproduce `defeated()` on our acyclic-mostly corpora?

## Refuted / unsettled (intellectual honesty)
- The no-spurious-minima k-SAT guarantee (asymmetric dynamics always finds solutions) — **refuted**.
- `defeated()` = least-fixpoint-of-graded-defense by Knaster–Tarski — **refuted** (tight identity
  unproven; loose generalization holds).
- The exact counting-quantifier graded-defense definition — **refuted** as stated.
- **Convergence of gradual argumentation on cyclic graphs — open problem**, and it's exactly where
  endoxa has cycles. This is the gating risk for adopting graded defeat.

## Not yet investigated (absence of investigation, not a negative verdict)
Secondary brief items — **cellular sheaves / sheaf-Laplacian energy / cohomology-as-inconsistency**,
**opinion dynamics** (DeGroot/Friedkin–Johnsen/bounded-confidence) for the reducer, **quantum
cognition** (order effects/contextuality), **optimal transport / Wasserstein barycenters** for belief
merging, **renormalization / multiscale coarse-graining**, **persistent homology** — surfaced no
verified claims this round (the energy + argumentation angles dominated the search budget). They remain
open for a follow-up pass; sheaves (inconsistency cohomology) and opinion dynamics (reducer consensus)
look the most promising of the unexamined set.

## Round-2 research — the uninvestigated formalisms: sheaves + opinion dynamics are ONE winner (verified 3-0)

A second deep-research pass (105 agents, 2.1M tokens) hit the formalisms the first pass skipped. The
headline, all 3-0 verified against primary sources: **cellular sheaf theory and bounded-confidence
opinion dynamics are the SAME machinery, and unlike the energy ideas they give endoxa something its
fixpoint + LLM-reducer demonstrably CANNOT — and it's computable on our finite weighted graph today.**

### S1 — Sheaf `H⁰ = ker(L_F)`: a deterministic, computable CONSISTENCY metric · **compose (run the experiment first)**
A cellular sheaf assigns a vector space (stalk) to each belief-node/edge + linear restriction maps; the
**sheaf Laplacian `L_F = δᵀδ`** is a finite symmetric PSD matrix whose **kernel is the space of globally
consistent belief assignments** (global sections), found by one nullspace computation. So "**is there a
globally consistent assignment over this belief-set / this world?**" becomes a principled *numeric* test —
something the stochastic LLM reducer can't give. The continuous version is the **degree-0 Dirichlet
energy** `E₀(x) = ‖δ₀x‖² = Σ_edges ‖F_{v◁e}x_v − F_{u◁e}x_u‖²` — a scalar **magnitude of inconsistency**
of a world. `H¹ = coker δ` is the *obstruction* space (genuine inconsistency no assignment resolves).
*Verified caveat:* "vanishing H⁰ ⇒ no consistent assignment" **overreaches** — the zero section is always
trivially consistent, so it's about *nontrivial* global sections; use Dirichlet energy as the scalar.

### S2 — Bounded-confidence opinion dynamics: a deterministic MULTI-CLUSTER reducer · **compose — directly attacks the V4 silent-pick failure**
The opinion-dynamics target is realized *inside the same sheaf framework* as a **nonlinear** Laplacian
`L^∇U_F x = δᵀ diag(ψ'_e(‖δ_e x‖²)) δx` with a **per-edge confidence threshold `D_e`**: an edge influences
only while the discrepancy of expressed opinions is below `D_e`. Its equilibria (Hansen–Ghrist Thm 11.1)
**partition edges into agreeing (active) vs silenced (disagreeing)** — i.e. they yield **multiple
disagreement clusters, not one blurred average.** This is a *deterministic* reducer that **structurally
cannot silent-pick** — exactly the V4 failure where the LLM reducer buries a conflict. It's the
strongest candidate to *complement* the LLM reducer: cluster deterministically, then let the LLM
verbalize each cluster.

### S3 — Sheaf ≡ opinion dynamics, and signed edges map in for free · **the unifying insight**
Hansen–Ghrist's opinion model *is* cellular-sheaf semantics (node = private opinion `x_v`, restriction
map = how it manifests in shared discourse, diffusion = consensus). And endoxa's **signed edges need no
new mechanism**: a negatively-signed (`attacks`) edge is *one restriction-map sign flip*; the per-edge
coboundary `(δx)_e` is the computable per-edge inconsistency. Neural Sheaf Diffusion (Bodnar et al.,
NeurIPS 2022) shows the sheaf geometry is precisely the principled fix for *heterophilic* graphs
(connected nodes that oppose) = our attacks-edges → **borrow the structure**, not the learned-GNN guarantees.

### Verdict + the cheap experiment (which, per the meta-lesson above, must come BEFORE any build)
**Compose — but the energy experiments just taught us paper-good ideas die on contact with our data, so
this one earns the same gate.** The open *design* choice is the **stalk**: the inconsistency metric wants
belief *embeddings* (768-d) as stalks with identity/sign restriction maps; the multi-cluster reducer
wants a low-dim *opinion/acceptance* stalk. Two cheap offline experiments:
1. **Does the Dirichlet inconsistency scalar light up on real conflict?** Compute `E₀` / dim `H¹` over
   each corpus's signed graph (embeddings as stalks) — does it spike exactly on the known open-conflict
   beliefs, and does a *world* (a suppressed defeating edge) show measurably different inconsistency than
   `main`? If the scalar doesn't track the gold conflicts, skip.
2. **Does the BC reducer produce ≥2 clusters where the LLM silent-picks?** Run threshold-gated dynamics
   on the 6–7 open-conflict neighborhoods (N3's set) — does it split into the two real sides
   deterministically? If yes, it's a structural fix for the silent-pick problem the LLM reducer has.

### Not substantiated this pass (unverified, not investigated to a verdict)
Persistence/TDA, optimal-transport/Wasserstein merging, quantum cognition, RG coarse-graining,
Ollivier/Forman Ricci curvature, and signed-graph structural-balance — surfaced **no surviving verified
claims**. Structural balance (frustration in the attacks-graph) and persistence (topology change across
worlds) are the most likely of these to repay a third, narrower pass; the rest read as metaphor here.

## Key sources (verified)
- Ramsauer et al., *Hopfield Networks is All You Need*, arXiv:2008.02217 (ICLR 2021).
- Santos/Niculae/Martins, *Hopfield–Fenchel–Young Networks*, JMLR 2025 (sparse exact retrieval).
- Potyka, quadratic-energy model for weighted bipolar argumentation; Amgoud et al., bipolar AF surveys
  (irit.fr/~Leila.Amgoud/BAFs.pdf).
- Grossi & Modgil, graded acceptability semantics for abstract argumentation.
- Dung 1995 (abstract argumentation); Hopfield 1982 (associative memory) — foundational, stable.
- Richardson & Domingos, Markov Logic Networks (JMLR/CACM); Bach et al., Probabilistic Soft Logic
  (thesis) — the weighted-logic/MAP-as-energy lineage (borrow-the-structure: wants calibrated weights
  our stochastic LLM can't guarantee).

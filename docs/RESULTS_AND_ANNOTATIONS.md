# Results and annotations

Kemdara should make results easy to share without turning heterogeneous machines and workloads into a popularity contest.

## Three contribution paths

| Contributor | Smallest useful action | Review boundary |
| --- | --- | --- |
| Student or curious user | Export a local result bundle and add an observation or question | Automated schema/privacy checks; annotation is clearly attributed |
| Engineer or researcher | Compare compatible reports, document environment differences, and link supporting evidence | Workload fingerprint and provenance must match the claim |
| Implementer | Add an isolated adapter with correctness tests, workload metadata, tradeoffs, and evidence references | Normal code review plus the experiment admission gate |

## Portable result bundle

A future `.kemdara` bundle should contain:

- the immutable versioned benchmark report;
- a workload fingerprint covering operation definition and parameters;
- Kemdara source revision, dependency-lock digest, compiler, target features, and build flags;
- the user's explicit machine-metadata sharing choices;
- optional annotations in a separate file;
- a short generated Markdown summary for humans.

Import must reject unsupported schemas, warn on changed workload fingerprints, and avoid ratios when either report has excessive timing variation. Publishing is always opt-in; the user sees the exact metadata before export.

## Annotation model

Annotations are useful when they record context a benchmark cannot infer: thermal throttling, virtualization, unusual system load, a suspected implementation regression, or why a protocol property matters in a deployment. They are not votes on which algorithm is best.

Each annotation should reference an immutable report and result ID and contain:

- attributed author identity or an explicit local-only/anonymous label;
- kind: `observation`, `hypothesis`, `methodology`, `caveat`, or `evidence`;
- the note and optional evidence links;
- creation time and revision history;
- review state without erasing disagreement.

Raw measurements remain immutable. Corrections produce a new report or annotation revision, never a silent edit.

## What would make people contribute

The exchange must give value before asking for work: an immediate visualization, a surprising but explainable tradeoff, a one-click export, and a comparison the contributor could not get from a single run. Public corpora should only follow after local import/compare and privacy preview are reliable.

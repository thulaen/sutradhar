# Owner decisions on SPEC-008 to SPEC-012, 30 October 2026

Status: historical source record. The owner answered these questions
through the Claude session's question form, up to four at a time, in a
session running on Dell. Each entry gives what was asked in short, the
options offered (the first was the drafter's recommendation) and the
owner's answer exactly as returned, inside a code block. The questions
are in the specifications' own decision lists:
[SPEC-008 to SPEC-010](../../specifications/linking/README.md) and
[SPEC-011 and SPEC-012](../../specifications/platform/README.md).

## Round 1: SPEC-010 model formats, memory and rebuilds

### SPEC-010 MD-01: bge-m3 published only as pickle

Asked: bge-m3's PyTorch weights exist only as a pickle file, which the
no-pickle rule (R106) forbids; its publishers also offer an official
ONNX export, which is data only. Offered: "Use official ONNX copy
(Recommended)", "Convert once in a sealed lab", "Choose another
embedder".

```text
Use official ONNX copy (Recommended)
```

### SPEC-010 MD-02: en_core_web_trf hides a pickle

Asked: en_core_web_trf holds its transformer weights in an embedded
pickle that spaCy opens itself; en_core_web_lg is data only. Both were
checked on Dell from the Hugging Face API. Offered: "Use
en_core_web_lg (Recommended)", "Convert once in a sealed lab", "Add a
separate safe name model".

```text
Use en_core_web_lg (Recommended)
```

### SPEC-010 MD-03: everyday memory over the 6 GB cap

Asked: by estimate, everyday use needs about 5,980 MB, past the safety
line of the 6 GB service cap, and about 7,150 MB while the visual map
is drawn. Offered: "Raise to 8 GB if confirmed (Recommended)", "Keep
6 GB, take turns".

```text
Raise to 8 GB if confirmed (Recommended)
```

### SPEC-010 MD-04: rebuilding with a second large model

Asked: how a rebuild with a second bge-m3-sized model fits beside the
serving one. Offered: "Take turns, keep suggesting (Recommended)",
"Pause suggestions during rebuild".

```text
Take turns, keep suggesting (Recommended)
```

## Round 2: SPEC-010 community models, the lock file and retraining

### SPEC-010 MD-05: a community model slower than 30 seconds

Asked: whether an administrator may activate a community model whose
trial projects a first suggestion slower than 30 seconds, where the
active model meets it. Offered: "Refuse it (Recommended)", "Allow
after a warning". The owner answered, and asked for MD-04 again:

```text
allow after a warning.

now ask me this question again:

 When a model is updated, the whole index has to be rebuilt with the new model while the old one keeps serving, which means two large models at once. How should that work?
```

### SPEC-010 MD-06: models.lock and the file limits

Asked: how `models.lock` counts under the 300-line limit. Offered:
"Exempt, tool-written (Recommended)", "Split into one file per model".

```text
Exempt, tool-written (Recommended)
```

### SPEC-010 MD-07: how a lab-trained model reaches use

Asked: how a model retrained in the Dell lab on a forum's decisions
reaches that forum once approved. Offered: "Site-only, never published
(Recommended)", "Verified for every forum".

```text
Site-only, never published (Recommended)
```

### SPEC-010 MD-08: the bar for a retrained model

Asked: what a retrained model must show before it is put to the owner.
Offered: "Better, with small tolerance (Recommended)", "Same bar as a
challenger".

```text
Better, with small tolerance (Recommended)
```

## Round 3: SPEC-010 MD-04 again, erasure, licences and proposals

### SPEC-010 MD-04, asked again at the owner's request

Asked again, in fuller words: during a rebuild for a new embedding
model, both large models cannot comfortably run at full speed side by
side within the memory limit; how should the forum behave? Offered:
"Take turns, keep suggesting (Recommended)", "Pause suggestions during
rebuild". This answer replaces the round 1 answer to MD-04.

```text
Pause suggestions during rebuild
```

### SPEC-010 MD-09: training content later deleted or erased

Asked: what happens to a retrained model when content it learned from
is deleted or erased. Offered: "Flag, replace within 30 days
(Recommended)", "Withdraw at once".

```text
Flag, replace within 30 days (Recommended)
```

### SPEC-010 MD-10: a community model with no licence

Asked: whether an administrator may use a community model that states
no licence. Offered: "Refuse it (Recommended)", "Allow after a
stronger warning".

```text
Allow after a stronger warning
```

### SPEC-010 proposals MP-01 to MP-18

Asked: whether to accept SPEC-010's 18 technical proposals as a group.
Offered: "Accept all 18 (Recommended)", "Ask me some separately".

```text
Accept all 18 (Recommended)
```

## Round 4: SPEC-008 first run, languages, reviews and hubs

### SPEC-008 Q-RD01: the first reading run takes weeks

Asked: the first full reading is estimated at one to several weeks per
million posts with full-precision bge-m3 on an ordinary processor,
longer than the "days" the owner had accepted. Offered: "Accept it
(Recommended)", "More cores while it lasts", "Recent years first". The
owner chose "Other":

```text
Accept it, but allow paid embeddings from Google and open ai also
```

### SPEC-008 Q-RD02: a paragraph in another language

Asked: how a mostly English post with a paragraph in another language
is read. Offered: "Skip that paragraph (Recommended)", "Judge by main
language".

```text
Judge by main language
```

### SPEC-008 Q-RD03: resource reviews

Asked: whether resource reviews are read. Offered: "No reviews
(Recommended)", "Yes, read reviews too", "Description only".

```text
No reviews (Recommended)
```

### SPEC-008 Q-RD04: hub pages

Asked: how hub pages are read. Offered: "Own text only (Recommended)",
"Also listed titles, low weight", "Also suggest links in hubs".

```text
Own text only (Recommended)
```

## Round 5: paid embeddings, reactions and the first-post summary

### Follow-up to Q-RD01: how paid embeddings fit

Asked: vectors from different models cannot be mixed in one index, so
a paid service cannot help bge-m3 finish faster, and it would send
forum posts to Google or OpenAI; how should paid embeddings fit?
Offered: "Optional, admin's choice (Recommended)", "Used by default
with a key", "Dell lab comparison only". The chosen option was
described as: "Off by default. An administrator may choose a Google or
OpenAI embedding model instead of bge-m3 for the whole index, entering
their own key, with a spending cap. Only guest-visible text is sent.
Switching back to bge-m3 means a rebuild."

```text
Optional, admin's choice (Recommended)
```

### Follow-up to Q-RD01: where paid embeddings are specified

Asked: where the paid-embeddings work is written up. Offered:
"Separate spec, drafted next (Recommended)", "Fold into SPEC-010 now".

```text
Separate spec, drafted next (Recommended)
```

### SPEC-008 Q-RD05: what "highest-reacted" counts

Asked: what counts as a thread's highest-reacted posts. Offered:
"Exclude Sad and Angry (Recommended)", "All reactions equally",
"XenForo's reaction score".

```text
All reactions equally
```

### SPEC-008 Q-RD06: reading the first post for the summary

Asked: how the first post is read "in full, up to the model's limit"
for the summary. Offered: "Average of its pieces (Recommended)", "One
input up to 8,192 tokens".

```text
Average of its pieces (Recommended)
```

## Round 6: SPEC-008 vectors, index and proposals; SPEC-009 begins

### SPEC-008 Q-RD07: vector precision

Asked: how precisely vectors are stored. Offered: "32-bit
(Recommended)", "16-bit".

```text
32-bit (Recommended)
```

### SPEC-008 Q-RD08: where the vector index lives

Asked: where the vector index lives, given that a search service is a
requirement for linking. Offered: "Service's own folder
(Recommended)", "In the search service".

```text
Service's own folder (Recommended)
```

### SPEC-008 proposals P-RD01 to P-RD14

Asked: whether to accept SPEC-008's 14 technical proposals as a group.
Offered: "Accept all 14 (Recommended)", "Ask me some separately".

```text
Accept all 14 (Recommended)
```

### SPEC-009 Q-MA01: the first useful suggestion

Asked: what counts as the first useful suggestion within 30 seconds.
Offered: "First fully checked (Recommended)", "Best search match,
provisional", "Only the finished result".

```text
First fully checked (Recommended)
```


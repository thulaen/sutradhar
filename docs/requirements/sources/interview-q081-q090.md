# Interview source: Q081–Q090

Part of the [Q061–Q090 transcript](interview-061-090.md). Status, provenance and
redaction notes are on that page.

## Q081

### Question

`````text
81. For real-time compression, which trade-off should be the default?
`````

### Options offered

`````text
1. Balance speed and storage (Recommended)
Choose and measure a fast compression setting within the memory budget, with stronger compression for older records where useful.
`````

`````text
2. Prioritise smallest storage
Use stronger compression where the memory limit permits, accepting more processor work and potentially slower reporting.
`````

`````text
3. Prioritise lowest delay
Use very fast compression, accepting larger stored files to keep collection and reporting responsive.
`````

### User answer

`````text
balance speed and storage, zen g should also support offloading storage to amazon s3 on setup or later, but once the user chooses s3 the system shouldn't keep copies on the server when the connection goes green, also there should be heartbeats and diagnostics to check if s3 is working, btw tell me the expected storage for a site that receives 1 million visitors per month
`````

## Q082

### Question

`````text
82. After verified upload to S3, may Zen G keep its settings, job queue and compact report summaries locally, while removing the uploaded detailed event files?
`````

### Options offered

`````text
1. Yes, keep operational data (Recommended)
S3 holds detailed analytics files; small local records keep the add-on responsive and track what has been uploaded.
`````

`````text
2. Move summaries to S3 too
Keep only essential settings, queue entries and file-location records locally; reports depend more heavily on S3 access.
`````

### User answer

`````text
Yes, keep operational data (Recommended)
`````

## Q083

### Question

`````text
83. If S3 becomes unavailable, what should happen to new analytics events?
`````

### Options offered

`````text
1. Use a capped temporary queue (Recommended)
Temporarily store events locally, retry uploads and remove verified batches; stop collection and show gaps if the queue fills.
`````

`````text
2. Stop collecting immediately
Keep no local event queue; show the missing-data period and resume collection when S3 returns.
`````

### User answer

`````text
Use a capped temporary queue (Recommended)
`````

## Q084

### Question

`````text
84. After detailed records expire at 90 days, how long should genuinely anonymous combined statistics remain by default?
`````

### Options offered

`````text
1. Two years (Recommended)
Support year-on-year comparisons while keeping a predictable cleanup schedule.
`````

`````text
2. Five years
Support longer-term comparisons with more storage and maintenance.
`````

`````text
3. Until the owner deletes them
Keep long-term anonymous history, with storage limits and clear usage warnings.
`````

### User answer

`````text
Two years (Recommended)
`````

## Q085

### Question

`````text
85. What should qualify as a distinct method toward the required 6,000?
`````

### Options offered

`````text
1. Different reasoning procedure (Recommended)
Require a distinct calculation or decision procedure, its own purpose and distinguishing tests; names, weight changes and rearranged presets do not count.
`````

`````text
2. Documented research variants too
Also count published variants when their behaviour differs meaningfully and separate tests demonstrate that difference.
`````

### User answer

`````text
different reasoning procedure plus documented research variants too.
`````

## Q086

### Question

`````text
86. If an enabled method needs evidence that does not exist yet, such as enough click history, what should it do?
`````

### Options offered

`````text
1. Wait visibly for its evidence (Recommended)
Keep it enabled with its positive configured weight, label it as awaiting data, and do not invent an output.
`````

`````text
2. Use an approved substitute
Use a documented alternative input where the method permits it, clearly label the estimate, and wait when no valid substitute exists.
`````

### User answer

`````text
Wait visibly for its evidence (Recommended)
`````

## Q087

### Question

`````text
87. If a method reaches its time or memory allowance before finishing, how should the engine respond?
`````

### Options offered

`````text
1. Save progress and continue later (Recommended)
Use bounded chunks where the method supports them; otherwise report the unfinished work and reject unsupported methods during qualification.
`````

`````text
2. Restart with less detail
Use a separately tested approximation or smaller sample, clearly label it, and report failure when no valid reduced version exists.
`````

### User answer

`````text
all methods and all things should support checkpoint even when the server is rebooted they should support resiliance that's why we need chaos testing, every method should finish, and methods shouldn't fight for ram.
`````

## Q088

### Question

`````text
88. When the forum is busy, should analysis slow down to protect readers’ page-loading speed?
`````

### Options offered

`````text
1. Protect forum responsiveness (Recommended)
Reduce background work under load, show the growing queue and resume faster processing when capacity returns.
`````

`````text
2. Reserve steady analysis capacity
Give analysis a fixed share of the server even during traffic peaks, so completion times are more predictable.
`````

### User answer

`````text
Reserve steady analysis capacity
`````

## Q089

### Question

`````text
89. Your earlier target was 30 seconds. Should that mean the first useful suggestion, or the completed result from every applicable method?
`````

### Options offered

`````text
1. First useful suggestion (Recommended)
Show an early result within the tested workload target, then visibly update it as the remaining methods finish.
`````

`````text
2. Every applicable method finished
Treat 30 seconds as a full-analysis acceptance requirement; release is blocked if the qualified workload cannot meet it.
`````

### User answer

`````text
First useful suggestion (Recommended)
`````

## Q090

### Question

`````text
90. May staff approve an early link suggestion while other methods are still analysing it?
`````

### Options offered

`````text
1. Yes, after essential checks (Recommended)
Allow approval once relevance, permissions and content-safety checks pass; clearly show that wider analysis is still running.
`````

`````text
2. Wait for the complete analysis
Display early suggestions for inspection, but enable approval only after every applicable method finishes.
`````

### User answer

`````text
Wait for the complete analysis
`````

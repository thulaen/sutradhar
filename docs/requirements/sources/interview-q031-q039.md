# Interview source: Q031–Q039

Part of the [Q031–Q060 transcript](interview-031-060.md). Status, provenance and
redaction notes are on that page.

## Q031

### Question

`````text
31 of 200 — Which arrangement should we design for using Python libraries?
`````

### Options offered

`````text
1. PHP plus local Python (Recommended)
Keep forum controls in PHP 8.5 and run a packaged Python helper on your server; include both in the 2 GB allowance.
`````

`````text
2. PHP plus hosted Python
Send approved work to a separately hosted Python service, with its own cost, privacy rules and declared memory allowance.
`````

`````text
3. PHP-only installed product
Use Python during research and testing, but implement the shipped functionality in PHP.
`````

### User answer

`````text
explain to me all options in detailed how it would install and how things would work,
`````

## Q032

### Question

`````text
32 of 200 — What should support for more than 200 Python libraries mean?
`````

### Options offered

`````text
1. Tested optional packs (Recommended)
Provide verified integrations in selectable packs, loading only what a job needs and documenting each library's actual contribution.
`````

`````text
2. All installed by default
Ship more than 200 reviewed libraries together, while still loading only the ones required for a job.
`````

`````text
3. Usefulness over a fixed count
Admit libraries only when they measurably help, without making 200 a release requirement.
`````

### User answer

`````text
Tested optional packs (Recommended)
`````

## Q033

### Question

`````text
33 of 200 — Given that clean-room code does not bypass patents, which admission rule should Zen G use?
`````

### Options offered

`````text
1. Exclude unresolved patents (Recommended)
Keep methods with active or uncertain patent concerns out until evidence resolves the concern; research alternatives.
`````

`````text
2. Consider licensed methods
Allow a method only after documented permission and review confirm the intended use is covered.
`````

`````text
3. Specialist review first
Require a qualified patent professional's review before admitting any method drawn from a patent.
`````

### User answer

`````text
Exclude unresolved patents (Recommended)
`````

## Q034

### Question

`````text
34 of 200 — What browser-based control panel do you use to manage your forum’s server? Enter its name if it is not listed.
`````

### Options offered

`````text
1. Plesk
Investigate a point-and-click installation route through your existing Plesk setup.
`````

`````text
2. cPanel / WHM
Investigate the available installation and background-service controls on your particular server.
`````

`````text
3. None or not sure
Choose a supported graphical installation route before promising that setup needs no commands.
`````

### User answer

`````text
cloudpanel, but i want this addon not to require control panels all things should be done on the xenforo acp, there should be no dependecies of using control panel, please everything should be designed for noobs and nothing else, noob friendly is the goal, point and click everything on xenforo's admin panel, no command line what so ever.
`````

## Q035

### Question

`````text
35 of 200 — Should Python perform analysis only, with PHP remaining responsible for permissions, approvals and changes to forum content?
`````

### Options offered

`````text
1. Analysis only (Recommended)
Python returns findings and proposals; PHP checks permissions and saves only authorised changes.
`````

`````text
2. Analysis and its own records
Python also manages its analysis records, while PHP retains control over approvals and forum content.
`````

`````text
3. Broader framework responsibilities
Allow Python to run additional framework features; define which part controls each action before implementation.
`````

### User answer

`````text
Analysis only (Recommended)
`````

## Q036

### Question

`````text
36 of 200 — Must installed Mars extensions share Zen G’s existing 2 GB allowance?
`````

### Options offered

`````text
1. Share the 2 GB (Recommended)
Extensions must fit within the same total; incompatible or oversized combinations cannot run together.
`````

`````text
2. Separate declared allowances
Keep the core within 2 GB, but allow extensions additional memory that is clearly shown before enabling them.
`````

`````text
3. Choose per extension
Let administrators decide whether an extension shares the core allowance or receives a separate allocation.
`````

### User answer

`````text
Separate declared allowances
`````

## Q037

### Question

`````text
37 of 200 — Which server operating systems must the first release support? This concerns the forum server, not your personal computer.
`````

### Options offered

`````text
1. Linux only (Recommended)
Support an explicit, tested set of Linux versions and prepare the Python packages for those systems.
`````

`````text
2. Linux and Windows
Build and test separate installers, background processing and recovery for both server families.
`````

`````text
3. Your server first
Qualify your exact server setup first and expand the supported list after release.
`````

### User answer

`````text
Linux only (Recommended)
`````

## Q038

### Question

`````text
38 of 200 — How should a beginner choose which optional analysis packs to install?
`````

### Options offered

`````text
1. Recommend from their goals (Recommended)
Ask a few plain-English setup questions, recommend suitable packs, and show download size and resource needs before installation.
`````

`````text
2. Install a standard collection
Start everyone with the same tested collection and let them add or remove packs afterward.
`````

`````text
3. Choose from a catalogue
Let owners browse capability descriptions and select each pack themselves.
`````

### User answer

`````text
install everything
`````

## Q039

### Question

`````text
39 of 200 — How should Zen G and its analysis packs receive updates?
`````

### Options offered

`````text
1. Notify, then approve (Recommended)
Show what changes and let an administrator start the checked update from XenForo's admin panel.
`````

`````text
2. Security fixes automatically
Install verified security fixes automatically during an agreed maintenance window; ask before other changes.
`````

`````text
3. All verified updates automatically
Install compatible, verified updates during an agreed maintenance window and report the results.
`````

### User answer

`````text
Notify, then approve (Recommended)
`````

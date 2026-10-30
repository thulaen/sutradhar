# Interview source: Q105–Q114

Part of the [Q091–Q120 transcript](interview-091-120.md). Status, provenance and
redaction notes are on that page.

## Q105

### Question

`````text
105. Before Zen G invites a likely expert to answer an unanswered thread, who should approve the invitation?
`````

### Options offered

`````text
1. A moderator approves (Recommended)
Suggest suitable members and an invitation; staff review it, and member preferences and frequency limits still apply.
`````

`````text
2. Automatic for opted-in experts
Send invitations automatically only to members who explicitly joined the expert programme, subject to frequency limits.
`````

### User answer

`````text
A moderator approves (Recommended)
`````

## Q106

### Question

`````text
106. Where should an approved topic hub live?
`````

### Options offered

`````text
1. A normal forum thread (Recommended)
Publish it as a clearly labelled guide thread, optionally pinned, using XenForo’s existing editing and permissions.
`````

`````text
2. A resource entry
Publish it through the Resource Manager, with its description, updates and existing resource permissions.
`````

`````text
3. A dedicated hub page
Create a separate page type with its own layout, permissions and link-destination support.
`````

### User answer

`````text
a dedicated hub page, but this feature should ship optional
`````

## Q107

### Question

`````text
107. Should topic hubs have a higher link limit than the normal five-link limit?
`````

### Options offered

`````text
1. Separate configurable hub limit (Recommended)
Allow more curated links in explicitly designated hubs, while keeping the normal discussion limit unchanged.
`````

`````text
2. Keep five links everywhere
Require hubs to select only five destinations, or divide a large topic into separate hubs.
`````

### User answer

`````text
Keep five links everywhere
`````

## Q108

### Question

`````text
108. How should staff approve the links in a newly drafted hub?
`````

### Options offered

`````text
1. Approve each link individually (Recommended)
Keep your existing rule: every proposed destination and anchor receives its own approval before the hub is published.
`````

`````text
2. Approve the complete hub
Allow one explicit approval of the fully previewed page to approve all its displayed links together.
`````

### User answer

`````text
Approve each link individually (Recommended)
`````

## Q109

### Question

`````text
109. When the optional hub module is enabled, may ordinary threads and resources receive suggestions linking to an approved hub page?
`````

### Options offered

`````text
1. Yes, approved hubs too (Recommended)
Add published, publicly readable hubs to eligible destinations; all relevance, duplicate and approval checks still apply.
`````

`````text
2. No, hubs only link outward
Keep automatic destination discovery limited to threads and resources.
`````

### User answer

`````text
Yes, approved hubs too (Recommended)
`````

## Q110

### Question

`````text
110. Who may request an AI-generated summary of a long thread?
`````

### Options offered

`````text
1. Staff only (Recommended)
Staff request a draft through a connected agent, review it, and approve its display.
`````

`````text
2. Members may request; staff approve
Members can submit a rate-limited request, but staff control generation and public approval.
`````

### User answer

`````text
staff only, also

Hot, Rising, Top, Best, and Controversial sorting. Reddit’s original formulas are public. Hot combines a post’s score with how recent it is. Best ranks replies by a confidence formula (the Wilson score), so a reply with 9 upvotes and 1 downvote beats one with 50 up and 40 down. zen-g recalculates scores as votes come in, and the add-on sorts by them. Reactions count as votes.

----

Karma. Each member gets a score from the reactions they receive, overall and per forum.

----

Alt-account signals. Flag new accounts that resemble banned ones (shared IPs, similar writing style). Treat these flags as hints for moderators, never as automatic bans, and make sure your privacy policy covers this.
`````

## Q111

### Question

`````text
111. If new replies arrive after a summary is approved, should the old summary remain visible?
`````

### Options offered

`````text
1. Show exactly what it covers (Recommended)
Keep it with its date and last covered reply, and offer a refresh request; hide affected text if source deletion or access changes make it unsafe to show.
`````

`````text
2. Hide until refreshed
Remove the summary from public view after new replies arrive, until a replacement is approved.
`````

### User answer

`````text
Show exactly what it covers (Recommended)
`````

## Q112

### Question

`````text
112. How should Zen G decide whether a reaction is a positive, negative or neutral vote?
`````

### Options offered

`````text
1. Use XenForo’s existing settings (Recommended)
Follow the administrator’s reaction scores and meanings; do not assume that a sad or angry face is a downvote.
`````

`````text
2. Separate Zen G mapping
Let administrators choose separate vote meanings for ranking and karma without changing XenForo’s own reaction scores.
`````

### User answer

`````text
follow the administrators reaction scores and meanings plus assume that a sad or angry face is a downvote, and all reactions should be fed to the ranking engine so that it learns.
`````

## Q113

### Question

`````text
113. Who should see a member’s overall and per-forum karma?
`````

### Options offered

`````text
1. Members and visitors (Recommended)
Show public scores based only on public content, with a plain explanation of how they are calculated.
`````

`````text
2. The member and authorised staff
Let people inspect their own score without publishing a public reputation ranking.
`````

`````text
3. Authorised staff only
Use karma as an internal moderation and expert-routing aid.
`````

### User answer

`````text
Members and visitors (Recommended)
`````

## Q114

### Question

`````text
114. When should Zen G raise a possible alternate-account flag?
`````

### Options offered

`````text
1. Require supporting signals (Recommended)
Require more than one sufficiently independent indicator and explain uncertainty; a shared IP or writing similarity alone does not trigger the flag.
`````

`````text
2. Show single-signal leads too
Also show isolated clues in a separate low-confidence list, clearly marked as insufficient to identify anyone.
`````

### User answer

`````text
Require supporting signals (Recommended)
`````

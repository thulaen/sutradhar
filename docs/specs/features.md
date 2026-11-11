# Feature list

This is the full list of Sutradhar's features, numbered 1 to 1357.
Version 1 holds all of them and the expanded scope in spec 0064.
Each behaviour has one owning design and build task. Roadmap Phase 6
builds the features in four
ranges, in this order:

1. Features 1 to 112: search features for XenForo and WordPress.
2. Features 113 to 712: the wider feature catalogue.
3. Features 713 to 1312: content features that AI assistants use
   through Sutradhar's MCP server, and the staff access around them.
4. Features 1313 to 1357: crawler, monitoring, bot and analytics
   additions, and two content tools.

Clean-room rule. Every feature is our own design. A feature that works
"as" another product does names the idea only. We learn what exists
from public pages and documents, never how it is built. We copy no
code, wording, screens, data, formulas or trademarked names. Features
that come close to another product get our own design, set out in the
"Clean-room notes" at the end of this list.

Within each range, features are built in four kinds, in this order:
insight-only features, then review-page changes, then drafts, then
public-page features.

Rules that hold for every feature:

- Google's guidelines come first. Every visitor and every search engine
  sees the same page.
- Each feature works on both XenForo and WordPress, or the app shows it
  as "not applicable" with the reason.
- Every change to a site goes through the review page, needs a person's
  Accept, applies through the platform's own functions and can be undone.
  No setting, workflow, script or token accepts a change in a person's
  place (owner, 11 November 2026).
- A post to a social network goes out only after a person approves it
  through feature 378.
- Every feature that changes a site records its effect through feature
  20, before-and-after proof.
- A feature reports a number only when `gate/metrics.toml` lists it with
  one approved source, a decision and a comparison (ADR-0013); it never
  blends different measures or sources into one score.
- Features that use reader behaviour read only darshak's grouped data,
  which carries no cookies (spec 0034). darshak adds differential
  privacy (added noise that hides any one person) from the OpenDP
  library to figures for small groups.
- No feature touches members' private data, private drafts or private
  messages.
- Nudges shown to members are off until the owner turns them on. They
  have a rate limit, and each member can turn them off.
- Where an SEO plugin on the site, such as Yoast or Rank Math, already
  does the job, the feature leaves it to that plugin and says so in the
  app.
- New content is only ever a draft. It cites the posts it came from and
  never invents facts. Staff publish it under a staff account.
- Heavy work runs on the owner's PC or the owner's cloud helpers, never
  on the site's server, at a polite pace.
- Assistants using the MCP server make drafts only. A person approves
  before anything is published. AI-written content is never posted as a
  member.
- A feature that learns something about a page passes it to link
  ranking only as a typed signal. A signal can only reorder links that
  already passed the relevance gate; popularity and traffic never make
  an off-topic link rank higher (spec 0027, "Signals from other
  features").

Tags at the end of a feature:

| Tag | Meaning |
|---|---|
| [Own account] | Needs the owner's own account with that service, which is free to create. |
| [Paid] | Needs the owner's own paid account or key. Costs are shown first and the owner can cap them. |
| [Approval] | The platform must approve Sutradhar's app before this can work. |
| [Public] | Adds something to public pages and needs a footprint budget in its specification. |
| [WP] | WordPress only. |
| [XF] | XenForo only. |

Features 19, 33, 35, 42, 43, 46 and 55, and features 107 to 112, also
change public pages and need footprint budgets.

Numbers 1000, 1284 and 1285 are historical references to features
933, 1283 and 1165. The work once placed in those slots belongs to
editor scoring (235, task #56), the draft intake (714, task #57), and
the writing brief (36, task #54).

## Range 1: features 1 to 112

### Linking and site structure

1. What-if simulator: Before a batch of changes is applied, show the predicted
   shift in internal link value across the site as a map. Also list the pages
   that gain and lose.
2. Inbound links for new content: When a thread or post is published, suggest
   links to it from the most relevant, strongest existing pages.
3. Orphan rescue: List pages with no internal links, ranked by search demand,
   with link suggestions.
4. Click-depth flattener: Find valuable pages more than 3 clicks from the home
   page. Suggest hub or link placements to bring them closer.
5. Anchor profile guard: Track the anchor texts pointing at each page. Block
   suggestions that would make the profile unnatural, for example too many
   identical exact-match anchors.
6. Moved-page fixer: Re-point internal links that go to merged, moved or
   redirected pages straight to the final page.
7. Links to the answering post: When the answer sits deep in a long thread, link
   to that post's own permalink.
8. Seasonal boosts: Learn seasonality from Search Console. Promote links to
   seasonal pages ahead of rising demand.
9. Glossary pages: Detect forum jargon and draft one definition page per term.
   Link uses of the term to that page.
10. Forum and blog bridge: When one owner pairs both a XenForo forum and a
    WordPress site to the same home, suggest relevant links between them in both
    directions.

### Search Console intelligence

11. Striking-distance finder: Find pages ranked 8 to 20 with high impressions.
    Suggest links and title improvements to reach page 1.
12. Click-rate anomalies: Find pages whose click rate is well below what is
    expected for their position, as modelled by ganit. Suggest snippet and title
    changes.
13. Ranking-drop forensics: When a page drops, list the likely causes in plain
    English, with evidence. Causes include content edits, lost links, indexing
    changes or a Google update date.
14. Google update timeline: Overlay Google's announced updates, taken from the
    Search Status Dashboard, on traffic. Break this down by forum section or
    category.
15. Indexing watcher: Check the most important pages through the URL Inspection
    API, within quota. Explain and fix any that are not indexed.
16. Cannibalisation radar: Find pages competing for the same queries, from query
    and page data. Suggest fixes: merge, differentiate or link.
17. Vocabulary gap: Find queries where searchers use different words from
    members. Suggest adding the searchers' wording to titles or first posts.
18. Image search harvesting: Find images getting Google Images impressions, and
    link them to and from their threads.
19. Google Discover readiness: Check eligibility, use the large image preview
    setting where appropriate, and track Discover traffic. This needs a
    public-page exception.

### Proof and planning

20. Before-and-after proof: Measure every applied change against a matched
    control group of similar unchanged pages. Report the effect with confidence
    intervals, for example "+312 clicks in 30 days, ±40".
21. SEO split testing: Change titles or descriptions on a random half of a group
    of similar pages, compare with the other half over a set period, and keep
    the winner. Every visitor and search engine sees the same version of each
    page.
22. Weekly top three: Show the three actions with the highest expected impact
    each week. Each one takes one click to queue for review.
23. 90-day forecast: Forecast traffic with and without the queued changes,
    showing the uncertainty.
24. Search demand while editing: While staff edit a title in the editor, show
    search demand for similar queries. Use the editor link map approach, with no
    requests while typing.

### Content

25. Thin-but-wanted rescue: Flag high-impression pages with thin or weak answers
    to staff and experts.
26. Expert routing: Send unanswered questions to members who answered similar
    ones, only if they have opted in, through XenForo alerts. On WordPress, do
    this only where a forum plugin provides notifications.
27. Answer completeness: Score whether accepted answers really answer the
    question, and flag weak ones to staff.
28. Community consensus: Draft "most recommended" sections for hubs, for example
    "Most recommended: Brand X, mentioned in 214 posts". Link each claim to its
    posts.
29. "State of" reports: Draft reports from aggregated community data (polls,
    recommendations and trends) that others can cite and link to. Include no
    member personal data.
30. Comparison pages: Detect "X versus Y" debates across threads, and draft
    comparison pages that cite the posts.
31. Product pages: Recognise products and models, using Wikidata where
    available. Draft one page per product that gathers its discussions.
32. Pruning advisor: For old, low-value pages, suggest improve, merge or
    noindex, with the traffic evidence for each.
33. Mega-thread contents: Add a contents list and page-by-page summaries for
    very long threads. This needs a public-page exception.
34. Thread splitting: Suggest splitting threads that answer several separate
    questions. The platform handles redirects for moved posts.
35. Summaries with sources: Add short summaries at the top of long threads, each
    point linked to its source post and approved by staff. This needs a
    public-page exception.
36. Blog briefs: Draft article outlines for WordPress authors from questions
    asked on the forum. The brief needs enough eligible source posts or
    pages, as [spec 0041](0041-editorial-coach.md) sets out.

### Technical health

37. Tag clean-up: Merge duplicate and near-duplicate tags, and suggest noindex
    for thin tag pages.
38. Duplicate titles: Find identical or near-identical titles, and suggest
    distinct, descriptive ones.
39. Soft-404 detector: Find empty, deleted or placeholder pages that still
    return "200 OK".
40. Canonical conflicts: Find pages whose canonical tag contradicts their
    redirects, sitemap or internal links.
41. Pagination health: Check that paginated threads and lists are crawlable and
    canonicalised consistently.
42. Broken-URL recovery: Old URLs that still receive links or traffic, for
    example from a vBulletin migration, get redirect suggestions to the right
    new pages. This needs a public-page exception.
43. Hreflang links: Add correct language-alternate links for multilingual
    sections. This needs a public-page exception.
44. Layout-shift fixer: Find images without dimensions and other causes of
    layout shift, with fixes.
45. Speed by page type: Group pages by template, and identify which template
    causes poor Core Web Vitals.
46. Googlebot crawl tracker: Keep a sampled, lightweight record of requests from
    Googlebot, verified against Google's published IP ranges, showing what
    Google crawls, how often, and crawls wasted on junk pages. This needs a
    public-page exception, and it is a request hook, not page output.
47. Outbound link labels: Check that sponsored, user-generated and nofollow
    labels are correct.
48. Signature and profile link audit: Find spammy outbound links in signatures,
    profiles and comment author links.

### Topical authority

49. Authority map: Build a knowledge graph of the site's topics and entities,
    showing strong and weak areas.
50. Hub completeness: Find subtopics people search for that a hub does not cover
    yet.
51. Rival topic gap: Find topics covered by rival sites that the owner names.
    Read only their public sitemaps, respecting robots.txt, at a polite pace.

### Reader behaviour

52. Link satisfaction score: Measure whether readers who click a Sutradhar link
    stay and read, and feed this back into ranking.
53. Drop-off map: Show where readers stop scrolling in each thread, with
    suggestions such as moving the answer up.
54. Exit-page rescue: Find pages where most visitors leave the site, and suggest
    better next-step links.

### AI search

55. AI crawler control centre: Allow or block named AI crawlers (for example
    GPTBot, ClaudeBot, Google-Extended and PerplexityBot) one by one, with
    plain-English trade-offs, through WordPress's robots filter and XenForo's
    robots.txt setting. This screen never touches Googlebot or Bingbot, and it
    needs a public-page exception.
56. AI referral analytics: Show traffic arriving from AI assistants (for example
    ChatGPT, Perplexity, Gemini and Copilot), and which pages it lands on.

### Structure and linking

57. Category re-homing: Suggest moving threads posted in the wrong forum section
    or category to the right one.
58. Query-aligned anchors: Prefer anchor wording that matches the target page's
    real search queries. Stay within the anchor profile guard (5).
59. Hub-and-spoke check: Check that every hub links to its threads, and every
    thread links back to its hub.
60. Link placement: Prefer placing suggested links in the main text, near the
    relevant passage, over closing lines.
61. Link density guard: Warn when a page carries so many links that each one's
    value is diluted, and cap suggestions there.
62. Internal nofollow audit: Find internal links wrongly marked nofollow.
63. Navigation suggestions: Find sections that earn significant search traffic
    but are missing from menus or navigation. Suggest placement in WordPress
    menus or XenForo navigation.
64. Sticky thread audit: Find stickies and WordPress sticky posts that hog link
    value without earning it, and suggest them for rotation.
65. Sister-site network: When one owner pairs several XenForo forums or several
    WordPress sites to the same home, suggest relevant links between them.
66. WooCommerce bridge: On WordPress only, link shop product pages and the forum
    threads discussing those products, in both directions.
67. Resources and gallery linking: On XenForo only, where Resource Manager or
    Media Gallery is installed, include their items in the linking graph.
68. Internal link click map: Show which internal links readers click, and where
    on the page, using darshak data.

### Search data and insight

69. Rich result tracker: Show which pages appear with rich results or in
    Google's "Discussions and forums" feature, and which dropped out. The data
    comes from Search Console's search appearance data.
70. Branded versus non-branded traffic: Split traffic by whether the searches
    contain the site's own name.
71. Search intent labels: Label each page and query as informational, commercial
    or navigational.
72. Emerging query alerts: Alert on queries appearing for the first time, or
    rising fast.
73. Mobile versus desktop gap: Find pages ranking very differently on mobile and
    desktop, with the likely reason.
74. International demand map: Show impressions by country and language, feeding
    translated editions (98).
75. Time-to-index tracker: Track how long new pages take to be indexed, using
    sampled URL Inspection checks within quota.
76. Demand versus activity: Compare search demand with posting activity per
    section.
77. Evergreen detector: Classify pages as evergreen or time-sensitive. The link
    value, freshness and expiry features use this.
78. Your-changes timeline: Mark theme, plugin, platform, settings and Sutradhar
    changes on traffic graphs.
79. Goal tracker: Show owner-set goals, with progress and the changes
    contributing most.
80. Search Console setup guardian: Check verification, ownership and submitted
    sitemaps, and explain fixes in plain English. Submit sitemaps through the
    API only after the owner approves.

### Protection

81. SEO regression alarm: After any detected theme, plugin or platform update,
    re-check a sample of pages for changed titles, canonicals, noindex tags,
    robots rules and structured data. Alert within hours.
82. Script visibility gap: Compare raw HTML with the rendered page, using a
    headless browser on the home. Flag content that only appears after the
    page's scripts run.
83. HTTPS and mixed content: Find insecure resources on secure pages.
84. Intrusive pop-up audit: Find overlays and interstitials that Google's
    guidance discourages, in rendered pages.
85. Image weight audit: Find oversized images and outdated formats, with
    suggestions.
86. Dead embed repair: Find deleted or broken video and media embeds in posts,
    and suggest replacements through the review page.
87. Date consistency: Check that the visible, structured-data and sitemap dates
    agree.
88. Site name and favicon check: Report problems and guide the fix. Any markup
    Sutradhar adds falls under the structured-data exception.

### Content quality

89. Quote bloat detector: Find replies made mostly of quoted text, and suggest
    them to moderators for trimming.
90. Thread noise reduction: Suggest converting "+1" and "same here" replies into
    reactions on XenForo. This is not applicable on WordPress without a plugin
    that provides reactions.
91. Troubleshooting guides: Draft step-by-step guides built from solved threads,
    citing them.
92. Version tagging: Tag answers by product or software version, using tags or
    prefixes.
93. Expired content handler: Mark ended sales, past events and closed offers as
    ended, with a link to what is current.
94. Featured snippet coach: Suggest formatting (numbered steps, short
    definitions or tables) for pages targeting queries that show snippets.
95. Citation suggestions: Suggest links to official documentation, from an
    owner-approved list of trusted domains, with correct link labels.
96. Article FAQ enrichment: On WordPress only, draft real forum questions as FAQ
    sections for related articles.
97. Archive page intros: Draft introductions for thin category, tag and forum
    description pages.
98. Translated editions: Draft translations of top answers for countries with
    demand, clearly labelled as translations and reviewed by staff before
    publishing. Connect them with hreflang under the existing exception (43).
99. Abandoned draft rescue: On WordPress only, find staff drafts matching search
    demand. Never use members' private drafts.
100. Content calendar: Turn seasonal demand into a staff plan with dates.

### Members and staff

101. Duplicate question interception: While a member types a new thread title,
     show matching answered public threads, using the cached editor map and no
     requests while typing. This is editor-only, under the editor rule.
102. Member title coaching: Give gentle suggestions for vague titles while a
     member types. This is editor-only, under the editor rule.
103. Solved-marking nudges: Remind askers to mark the answer that worked,
     through XenForo alerts. On WordPress, do this only where a forum plugin
     provides notifications.
104. Moderator section scorecards: Show moderators with permission the search
     health and the single biggest fix for each section.
105. Staff SEO coach: Attach a short plain-English lesson to every suggestion
     type.
106. Referral backlink discovery: Find external pages linking to specific
     threads from darshak's referrer data. Keep the domain and path only, with
     query strings removed.

### Rich results and media (each needs an approved footprint exception)

107. Event detection: Add Event structured data for meetup and launch threads,
     validated by Rust shape checks. This needs an approved footprint exception.
     The shape checks are our own Rust rewrite of SHACL checks, with the same
     aim.
108. Video key moments: Add VideoObject structured data for embedded videos,
     with timestamps where the thread references them. This needs an approved
     footprint exception.
109. Image licence metadata: Add licence structured data for owner-original
     images only. This needs an approved footprint exception.
110. Image and video sitemaps: Provide image and video sitemaps, unless an SEO
     plugin already provides them. This needs an approved footprint exception.
111. Text-and-data-mining opt-out: Set a TDMRep reservation, following the
     current specification, as the owner chooses. This needs an approved
     footprint exception.
112. Expert verification: Owner-designated experts can mark answers "verified",
     shown as a visible label. This needs an approved footprint exception.

## Range 2: features 113 to 712

### Site crawler and technical audit

113. Built-in site crawler: A site crawler that runs on the home, at the polite
     pace, respecting robots.txt.
114. Scheduled and automatic crawls: Run crawls on a schedule, plus automatic
     crawls after detected platform, theme or plugin updates.
115. Crawl comparison: Compare any two crawls to show new, removed and changed
     pages and issues.
116. List mode: Crawl only a pasted or uploaded list of URLs.
117. Sitemap audit: Find sitemap URLs that are not linked, linked URLs missing
     from sitemaps, and sitemap URLs that redirect or error.
118. Noindexed URLs in sitemaps: List noindexed URLs that appear in sitemaps.
119. Sitemap limit and format checks: Check sitemaps against the limits of
     50,000 URLs and 50 MB, and check for valid dates.
120. Robots.txt syntax checks: Check robots.txt syntax and size.
121. Robots.txt tester: Test any URL against the current robots.txt and a draft
     one.
122. Status code report: Report every redirect and error, with the pages linking
     to each.
123. Flaky responses: Find pages that sometimes error across repeated fetches.
124. Redirect chains and loops: Find server redirect chains and loops.
125. Temporary redirect misuse: Find temporary redirects used for permanent
     moves.
126. Meta refresh and script redirects: Find meta refresh redirects and
     redirects made by the page's scripts.
127. Redirects to the home page: Find redirects to the home page instead of a
     relevant page.
128. HTTPS redirect gaps: Find gaps in HTTP-to-HTTPS redirection.
129. Www and non-www duplicates: Find duplicate www and non-www versions.
130. Trailing-slash inconsistencies: Find URLs that are inconsistent about
     trailing slashes.
131. Letter-case duplicate URLs: Find duplicate URLs that differ only in letter
     case.
132. URL structure audit: Check URL parameters, underscores, length and folder
     depth.
133. Mixed URL variants in links: Find internal links using different URL
     variants for the same page.
134. Tracking parameters in links: Find internal links carrying tracking
     parameters.
135. Links to noindexed pages: Find internal links pointing to noindexed pages.
136. Links to non-HTTPS URLs: Find internal links pointing to non-HTTPS URLs.
137. Script-only links: Find links reachable only through actions run by the
     page's scripts, which crawlers cannot follow.
138. Empty or generic anchors: Find empty or generic anchor text ("click here",
     "this") across templates.
139. Excessive template outbound links: Find templates that carry excessive
     outbound links.
140. Navigation-only internal links: Find pages whose only internal links come
     from navigation, not body content.
141. Internal link report per URL: Show links in, links out, anchors and link
     position for each URL.
142. Response time per URL: Show the response time of each URL, by section.
143. Page weight per URL: Show the weight of HTML, scripts, styles and images
     for each URL.
144. Missing or multiple titles: Find pages with missing or multiple title tags.
145. Boilerplate-only titles: Find titles that are just the site name, or
     "Untitled".
146. Title truncation preview: Preview title truncation by pixel width, as
     search results would show it.
147. Keyword stuffing check: Find keyword-stuffed titles and descriptions.
148. Meta description problems: Find meta descriptions that are missing,
     duplicated, repeated on one page or truncated.
149. Conflicting robots instructions: Find robots instructions where the HTTP
     header and the meta tag conflict.
150. Word count and text ratio: Show word count and text-to-code ratio,
     highlighting thin templates.
151. Boilerplate ratio: Compare template text with unique content.
152. Placeholder text finder: Find placeholder text left on pages ("lorem
     ipsum", "coming soon").
153. Broken images: Find broken images, and where they are used.
154. Image file name quality: Flag poor image file names such as "IMG_1234.jpg".
155. Missing viewport tag: Find pages with no viewport tag.
156. Language and encoding problems: Find language declaration and character
     encoding problems.
157. Social card audit: Audit Open Graph and similar social cards, with
     previews.
158. Feed validity checks: Check RSS and Atom feeds for validity and
     discoverability.
159. 404 page quality: Check that the 404 page has helpful links and search.
160. Mobile and desktop parity: Find content or links missing on mobile compared
     with desktop.
161. Rendered screenshots: Take rendered screenshots of each URL, on desktop and
     mobile.
162. Blocked scripts and styles: Find scripts or styles blocked by robots.txt
     that break rendering for search engines.
163. Caching and compression audit: Audit caching and compression.
164. TLS certificate expiry alerts: Alert on TLS certificate expiry.
165. Security headers audit: Audit security headers, with plain-English fixes.
166. Open redirect check: Find open redirect vulnerabilities on the site.
167. Exposed software versions: Find publicly visible software versions that
     reveal outdated software.
168. Directory listing check: Find folders with directory listing switched on.
169. Exposed sensitive files: Find exposed sensitive files such as backups, .git
     folders and logs.
170. Exposed email addresses: Find plain-text email addresses exposed to
     harvesters.
171. Cookie audit: Audit cookies, for privacy compliance.
172. Accessibility audit: Audit public pages for accessibility with axe-core,
     prioritised by traffic.
173. Third-party script inventory: List every third-party script.
174. Third-party script cost: Show the load time each third-party script adds.
175. PDF and document audit: Check documents' titles, sizes and whether they are
     indexed.
176. Hidden text and cloaking check: Check for hidden text and cloaking risk,
     catching theme or plugin mistakes.
177. Doorway-like patterns: Find many near-identical pages.
178. Invisible structured data: Find structured data describing content that is
     not visible on the page.
179. Indexed spam clean-up: Find spam URLs that reached Google's index.
180. Hacked content detection: Detect injected links and pharma spam.
181. Defacement monitor: Watch for unexpected content changes on key pages.
182. Uptime and DNS monitor: Monitor the site's uptime and DNS.
183. Custom extraction: Extract data with CSS selectors, XPath or regular
     expressions.
184. Custom search: Search for any text or code across the site.
185. Custom rule checks: Run custom checks on every crawled page, written as
     user scripts in Rhai, a small scripting language run inside Sutradhar.
186. Spelling and grammar checks: Check staff-written pages with a self-hosted
     LanguageTool. Check members' posts only if the owner opts in.
187. Crawl segments: Segment crawls by section, template or author, usable in
     every crawl report.
188. Crawl visualisation: Show site-structure diagrams, both force-directed and
     tree.
189. Traffic-based issue priority: Prioritise every issue by the traffic of the
     pages it affects.
190. Fix verification: Re-crawl only the affected URLs and confirm the fix.
191. Crawl alerts: Alert on new critical issues.
192. Redirect map verifier: Verify redirect maps for site migrations.
193. Migration checker: Compare a staging or new site with the live one before
     switching.
194. Password-protected staging crawls: Crawl staging sites that are protected
     by a password.
195. Crawl exports: Export crawls as CSV, Parquet and MCP.

### Log files and bot analysis

196. Server log analysis: Analyse uploaded or downloaded hosting logs, in common
     formats.
197. Verified bot identification: Identify verified bots in logs, using search
     engines' published IP ranges.
198. Crawl budget by section: Show the crawl budget for each section.
199. Crawl frequency versus traffic: Find pages crawled often but never visited,
     and valuable pages rarely crawled.
200. Bot status codes over time: Show the status codes served to bots over time.
201. Time to first Googlebot visit: Measure the time from publishing to
     Googlebot's first visit.
202. Bot versus human load: Compare the server load from bots with that from
     humans.
203. Fake bot detection: Detect visitors that claim to be Googlebot but are not.
204. AI crawler activity: Show AI crawler activity found in logs.
205. Crawl spike and drop alerts: Alert on sudden crawl spikes and drops.

### Keyword and topic research

206. Keyword workspace: Gather every query from Search Console and Bing,
     de-duplicated.
207. Keyword clustering: Cluster keywords by meaning.
208. Search volumes: Get search volumes from Google Ads Keyword Planner. [Own
     account]
209. Keyword difficulty: Get keyword difficulty through a SERP (search results
     page) data provider. [Paid]
210. Competitor keyword gap: Find keyword gaps against competitors. [Paid]
211. Question finder: Find every question-style query, grouped by topic.
212. Keyword-to-page mapping: Map one target page to each keyword cluster,
     flagging clusters with none.
213. Wikipedia interest trends: Show topic interest trends from Wikipedia page
     views.
214. Forum long-tail ideas: Find long-tail ideas in the forum's own language,
     kept only if real searches confirm them.
215. Owner keyword lists: Let the owner define keyword lists and tags.
216. Business value tagging: Let the owner mark which topics matter most.
217. Opportunity score: Give each keyword cluster an opportunity score.
218. Zero-click queries: Find queries with high impressions but almost no
     clicks.
219. Irrelevant query filter: Filter out queries the site should not chase.
220. Intent shift detector: Find queries whose meaning to searchers changes over
     time.
221. Semantic keyword map: Show a semantic keyword map as an interactive visual.
222. Content idea board: Show content ideas ranked from all research sources.

### Rank and visibility tracking

223. Daily position history: Keep daily position history from Search Console and
     Bing.
224. Visibility by section: Show the change in Search Console impressions and
     average position for each section, compared with the previous 28 days; no
     total for the whole site and no blended index.
225. Position distribution charts: Chart how positions are distributed.
226. Regular-expression query segments: Define query segments with regular
     expressions.
227. True rank tracking: Track true ranks, including search features, through a
     SERP data provider. [Paid]
228. Competitor share of voice: Measure share of voice against competitors.
     [Paid]
229. SERP landscape: Show which search features appear for your queries. [Paid]
230. Local rank grid: Show map-pack positions across a city. [Paid]
231. Brand SERP monitor: Watch what appears when people search the site's name.
     [Paid]
232. Position swing alerts: Alert on unusual position swings.
233. Weekly winners and losers: Show each week's winners and losers.
234. Landing page trends: Show landing page trends by section.

### Content optimisation and editorial

235. Editor content scoring: Show the same approved content score for
     the saved draft in the WordPress editor and in a small panel in the
     XenForo editor, for guest-visible drafts only, never conversation
     drafts. The service scores the saved version when it next calls the
     site and stores it through the add-on's or plugin's own contract;
     typing sends no requests to Sutradhar or their routes. [Spec
     0042](0042-coach-additions.md) owns the display.
236. Readability analysis: Analyse sentence length, passive voice and reading
     age.
237. Term coverage: Compare term coverage with the site's own best-performing
     pages on the topic.
238. Competitor page comparison: Compare with competitor pages the owner names,
     fetched politely.
239. Heading outline suggestions: Suggest heading outlines.
240. Brand voice checker: Check text against the brand voice guide (412).
241. Internal plagiarism check: Find new articles copying existing pages.
242. Content inventory: List every page with its type, author, age, traffic and
     links.
243. Editorial workflow stages: Move work through the brief, draft, review and
     published stages, with assignees.
244. Editorial calendar: Keep an editorial calendar.
245. AI draft assistant: Help staff draft articles with AI, with citations
     required.
246. Rewrite suggestions: Suggest rewrites for unclear paragraphs.
247. Image suggestions: Suggest images from the site's own media library.
248. Reusable content templates: Offer reusable templates for guides, reviews
     and comparisons.
249. Author profile completeness: Check how fully author profiles are filled in.
     [WP]
250. Performance by author: Show performance by author, category and format.
251. Approval rules by category: Set approval rules for each category.
252. Fact-check flags: Flag numbers, prices and dates in drafts for
     fact-checking.
253. Search and social previews: Show search and social previews while editing.
254. Content repurposing: Turn a thread into an article, then a social post,
     then a newsletter.
255. Experience and expertise checklist: Give each article an experience and
     expertise checklist.
256. Editor accessibility checks: Run accessibility checks inside the editor.
257. Content score history: Keep the history of content scores.
258. Internal content gap: Find topics the site mentions often but never covers
     properly.

### Competitive intelligence

259. Competitor registry: Keep a registry of competitors.
260. Competitor sitemap monitoring: Monitor competitors' sitemaps for changes.
261. Competitor page change alerts: Alert on changes to named competitor pages.
262. Competitor speed comparison: Compare speed with competitors using the
     Chrome UX Report.
263. Competitor ranking overlap: Show ranking overlap with competitors. [Paid]
264. Competitor backlink overlap: Show backlink overlap with competitors. [Paid]
265. Competitor social activity: Track competitors' social activity. [Approval]
266. Competitor structured data: Show how competitors use structured data.
267. Competitor publishing pace: Track how fast competitors publish.
268. Competitor new topic alerts: Alert when competitors start covering a new
     topic.
269. Combined competitive gap report: Combine every competitive gap into one
     report.

### Backlinks and digital PR

270. Bing inbound links: Show inbound links from Bing Webmaster Tools.
271. Search Console links import: Import Search Console's links export.
272. Provider backlink data: Get backlink data from a provider: Ahrefs, Semrush,
     Majestic or Moz. [Paid]
273. Backlink change alerts: Alert on new and lost backlinks.
274. Risky link review: Review risky links, explaining why action is rarely
     needed.
275. Disavow file manager: Manage the disavow file, used only after a manual
     action.
276. Outreach targets: Find sites linking to similar resources. [Paid]
277. Unlinked mentions: Find unlinked mentions through brand monitoring.
278. Broken backlink reclamation: List sites linking to dead URLs.
279. Outreach pipeline: Track outreach in a pipeline.
280. Outreach emails: Send outreach emails from the owner's own email account.
     [Own account]
281. PR campaign tracker: Track PR campaigns.
282. External anchor text profile: Show the anchor text profile of external
     links.
283. Link growth trends: Show trends in link growth.
284. Guest content tracker: Track guest content.

### AI search visibility

285. Prompt library: Build a library of questions people ask AI assistants, from
     searches and forum questions.
286. Persona-based prompt sets: Group prompts into sets by persona.
287. AI answer tracking: Track AI answers across chosen models, using the
     owner's API keys. [Paid]
288. Brand mention rate: Measure how often each AI model mentions the brand.
289. Citation tracking: Track which pages AI answers cite.
290. AI share of voice: Measure AI share of voice against competitors.
291. AI answer sentiment: Measure the sentiment of AI answers about the brand.
292. Accuracy checker: Find claims in AI answers that the site's own published
     pages contradict, with evidence. It keeps no stored list of facts.
293. Source gap: Find sites AI cites instead of yours, by topic.
294. AI visibility by language: Show AI visibility for each language.
295. AI answer change alerts: Alert when an AI answer about the brand changes.
296. AI citation recommendations: Recommend ways to earn AI citations.
297. AI crawler access check: Check AI crawler access for each page.
298. AI answer snapshot archive: Keep an archive of AI answer snapshots.
299. AI tracking costs: Estimate and cap the cost of AI tracking.
300. Brand facts page draft: Draft a clear, citable page of official facts, from
     the site's own pages. Staff publish it. It keeps no stored list of
     facts.
301. AI misinformation correction: Run a workflow for correcting AI
     misinformation.
302. AI product descriptions: Show how AI describes the owner's products.
303. AI competitor positioning: Show how competitors are positioned in AI
     answers.
304. Prompt clustering: Cluster prompts by intent.

### Reporting, dashboards, and forecasting

305. Scheduled reports: Send scheduled PDF and email reports.
306. White-label reports: Make white-label reports for agencies.
307. Executive summary: Show a one-screen executive summary.
308. Chart date notes: Add manual notes on any chart date.
309. Google Sheets export: Export to Google Sheets. [Own account]
310. Data warehouse export: Export to a data warehouse, BigQuery or Snowflake.
     [Own account]
311. Search registration attribution: Attribute registrations to search traffic.
312. Search traffic value: Estimate the value of search traffic using owner-set
     values.
313. Anomaly detection: Detect anomalies across every metric.
314. KPI targets and alerts: Set KPI (key performance indicator) targets, with
     alerts.
315. Search member cohorts: Show cohorts of members who arrived from search.
316. Report template library: Offer a library of report templates.
317. Data freshness indicators: Show how fresh each piece of data is.
318. Report comments: Let people comment on reports.
319. KPI glossary: Explain each KPI in a glossary.

### Experimentation

320. Experiment registry: Record each experiment's hypothesis, metrics, duration
     and result.
321. Sample size calculator: Calculate the sample size and duration an
     experiment needs.
322. Sequential testing statistics: Run a statistics engine with sequential
     testing.
323. Holdout sections: Hold out sections to measure Sutradhar's overall effect.
324. Section feature flags: Switch features on or off per section with feature
     flags.
325. WordPress template experiments: Run server-side template experiments on
     WordPress. [WP] [Public]
326. Experiment idea generator: Generate experiment ideas.
327. Experiment learnings library: Keep a library of what experiments have
     taught.
328. Guardrail metrics: Use guardrail metrics that stop harmful tests.
329. Experiment conflict detector: Detect overlapping experiments that conflict.

### Page speed

330. Speed budgets per template: Set speed budgets for each template.
331. Plugin and add-on speed cost: Shows how much each plugin or add-on slows
     the site.
332. Font loading audit: Checks how the site's fonts load and how that affects
     page speed.
333. Full-page cache hit check: Checks whether pages are served from the
     full-page cache.
334. CDN detection and advice: Finds whether the site uses a CDN (content
     delivery network) and gives advice on it.
335. Speed regression alerts: Sends an alert when pages get slower after an
     update.
336. Throttled mobile processor tests: Tests page speed on a slowed-down mobile
     processor.
337. Image CDN suggestions: Suggests ways to serve images through an image CDN
     (content delivery network).

### Local presence, listings, and reviews

338. Google Business Profile health: Checks the health of the Google Business
     Profile. [Approval]
339. Google Business Profile post scheduling: Schedules posts on the Google
     Business Profile. [Approval]
340. Google review monitoring: Watches for new Google reviews. [Approval]
341. Review reply drafts: Drafts replies to reviews in the brand voice.
342. Review sentiment and themes: Shows the feeling and the main themes in
     reviews.
343. Review request campaigns: Runs campaigns that ask customers for reviews.
     [Own account]
344. Name, address and phone consistency: Checks that the name, address and
     phone number match across the site's own pages.
345. LocalBusiness structured data: Adds LocalBusiness structured data to the
     site. [Public]
346. Location pages manager: Manages the site's location pages. [WP]
347. Multi-location dashboard: Shows all locations on one dashboard.
348. Apple Business Connect sync: Keeps Apple Business Connect in step with the
     site's details. [Approval]
349. Yelp review monitoring: Watches for new Yelp reviews. [Paid]
350. Trustpilot review monitoring: Watches for new Trustpilot reviews. [Paid]
351. Facebook recommendations monitoring: Watches for new Facebook
     recommendations. [Approval]
352. Opening and holiday hours sync: Keeps opening hours and holiday hours in
     step across listings. [Approval]
353. Local landing page audit: Checks the site's local landing pages.
354. Local event posting: Posts local events to listings. [Approval]
355. Local citation checklist: Gives a checklist of local citations (places that
     list the business's name, address and phone).

### Social publishing and engagement

356. Bluesky publishing: Publishes posts to Bluesky.
357. Mastodon publishing: Publishes posts to Mastodon.
358. Threads publishing: Publishes posts to Threads. [Approval]
359. Facebook Pages publishing: Publishes posts to Facebook Pages. [Approval]
360. Instagram publishing: Publishes posts to Instagram. [Approval]
361. LinkedIn pages publishing: Publishes posts to LinkedIn pages. [Approval]
362. X publishing: Publishes posts to X. [Paid]
363. Pinterest publishing: Publishes posts to Pinterest. [Approval]
364. TikTok publishing: Publishes posts to TikTok. [Approval]
365. YouTube uploads and descriptions: Uploads videos to YouTube and writes
     their descriptions. [Own account]
366. Discord announcements: Posts announcements to Discord through webhooks.
367. Telegram channel posting: Posts to a Telegram channel.
368. Cross-network post scheduler: Schedules posts and shows them on one
     calendar across all networks.
369. Sharing of hub pages: Drafts social posts for approved hub pages and top
     threads. Each post waits for a person's approval through feature 378.
370. Best time to post: Shows the best time to post on each network.
371. Composer with previews: Writes posts with a preview for each network and
     checks each post's length.
372. AI caption drafts: Drafts captions with AI, each citing the thread it came
     from.
373. Hashtag suggestions: Suggests hashtags for posts.
374. Unified social inbox: Brings messages from all social networks into one
     inbox.
375. Social analytics: Shows how social posts perform.
376. Shared link click tracking: Counts clicks on links shared on social
     networks. [Public]
377. Evergreen recycling queue: Keeps a queue of lasting posts to share again
     over time. Each post waits for a person's approval through feature 378.
378. Social post approvals: Lets a person approve social posts before they go
     out.
379. Social profile audit: Checks the brand's social profiles across networks.
380. Campaign grouping: Groups social posts into campaigns.
381. Brand library attachments: Attaches files from the brand library to social
     posts.
382. YouTube channel analytics: Shows how the YouTube channel performs. [Own
     account]
383. Posting from feeds: Drafts social posts from any feed. Each post waits for
     a person's approval through feature 378.

### Brand monitoring and listening

384. Mention monitoring on free sources: Watches for brand mentions on free
     sources (Bluesky, Mastodon and RSS).
385. News monitoring through GDELT: Watches news for mentions through GDELT.
386. Google Alerts feeds: Reads mentions from Google Alerts feeds.
387. Hacker News monitoring: Watches Hacker News for mentions.
388. Podcast mentions: Finds mentions in podcasts through Podcast Index.
389. Wikipedia edit monitoring: Watches Wikipedia for edits that matter to the
     brand.
390. Reddit monitoring: Watches Reddit for mentions. [Approval]
391. YouTube mentions: Finds mentions on YouTube. [Own account]
392. Mention sentiment: Shows whether mentions are positive, negative or
     neutral.
393. Mention spike alerts: Sends an alert when mentions suddenly rise.
394. Influencer identification: Finds influential people who talk about the
     brand or its topics.
395. Topic and hashtag tracking: Tracks chosen topics and hashtags.
396. Share of voice in mentions: Shows the brand's share of mentions compared
     with others.
397. Review rating over time: Tracks the average rating in new reviews for each
     review site that features 340, 349 and 350 watch, each site on its own,
     compared with the period before until a year of reviews exists and then
     with the same period a year before; no average across sites and no
     blended score.
398. Crisis mode: Gives a step-by-step playbook to follow in a crisis.
399. Sponsor and partner mentions in forum: Finds mentions of sponsors and
     partners inside the forum.
400. Boolean query builder: Builds searches for mentions with AND, OR and NOT.
401. Exclusion filters: Leaves out mentions that match chosen filters.
402. Mention tagging and assignment: Tags mentions and gives them to a team
     member.
403. Reply suggestions: Suggests replies to mentions, but a person always sends
     them.
404. Multilingual mentions: Finds mentions in many languages and translates
     them.
405. Mention reports and exports: Makes reports of mentions and exports them.
406. Helpful-reply opportunities: Finds public discussions the forum already
     answers, so staff can join them honestly.

### Brand assets and guidelines

407. Brand hub: Keeps logos, colours, fonts and usage rules in one place, with
     versions.
408. Digital asset library: Stores digital assets with tags, search and
     versions.
409. Asset usage tracking: Tracks where each asset is used across the site.
410. Brand guideline documents: Makes brand guideline documents from design
     tokens (the stored colour, font and size values).
411. Off-brand colours and fonts: Finds pages that use colours or fonts outside
     the brand.
412. Brand voice guide: Gives a guide to the brand voice, with examples.
413. Branded social image templates: Gives branded templates for social images.
414. Asset request and approval workflow: Lets people request assets and have
     them approved.
415. Expiring licensed asset alerts: Sends an alert before a licensed asset's
     licence runs out.
416. Image rights registry: Records the rights for each image.
417. Brand kit export: Exports a brand kit for press and sponsors.
418. Duplicate asset detection: Finds duplicate assets in the library.
419. Format and size conversions: Converts assets to other formats and sizes.
420. Outdated logos in content: Finds old logos in posts and pages.
421. Sponsor co-branding rules: Sets rules for showing the brand alongside
     sponsors' brands.
422. Font licence tracker: Tracks the licence for each font.

### Surveys and research

423. Member survey pages: Shows survey pages to signed-in members. [Public]
424. Survey distribution: Sends surveys through alerts, email and links.
425. Survey analytics: Analyses survey answers with cross-tabs and confidence
     intervals.
426. Open-text answer themes: Groups open-text survey answers by theme.
427. Satisfaction and NPS tracking: Tracks satisfaction and NPS (Net Promoter
     Score) over time.
428. Concept tests: Tests names, logos and ideas with members.
429. Survey-based segments: Builds member segments from survey answers.
430. Survey templates: Gives ready-made survey templates.
431. Response quotas and weighting: Sets quotas for responses and weights
     answers.
432. Survey results in reports: Feeds survey results into reports.
433. XenForo poll analytics: Analyses the results of XenForo polls. [XF]
434. Question wording tests: Tests different wordings of a survey question.
435. Long-running tracker surveys: Runs the same survey over a long time to
     track change.

### Sponsors, CRM, email, and marketing automation

436. Sponsor and partner CRM: Keeps a CRM (customer relationship manager) for
     sponsors and partners.
437. Sponsorship pipeline and forecasts: Shows the sponsorship pipeline and
     forecasts its income.
438. Sponsor reports: Reports to sponsors on what they received.
439. Sponsor media kit generator: Makes a media kit for sponsors.
440. Rate card manager: Manages the rate card (the price list for sponsors).
441. Contract renewal reminders: Sends reminders before contracts need renewing.
442. Sponsored content approvals: Lets a person approve sponsored content.
443. Newsletter builder: Builds newsletters.
444. Newsletter content suggestions: Suggests content for newsletters.
445. Subscriber lists with consent: Keeps subscriber lists with a record of each
     person's consent.
446. Click analytics without pixels: Counts clicks in emails without using
     open-tracking pixels.
447. Lead forms: Adds forms that collect leads. [WP] [Public]
448. Welcome sequences: Sends a series of welcome emails, with consent.
449. Sponsor enquiry lead scoring: Scores sponsor enquiries by how promising
     they are.
450. Member lifecycle segments: Groups members by their stage of membership,
     shown only as totals.
451. Re-engagement campaigns: Runs campaigns to bring back quiet members, with
     consent.
452. Email preferences respected: Always respects members' existing email
     preferences.
453. Email authentication checks: Checks email authentication (SPF, DKIM and
     DMARC).
454. Partner and sponsor ticketing: Tracks requests from partners and sponsors
     as tickets.
455. CRM sync: Syncs with HubSpot or Salesforce. [Own account]
456. Campaign link builder: Builds campaign links with consistent naming.
457. Campaign return on investment: Shows the return on investment for each
     campaign.

### Workflow automation

458. Visual workflow builder: Builds workflows by arranging steps on screen.
459. Trigger library: Gives triggers for workflows from Sutradhar events,
     schedules and integrations.
460. Action library: Gives actions that workflows can take.
461. Scheduled workflows: Runs workflows on a schedule.
462. Incoming webhooks: Takes incoming webhooks through the owner's cloud
     mailbox, with no ports opened. [Own account]
463. Workflow templates: Gives ready-made workflow templates.
464. Run history with replay: Keeps a history of workflow runs that can be
     replayed step by step.
465. Retries, fallbacks and alerts: Retries failed steps, falls back to other
     steps and sends alerts.
466. Branching, loops and data mapping: Lets workflows branch, loop and map data
     between steps.
467. Script steps: Lets workflow steps run user scripts written in Rhai.
468. AI steps: Lets workflow steps use the local AI model.
469. Human approval steps: Adds steps that wait for a person's approval.
470. Workflow versions and rollback: Keeps versions of each workflow and can
     roll back to an earlier one.
471. Quotas per integration: Sets usage quotas for each integration.
472. Credential vault: Keeps credentials in the operating system's secure store.
473. Test runs with sample data: Runs workflows on sample data to test them.
474. Dry-run mode: Runs a workflow without making any real changes.
475. Workflow permissions: Controls who can use each workflow, with Cedar rules.
476. Workflow variables: Lets workflows store and use variables.
477. Workflow templates as files: Shares workflow templates as files.

### Integrations

478. Slack integration: Connects Sutradhar to Slack.
479. Discord bot: Connects Sutradhar to Discord through a bot.
480. Microsoft Teams integration: Connects Sutradhar to Microsoft Teams.
481. Matrix integration: Connects Sutradhar to Matrix.
482. Email integration: Connects Sutradhar to email through IMAP and SMTP.
483. Google Analytics 4 import: Imports data from Google Analytics 4.
484. Matomo import: Imports data from Matomo.
485. Plausible and Fathom import: Imports data from Plausible and Fathom.
486. Microsoft Clarity import: Imports insights from Microsoft Clarity.
487. Google Tag Manager audit: Checks the site's Google Tag Manager set-up.
488. Cloudflare analytics and purge: Imports Cloudflare analytics and purges the
     Cloudflare cache.
489. Google Merchant Center feeds: Makes product feeds for
     Google Merchant Center. [WP]
490. Google Drive report storage: Stores reports in Google Drive.
491. Dropbox integration: Connects Sutradhar to Dropbox.
492. OneDrive integration: Connects Sutradhar to OneDrive.
493. Google Calendar integration: Connects Sutradhar to Google Calendar.
494. Outlook Calendar integration: Connects Sutradhar to Outlook Calendar.
495. Notion integration: Connects Sutradhar to Notion.
496. Airtable integration: Connects Sutradhar to Airtable.
497. Trello integration: Connects Sutradhar to Trello.
498. Asana integration: Connects Sutradhar to Asana.
499. Linear integration: Connects Sutradhar to Linear.
500. Jira integration: Connects Sutradhar to Jira.
501. GitHub Issues integration: Connects Sutradhar to GitHub Issues.
502. Mailchimp integration: Connects Sutradhar to Mailchimp.
503. Brevo integration: Connects Sutradhar to Brevo.
504. Stripe metrics: Imports metrics from Stripe.
505. Patreon integration: Connects Sutradhar to Patreon.
506. Shopify integration: Connects Sutradhar to Shopify.
507. Ghost integration: Connects Sutradhar to Ghost.
508. Discourse integration: Connects Sutradhar to Discourse.
509. Figma assets: Brings in assets from Figma.
510. Canva integration: Connects Sutradhar to Canva. [Approval]
511. Unsplash images: Finds images on Unsplash.
512. Pexels images: Finds images on Pexels.
513. Wikimedia Commons images: Finds images on Wikimedia Commons.
514. DeepL translation: Translates text with DeepL. [Paid]
515. AI provider keys: Uses the owner's keys for AI providers (OpenAI,
     Anthropic, Google and Perplexity). [Paid]
516. SERP data providers: Gets search results data from providers such as
     DataForSEO. [Paid]
517. Bitly link shortening: Shortens links with Bitly.
518. Zapier and n8n bridges: Links to Zapier and n8n for owners who already use
     them.
519. WhatsApp Business notifications: Sends notifications through WhatsApp
     Business. [Paid]
520. Twilio SMS alerts: Sends alerts by SMS through Twilio. [Paid]
521. Microsoft Advertising keyword data: Imports keyword data from Microsoft
     Advertising.
522. RSS output feeds: Gives an RSS feed for any report.
523. Editorial calendar iCalendar export: Exports the editorial calendar as an
     iCalendar file.

### Audience analytics and memberships

524. Content analytics: Shows how content performs by author, section and tag.
525. Members compared with guests: Compares how members and guests use the site.
526. Real-time dashboard: Shows site activity as it happens.
527. Engagement per page: Shows each page's darshak engaged-visit rate,
     compared with pages of the same kind in its section; the visit count only
     hides pages with too few visits and is never shown. No blended score.
528. Registration and upgrade conversion: Shows how many visitors register or
     upgrade from each page.
529. Paywalled content structured data: Adds structured data for content behind
     a paywall. [Public]
530. Conversion funnels: Shows the steps from visit to registration, first post
     and upgrade.
531. Traffic share by source: Shows what share of traffic comes from each
     source.
532. Best publishing times and formats: Shows the best times and formats for
     publishing.
533. Sponsored content performance: Shows how sponsored content performs.
534. Analytics export: Exports analytics data.
535. Privacy dashboard: Shows a privacy dashboard and drafts a privacy page.
536. Attention time distribution: Shows how long visitors pay attention to
     pages.
537. Session depth by source: Shows how many pages visitors view in a visit, by
     source.
538. Registration wall advice: Advises which content to put behind registration.

### Team, governance, and enterprise

539. SEO task management: Manages SEO tasks.
540. Automatic tasks from issues: Makes tasks from found issues automatically.
541. Comments and mentions: Lets people comment and mention others on any report
     or issue.
542. Approval chains: Sets chains of approvals for high-risk changes.
543. Team activity feed: Shows what the team is doing.
544. Agency workspaces: Gives agencies workspaces for many client sites.
545. Task deadlines and service levels: Sets deadlines and service levels for
     tasks.
546. Team playbooks: Gives playbooks for migration, launch and recovery.
547. Decisions knowledge base: Keeps a team knowledge base of decisions.
548. Change freeze windows: Sets times when no changes may be made.
549. Impact-ranked backlog: Ranks the backlog by impact against effort.
550. Effort estimates: Estimates the effort for each task.
551. Training sandbox: Gives a safe practice space with demo data.
552. Team access and retention policies: Sets access and data retention policies
     for each team.
553. Feature usage analytics: Shows which features are used.
554. Quarterly planning board: Gives a board for planning each quarter.
555. Stakeholder digests: Sends short summaries to stakeholders.
556. Permission templates: Gives ready-made sets of permissions.

### International and multi-site

557. Portfolio dashboard: Shows all connected sites on one dashboard.
558. Cross-site duplicate content: Finds duplicate content across the owner's
     own sites.
559. Market selection advice: Advises which markets to enter.
560. Currency and unit checks: Checks currencies and units on international
     pages.
561. Translation memory and glossary: Keeps past translations and a glossary of
     terms.
562. Translation review workflow: Lets people review translations.
563. Search results by country: Shows how search results differ by country.
     [Paid]
564. Country targeting structure audit: Checks how the site's structure targets
     each country.
565. Per-market reports: Makes a report for each market.
566. Local-language query clustering: Groups search queries in local languages.
567. Regional content calendars: Keeps a content calendar for each region.
568. Cross-market link opportunities: Finds chances to link pages across
     markets.

### Video and podcast SEO

569. YouTube title and description optimisation: Improves YouTube titles and
     descriptions. [Own account]
570. Chapter suggestions: Suggests video chapters from transcripts.
571. Article drafts from videos: Drafts articles from video transcripts.
572. Video embedding suggestions: Suggests relevant threads in which to embed
     videos.
573. Article drafts from podcasts: Drafts articles from podcast show notes.
574. Podcast episode pages: Makes podcast episode pages with transcripts. [WP]
575. Video performance by topic: Shows how videos perform by topic.
576. YouTube comment insights: Shows insights from YouTube comments. [Own
     account]
577. Short-clip suggestions: Suggests short clips from long videos, given as
     timestamps.
578. Podcast directory checklist: Gives a checklist of podcast directories to be
     listed in.

### E-commerce SEO

579. Product page audit: Checks product pages.
580. Out-of-stock product handling: Handles out-of-stock and discontinued
     products.
581. Faceted navigation crawl traps: Finds crawl traps (endless page sets) made
     by faceted navigation (filters).
582. Category page content suggestions: Suggests content for category pages.
583. Duplicate manufacturer descriptions: Finds product descriptions copied from
     manufacturers.
584. Product structured data: Adds Product structured data. [Public]
585. Price and availability consistency: Checks that prices and availability
     match everywhere.
586. Review snippet eligibility: Checks whether pages can show review snippets
     in search. [Public]
587. Free merchant listings readiness: Checks readiness for free merchant
     listings.
588. Product image audit: Checks product images.
589. Guide-to-product internal links: Suggests internal links from guides to
     products.
590. Product Q&A drafts: Drafts product questions and answers from forum
     discussions.
591. Seasonal product demand: Shows how demand for products changes by season.
592. Search traffic sales attribution: Links sales to the search traffic that
     led to them.
593. Sales event landing pages: Makes landing pages for sales events.

### Data platform

594. SQL console: Runs SQL queries over the archive.
595. Scheduled data exports: Exports data on a schedule.
596. Data dictionary: Explains what each data field means.
597. Custom metrics: Lets the owner make custom metrics.
598. Report read access: Gives read access to reports through MCP and the app.
599. Historical data bulk import: Imports historical data in bulk, including
     other tools' exports.
600. Import data quality checks: Checks the quality of imported data.
601. Retention controls per dataset: Sets how long each dataset is kept.

### Alerts and notifications

602. Alert centre: Shows all alerts in one place.
603. Severity levels: Gives each alert a severity level.
604. Routing rules per alert type: Sends each type of alert to the right place.
605. Quiet hours: Holds alerts during quiet hours.
606. Digest frequency: Sets how often alert digests are sent.
607. Snoozing and acknowledging alerts: Lets people snooze alerts or mark them
     as seen.

### In-app SEO assistant

608. Ask questions about data: Answers questions about your data in plain
     English.
609. Daily AI briefing: Gives a daily briefing written by AI.
610. Explain this chart: Explains a chart in plain words.
611. Reports from plain-English requests: Makes reports from a request written
     in plain English.
612. AI task planner: Plans tasks with AI.
613. What-if questions: Answers what-if questions with the simulator (feature
     1).
614. Goals and preferences memory: Remembers the owner's goals and preferences.
615. Sources for every claim: Cites sources for every claim.
616. Offline local model: Works offline with the local model.
617. Proposed actions: Proposes actions, and the review page applies them.

### Distribution and syndication

618. Syndication rules: Sets syndication rules, with correct canonical tags for
     republished content.
619. Flipboard and feed reader readiness: Checks readiness for Flipboard and
     feed readers.
620. News publisher readiness checklist: Gives a checklist for being ready as a
     news publisher.
621. Content partnership tracker: Tracks content partnerships.
622. Republishing permissions log: Keeps a log of permissions to republish
     content.
623. Newsletter web archive pages: Makes web archive pages for past newsletters.
     [WP]

### Brand entity and knowledge panel

624. Knowledge panel readiness: Checks readiness for a knowledge panel
     (Organization data and sameAs).
625. Wikidata presence check: Checks the brand's presence on Wikidata, with
     guidance that follows Wikidata's rules (no self-promotion).
626. Brand name consistency: Checks that the brand name is used the same way
     across the web.
627. Founder and team pages: Makes founder and team pages. [WP]
628. Official profiles registry: Keeps a list of the brand's official profiles.

### Compliance and trust

629. Affiliate disclosure checker: Checks that affiliate links are disclosed.
630. Sponsored content labels: Puts visible labels on sponsored content.
631. Accessibility statement draft: Drafts an accessibility statement.
632. Cookie policy draft: Drafts a cookie policy.
633. Data request log: Keeps a log of data requests made under privacy laws.
634. Advertising-standards check: Finds claims in sponsored posts that need
     evidence.

### More technical checks

635. Non-thread duplicate pages: Finds duplicate and near-duplicate pages other
     than threads (articles, tags and archives).
636. Insecure outbound links: Finds outbound links to insecure sites.
637. Very large page structures: Finds pages with very large structures (DOM
     size).
638. Too many requests per page: Finds pages that make too many requests.
639. Main content in iframes: Finds main content placed inside iframes.
640. Links to blocked URLs: Finds internal links to URLs blocked by robots.txt.
641. Blocked URLs in sitemaps: Finds URLs blocked by robots.txt but listed in
     sitemaps.
642. Orphaned sitemap files: Finds sitemap files that nothing links to.
643. Text only in images: Finds important text shown only inside images.
644. Comment pagination duplicates: Finds duplicate pages made by WordPress
     comment pagination. [WP]
645. WordPress crawl shaping: Shapes how WordPress reply links, attachment
     pages, feeds and search pages are crawled. [WP]

### Platform extras

646. Views against search traffic: Finds threads with many views but little
     search traffic, and the reverse. [XF]
647. Search Console per section: Sets up Search Console properties for each
     forum section.
648. Thread prefixes in titles: Uses thread prefixes in titles where they help.
     [XF]
649. Category and tag planner: Plans the category and tag structure. [WP]
650. Permalink structure advice: Gives advice on the permalink structure. [WP]
651. Block editor move SEO check: Checks SEO when moving from the classic editor
     to blocks. [WP]
652. XenForo upgrade SEO checklist: Gives an SEO checklist for XenForo upgrades.
     [XF]
653. Theme switch SEO checklist: Gives an SEO checklist for switching themes.
     [WP]
654. Reverse image monitoring: Finds who reuses your images. [Paid]
655. Image context check: Checks image captions and the text around images.

### Community-led growth

656. Ambassador programme tracker: Tracks the ambassador programme.
657. Member invite links: Gives members invite links, with tracking. [Public]
658. Post reuse permission requests: Asks members for permission before reusing
     their posts in marketing.
659. Contributor spotlight drafts: Drafts posts that spotlight contributors.
660. Community events calendar: Shows a calendar of community events. [Public]
661. Expert Q&A planner: Plans question-and-answer sessions with an expert.
662. Newcomer first steps: Shows first steps to newcomers who arrive from
     search. [Public]
663. Sponsor Q&A planner: Plans question-and-answer sessions with a sponsor.

### Paid search alignment

664. Ads and organic overlap: Shows where ads and organic results overlap, so
     the owner can stop paying for queries the site already wins. [Own account]
665. Ad landing page checks: Runs SEO checks on the pages that ads link to.
666. Paid versus organic share: Shows the share of traffic from paid search
     compared with organic search.
667. Shopping ads versus free listings: Compares Shopping ads with free product
     listings. [WP]

### Other search engines

668. Yandex Webmaster: Connects the site to Yandex Webmaster. [Own account]
669. Naver Search Advisor: Connects the site to Naver Search Advisor. [Own
     account]

### More analytics, workflow, crawl and press features

670. Audience personas from forum data: Builds audience personas from forum
     data, in aggregated form only.
671. Competitor traffic estimates: Estimates competitors' traffic. [Paid]
672. Staff advocacy: Lets staff share approved posts.
673. Review benchmarking: Compares the site's reviews with those of competitors.
     [Approval]
674. Mention geography: Shows where mentions come from geographically.
675. Membership offer pages: Drafts membership offer pages. Every visitor and
     every search engine sees the same offer page. [Public]
676. Workflow data tables: Gives simple data tables for use in workflows.
677. Workflow input forms: Gives input forms for workflows inside the app.
678. Workflow diagrams: Draws workflow diagrams to document processes.
679. Error workflows: Runs an error workflow when another workflow fails.
680. Sub-workflows: Lets one workflow call another as a sub-workflow.
681. N-gram analysis: Counts how often phrases appear across the site.
682. Content similarity map: Maps how similar the site's pages are to each
     other.
683. Custom user agent crawling: Crawls with custom user agents to test how
     pages respond to different bots.
684. Store or club locator: Adds a page where visitors find stores or clubs.
     [WP] [Public]
685. Business Profile photos: Manages Google Business Profile photos. [Approval]
686. Search Console history archive: Keeps the full Search Console history
     beyond Google's 16 months.
687. Bing Webmaster history archive: Keeps the full Bing Webmaster history.
688. Anonymised query share: Shows the share of traffic from queries that Google
     does not name (anonymised queries).
689. Landing page and query matrix: Lets the owner explore which queries lead to
     which landing pages.
690. Indexing stats per sitemap: Shows indexing figures for each sitemap.
691. Newsroom builder: Builds a newsroom section on the site. [WP]
692. Press release drafting: Drafts press releases, with no paid wire
     distribution.
693. Media contact list: Keeps a list of media contacts, with their consent.
694. Press coverage archive: Keeps an archive of press coverage.
695. Competitor launch alerts: Alerts the owner when competitors launch
     something on social networks.
696. Social post performance prediction: Predicts how well a social post will
     perform.
697. Bluesky best threads feed: Offers a Bluesky custom feed of the community's
     best threads.
698. Mastodon verification links: Adds links that verify the site's Mastodon
     accounts. [Public]
699. Link-in-bio page: Builds a link-in-bio page. [WP] [Public]
700. Print campaign QR codes: Makes QR codes for print campaigns, with tracking.
     [Public]
701. Community sentiment by topic: Tracks how members feel about each forum
     topic, from our own analytics, compared with the month before; no blended
     index.
702. Product feedback mining: Finds feature requests and complaints about the
     owner's own products.
703. Competitor mentions in the forum: Finds where members mention competitors
     inside the forum.
704. Sponsor engagement early warning: Warns early when a sponsor's sections
     lose engagement.
705. Content licensing deal tracker: Tracks content licensing deals, for example
     with AI companies.
706. Brand safety check: Makes sure no sponsored placement sits next to a
     sensitive thread.
707. Email accessibility check: Checks emails and newsletters for accessibility.
708. Email list hygiene: Finds bounced and inactive addresses on the email list.
709. Best newsletter send time: Finds the best time to send newsletters.
710. Newsletter subject line tests: Tests different subject lines for
     newsletters.
711. Newsletter referral programme: Runs a referral programme for the
     newsletter.
712. Unified sponsor and partner timeline: Shows one timeline of each sponsor
     and partner across email, social networks and the CRM (customer records
     system).

## Range 3: features 713 to 1312

### The MCP content toolkit

713. Separate MCP content section: Gives the MCP server a separate content
     section with its own permissions, kept apart from admin tools.
714. Create draft tool: create_draft starts a draft of any supported content
     type, for XenForo or WordPress. The shared draft flow checks words
     and claims barred by site policy before approval. See
     [spec 0052](0052-hubs-summaries-drafts.md).
715. Update draft tool: update_draft changes a draft and keeps a version history
     of every change.
716. List and get drafts: list_drafts and get_draft find and read drafts, with
     filters.
717. Draft diff: Shows the changes between any two versions of a draft.
718. Content type registry resource: Lists every supported content type, with
     its fields, rules and examples.
719. House style guide resource: Gives the site's house style for spelling
     variant, capitalisation, numbers and formatting.
720. Source pack resource: Bundles the cited posts and pages for a topic.
721. Topic resource: Gathers everything the site knows about a topic in one
     place.
722. MCP prompts per content type: Offers MCP prompts (ready-made recipes) for
     each content type.
723. Elicitation before drafting: The assistant asks the owner about audience,
     length and tone before it drafts.
724. MCP sampling: Sutradhar asks the assistant's own model to write, so
     Sutradhar adds no AI costs.
725. Local model fallback: Uses the local model when an assistant does not
     support sampling.
726. Prompt autocomplete: Autocompletes topics, sections, authors and templates
     in prompts.
727. Progress notifications: Sends progress notifications for long content jobs.
728. Resource subscriptions: Tells the assistant when a source thread changes
     during drafting.
729. Content job queue: Queues content jobs for batch creation, with caps.
730. Content permissions per assistant: Sets which content types each assistant
     may create.
731. Daily creation limits: Sets a daily creation limit for each assistant.
732. Draft provenance record: Records, for each draft, the assistant, model,
     recipe and sources used.
733. Draft locking: Locks a draft so two assistants cannot edit it at once.
734. Undo for content actions: Lets the owner undo any MCP content action.
735. Dry-run mode: Previews what a content tool would create, without saving
     anything.
736. Structured output schema: Gives every content type a structured output
     schema and checks drafts against it before saving.
737. Content activity report: Reports the content activity of each assistant.
738. Exemplars resource: Gives the site's best examples of each content type.
739. Draft attachment upload: Uploads attachments for drafts into a staging
     area.
740. Draft export: Exports drafts as Markdown, HTML, BBCode or WordPress blocks.
741. Format conversion: Converts content between BBCode, WordPress blocks and
     Markdown.
742. Exact site preview: Previews a draft exactly as the site will display it.
743. Batch plan-then-apply: One approval of a plan creates many drafts.
744. Template filler: Creates one draft per row of a data table, for example a
     spec page per product. It is capped and every draft is reviewed.
745. Project working notes: Keeps working notes for each content project.
746. Content projects: Groups drafts, sources and decisions into content
     projects.
747. Capability discovery: Shows what content Sutradhar can create on this
     particular site.

### Research and sources

748. Best source posts: Finds the best source posts for a topic, ranked by
     accepted answers, reactions and expertise.
749. Claim extraction: Pulls claims out of threads, each linked to its exact
     post.
750. Disagreement map: Shows where members disagree on a question, and why.
751. Strongest member quotes: Finds the strongest member quotes on a topic, used
     only with the member's permission (feature 658).
752. Data from threads into tables: Pulls data from threads into tables, such as
     the specs, prices and settings members report.
753. Topic timeline: Shows how a topic evolved on the forum over time.
754. Experts to interview: Suggests experts to interview for a piece.
755. Research pack: Bundles the queries, questions and trends for a topic.
756. Gap finder: Finds questions searchers ask that a draft does not answer.
757. Related internal pages: Suggests related internal pages to cite while
     writing.
758. Wikipedia and Wikidata facts: Gives facts from Wikipedia and Wikidata, with
     citations.
759. Site activity statistics: Gives statistics from the site's own activity,
     with differential privacy (added noise that hides any one person) for small
     groups.
760. Relevant member photos: Finds relevant member photos, with their rights
     checked.
761. Relevant embedded videos: Finds relevant videos already embedded on the
     forum.
762. Earlier coverage check: Checks whether the site has covered the topic
     before.
763. Outdated source flags: Flags sources that are out of date.
764. Source credibility score: Gives each source a credibility score.
765. Strongest opposing views: Finds the strongest opposing views, for balance.
766. Glossary lookup: Looks up glossary entries for terms used in a draft.
767. Named entities for a topic: Lists the named entities for a topic, with
     their Wikidata IDs.
768. Site search questions: Shows the questions readers typed into the site's
     own search.
769. WordPress comment insights: Draws insights from WordPress comments to help
     update articles. [WP]
770. Saved research notes: Saves research notes for each project.
771. Cited post snapshots: Keeps snapshots of cited posts in case they are
     edited later.
772. Citation formatter: Formats citations using the display names members have
     chosen.
773. Research summary: Summarises research, with a confidence level for each
     point.
774. Verified fact list: Keeps a list of verified facts for each project.
775. Interview question generator: Writes interview questions built from forum
     questions.
776. Survey results in drafts: Pulls survey results into drafts.
777. Rival coverage on a topic: Shows what rivals have published on the topic,
     from competitor monitoring. It reads only the rivals the owner names.

### Planning and outlining

778. Series planner: Plans a multi-part series, with the order of parts and the
     links between them.
779. Outline generator: Writes an outline for each content type.
780. Angle generator: Suggests different takes on a topic, ranked by demand.
781. Audience definition: Defines the audience for each piece.
782. Search intent check: Checks an outline against search intent (what the
     searcher wants).
783. Section length planning: Plans how long each section should be.
784. Internal link plan: Plans internal links before writing starts.
785. Media plan: Lists the images, charts and diagrams a piece needs.
786. Pillar page planner: Plans one main page and its supporting pieces.
787. Existing page update plan: Plans an update to an existing page.
788. Interview plan: Plans an interview's format, questions and promotion.
789. Cadence planner: Plans how often recurring content appears.
790. Hook and introduction options: Offers options for the opening hook and
     introduction.
791. Call-to-action planner: Plans the call to action, such as join, reply or
     subscribe.
792. Effort and time estimate: Estimates the effort and time each piece needs.
793. Dependency planning: Shows which pieces must come first.
794. Review path: Shows who must check each piece.
795. Promotion plan: Plans how each piece is promoted.
796. Refresh schedule: Sets a refresh schedule for each piece, based on whether
     it is evergreen.
797. Format recommendation: Recommends a format, such as a thread, article,
     video or checklist.
798. Reading level target: Sets a target reading level for each audience.
799. Second-assistant outline review: A second assistant reviews the outline.
800. Sensitive topic risk assessment: Assesses risk for sensitive topics; legal,
     medical and financial topics need extra review.
801. Accessibility plan: Lists the alt text, transcripts and captions a piece
     needs.
802. Success measure: Sets a measure of success for each piece.

### Article types

803. How-to guides: Drafts how-to guides.
804. Step-by-step tutorials: Drafts step-by-step tutorials with screenshots.
805. Beginner's guides: Drafts guides for beginners.
806. Ultimate guides: Drafts ultimate guides (pillar articles).
807. Buyer's guides: Drafts guides for buyers.
808. Gift guides: Drafts gift guides.
809. List articles: Drafts list articles with real substance.
810. Best thread roundups: Drafts roundups of the best community threads.
811. Expert roundups: Drafts roundups of experts' views, with each expert's
     permission.
812. Q&A interviews: Drafts interviews in question-and-answer form, from AMAs
     (ask-me-anything sessions) or written interviews.
813. Member project case studies: Drafts case studies of member projects.
814. Build logs and write-ups: Drafts build logs and project write-ups from
     member threads.
815. Opinion pieces: Drafts opinion pieces with a staff byline.
816. News posts: Drafts news posts about the site's topic area.
817. Explainers: Drafts explainers of the "What is X?" kind.
818. Myth-busting articles: Drafts articles that bust myths.
819. Pros and cons articles: Drafts articles weighing pros and cons.
820. Alternatives articles: Drafts "Alternatives to X" articles.
821. Cost breakdown articles: Drafts articles that break down costs.
822. Maintenance schedules: Drafts maintenance schedules.
823. Problem-and-solution articles: Drafts articles that set out a problem and
     its solution.
824. Lessons learned and post-mortems: Drafts lessons-learned articles and
     post-mortems.
825. History articles: Drafts history articles.
826. Profiles: Drafts profiles of people, companies and makers.
827. Trend analysis: Drafts trend analysis.
828. Predictions: Drafts predictions, clearly labelled as such.
829. Community year in review: Drafts a review of the community's year.
830. Community awards results: Drafts the results of community awards.
831. Event recaps: Drafts recaps of events.
832. Announcement articles: Drafts announcement articles.
833. Site update articles: Drafts articles about updates to the site. [WP]
834. Curated resource lists: Drafts curated lists of resources.
835. Reading lists and beginner paths: Drafts reading lists and paths for
     beginners.
836. Tested method comparisons: Drafts comparisons of methods from tests, such
     as "3 ways to do X, tested".
837. Member experiment write-ups: Drafts write-ups of members' experiments.
838. Forum data stories: Drafts data stories from forum statistics.
839. Long-form features: Drafts long-form features.
840. Personal story collections: Drafts collections of personal stories, with
     consent.
841. Seasonal guides: Drafts seasonal guides.
842. Safety guides: Drafts safety guides; expert review is required.
843. Regulation and legal explainers: Drafts explainers on regulation and law;
     expert review is required.
844. Product launch coverage: Drafts coverage of other companies' product
     launches.
845. Staff-tested reviews: Drafts reviews of products staff have tested, with
     disclosure.
846. Weekly reader Q&A columns: Drafts weekly columns answering readers'
     questions.
847. Ask-the-expert articles: Drafts ask-the-expert articles.

### Forum-native content

848. Discussion-starter threads: Drafts staff threads that start discussions.
849. Recurring weekly threads: Drafts threads that recur each week.
850. Monthly showcase threads: Drafts monthly threads where members show their
     work.
851. Event megathreads: Drafts structured megathreads for big events.
852. Living wiki-style first posts: Drafts first posts that work like a wiki and
     that staff keep up to date.
853. Pinned FAQ threads: Drafts a pinned FAQ thread for each section.
854. Section welcome threads: Drafts a welcome thread for each section.
855. Balanced polls: Drafts polls with balanced options. [XF]
856. Debate threads: Drafts debate threads, with ground rules.
857. Monthly challenge threads: Drafts monthly challenge threads.
858. Contest threads: Drafts contest threads, with legally sound rules.
859. Giveaway announcements: Drafts giveaway announcements, with terms that meet
     the law.
860. AMA threads: Drafts threads that announce and host AMAs (ask-me-anything
     sessions).
861. Event threads: Drafts event threads with schedules and live updates.
862. Group buy threads: Drafts threads that organise group buys.
863. Marketplace guidance posts: Drafts guidance posts for the marketplace and
     classifieds.
864. Section introduction posts: Drafts posts that say what belongs in a
     section.
865. Rules reminder posts: Drafts posts that remind members of the rules.
866. Moderator announcements: Drafts announcements from moderators.
867. Staff answers to open questions: Drafts staff answers to unanswered
     questions, labelled as staff and cited.
868. Solved thread follow-ups: Drafts follow-up posts on solved threads, with
     newer information.
869. Merge explanations: Drafts posts in merged threads that explain the merge.
870. Split thread notices: Drafts notices that a thread has been split.
871. Locked thread explanations: Drafts explanations of why a thread is locked,
     with alternatives.
872. Fill-in thread templates: Drafts fill-in thread templates for members, for
     each section.
873. Prefix and tag proposals: Drafts proposals for prefixes and tags for new
     sections.
874. Resource Manager descriptions: Drafts descriptions for the Resource
     Manager. [XF]
875. Media Gallery album descriptions: Drafts descriptions of Media Gallery
     albums. [XF]
876. New section proposals: Drafts proposals for new sections, with descriptions
     and rules.
877. Trophy descriptions: Drafts descriptions of trophies. [XF]
878. User title ladder: Drafts the names and descriptions in the user title
     ladder. [XF]
879. Notice banner text: Drafts text for notice banners.
880. Featured content blurbs: Drafts short blurbs for featured content. [XF]
881. Weekly digest posts: Drafts weekly digest posts of the top threads.
882. Best answers of the month: Drafts "Best answers of the month" posts.
883. Newcomer question collections: Drafts collections of newcomers' questions,
     with answers.
884. Off-topic social threads: Drafts off-topic social threads.
885. Seasonal threads: Drafts seasonal threads.
886. Milestone celebration posts: Drafts posts that celebrate milestones.
887. Site feedback threads: Drafts threads that ask for feedback on the site.
888. Bug report templates: Drafts structured bug report templates for members.
889. Showcase prompts: Drafts showcase prompts, such as "Share your setup".
890. Collaborative list threads: Drafts collaborative list threads, curated by
     staff.
891. Thread series navigation posts: Drafts posts that link the parts of a
     thread series.
892. Long thread closing summaries: Drafts closing summaries for long-running
     threads.

### Learning content

893. Multi-lesson courses: Drafts courses of several lessons built from forum
     knowledge.
894. Lesson pages: Drafts lesson pages with objectives and exercises.
895. Quizzes: Drafts quizzes, with the answers in spoiler tags.
896. Flashcard sets: Drafts flashcard sets that can be exported.
897. Practice exercises: Drafts practice exercises with worked solutions.
898. Learning paths: Drafts learning paths from beginner to expert.
899. Prerequisite maps: Drafts maps of what to learn first.
900. Worked examples: Drafts worked examples.
901. Email courses: Drafts email courses that run over several days.
902. Self-assessment checklists: Drafts checklists learners use to assess
     themselves.
903. Common mistakes guides: Drafts guides to common mistakes.
904. Study guides: Drafts study guides for exams and certifications in the
     topic.
905. Hands-on project instructions: Drafts instructions for hands-on projects.
906. Mentor teaching notes: Drafts teaching notes for community mentors.
907. Live session lesson plans: Drafts lesson plans for live sessions.
908. Lesson recaps: Drafts recaps of lessons.
909. Knowledge checks in articles: Drafts knowledge checks placed inside
     articles.
910. Analogy explainers: Drafts explainers that teach concepts through
     analogies.
911. Visual explainers: Drafts explainers paired with diagrams.
912. Troubleshooting drills: Drafts practice scenarios for troubleshooting.
913. Case-based learning modules: Drafts learning modules built around cases.
914. Mentorship programme guide: Drafts a guide to the mentorship programme.
915. Learner Q&A digests: Drafts digests of questions and answers for learners.
916. Course FAQs: Drafts FAQs for courses.
917. Lesson discussion prompts: Drafts discussion prompts that link lessons to
     threads.
918. Discussion assignments: Drafts discussion assignments.
919. Assessment rubrics: Drafts rubrics for assessing work.
920. Course descriptions: Drafts descriptions of courses.
921. Recommended next lessons: Drafts recommendations for the next lesson.
922. Course update notes: Drafts notes on updates to courses.

### Reference content

923. Cheat sheets: Drafts cheat sheets.
924. Quick-reference PDF cards: Drafts quick-reference cards as PDFs.
925. Community spec sheets: Drafts spec sheets built from community data.
926. Compatibility charts: Drafts charts of what works with what.
927. Settings references: Drafts references for settings and configuration.
928. Part and model lookup tables: Drafts lookup tables of part numbers and
     models.
929. Acronym lists: Drafts lists of acronyms.
930. Timelines: Drafts timelines.
931. Recurring event calendars: Drafts calendars of recurring events in the
     topic area.
932. Curated directories: Drafts curated directories of clubs, shops and
     resources.
933. Static maps: Drafts static maps.
934. Conversion tables: Drafts tables of sizes, measurements and conversions.
935. Error code references: Drafts error code references built from forum
     knowledge.
936. Standard procedures: Drafts standard procedures.
937. Project tool and parts lists: Drafts lists of tools and parts for each
     project.
938. Price history tables: Drafts dated tables of price history, from forum
     mentions.
939. Version history references: Drafts references of version history.
940. Product decision matrices: Drafts decision matrices for choosing products.
941. Formula references: Drafts references of formulas and calculations.
942. Safety data summaries: Drafts summaries of safety data, with sources.
943. Standards and regulation summaries: Drafts summaries of standards and
     regulations, by country.
944. Member templates: Drafts templates for members, such as forms, logs and
     spreadsheets.
945. Printable checklists: Drafts checklists that can be printed.
946. Wiring and assembly diagrams: Drafts reference diagrams for wiring and
     assembly.
947. A–Z topic indexes: Drafts A–Z indexes of topics.
948. Start here pages: Drafts a "Start here" page for each section.
949. PDF reference bundles: Drafts bundles of references as downloadable PDFs.
950. Identification field guides: Drafts field guides for identifying things.
951. Member benchmark tables: Drafts benchmark tables from members' tests.
952. Quick-fixes lists: Drafts lists of common problems, each with a one-line
     fix.

### Community and policy pages

953. Community guidelines: Drafts the community guidelines.
954. Moderation policy: Drafts the moderation policy.
955. Terms of service draft: Drafts the terms of service, flagged for legal
     review.
956. Editorial policy: Drafts the editorial policy.
957. Corrections policy and page: Drafts the corrections policy and the
     corrections page.
958. AI use disclosure: Drafts a page on how the site uses AI.
959. Sponsorship and advertising policy: Drafts the policy on sponsorship and
     advertising.
960. About page: Drafts the About page.
961. Staff page: Drafts the staff page.
962. Contact page: Drafts the contact page.
963. Forum help pages: Drafts help pages on using the forum, such as posting,
     formatting and alerts.
964. New member guide: Drafts a guide for new members.
965. Moderator handbook: Drafts a handbook for moderators.
966. Staff onboarding guide: Drafts a guide for new staff.
967. Moderation transparency report: Drafts a report that shows how moderation
     works.
968. Community history page: Drafts a page on the community's history.
969. Appeals process page: Drafts a page on how to appeal moderation decisions.
970. Event code of conduct: Drafts a code of conduct for events.
971. Content licence page: Drafts a page on how members' content may be reused.
972. Copyright and takedown page: Drafts a page on copyright and takedown
     requests.
973. Security disclosure page: Drafts a page on how to report security problems.
974. Donation and support page: Drafts a page for donations and support.
975. Partner programme page: Drafts a page for the partner programme.
976. Trust and safety page: Drafts a trust and safety page.
977. Age and parental guidance page: Drafts a page on age limits and guidance
     for parents.
978. Site incident notice templates: Drafts templates for notices about site
     incidents.
979. Data export guide: Drafts a guide to exporting your data.
980. Account deletion guide: Drafts a guide to deleting your account.
981. Trophy and badge guide: Drafts a guide to trophies and badges.
982. Community roadmap page: Drafts a page on the community's roadmap.

### Visual content

983. Charts from site data: Drafts charts from site data.
984. Template infographics: Drafts infographics from templates.
985. Process diagrams: Drafts process diagrams in Mermaid.
986. Decision flowcharts: Drafts flowcharts for making decisions.
987. Timeline graphics: Drafts timeline graphics.
988. Comparison graphics: Drafts graphics that compare things.
989. Quote cards: Drafts quote cards, with permission.
990. Statistic cards: Drafts cards that show statistics.
991. Featured images from templates: Makes featured images for articles from
     templates.
992. Section banners: Makes banner images for site sections.
993. Annotated screenshots: Adds arrows and highlights to screenshots.
994. Personal data blurring: Blurs personal data in screenshots automatically.
995. Platform image crops: Crops images to each platform's sizes.
996. Image compression: Compresses images before upload.
997. Optional watermarking: Adds a watermark to images, if the owner chooses it.
998. Member photo collages: Makes collages of member photos, with their
     permission.
999. Before-and-after pairs: Makes before-and-after image pairs.
1000. Historical reference. See 933, Static maps.
1001. Brand icon sets: Makes icon sets from the brand library.
1002. Slide decks from articles: Turns articles into slide decks.
1003. Social media carousels: Makes image carousels for social media.
1004. Posters and flyers: Makes printable posters and flyers.
1005. Badge and trophy icons: Makes icons for badges and trophies.
1006. Chart text alternatives: Gives every chart a text alternative and a data
      table.
1007. Colour-blind palette check: Checks that colour palettes are safe for
      colour-blind readers.
1008. Diagrams from plain English: Makes diagrams from a plain-English
      description.
1009. Step-by-step illustrations: Makes step-by-step illustrations from a
      sequence of screenshots.
1010. Video thumbnail templates: Makes templates for video thumbnails.
1011. PDF covers: Makes covers for downloadable PDFs.
1012. Custom reaction images: Makes custom images for forum reactions. [XF]
1013. Alt text for visuals: Writes alt text and captions automatically for
      generated visuals.
1014. Visual style presets: Applies visual style presets from the brand hub.
1015. Self-updating charts: Charts update automatically when their data changes.
      Each update goes live only after a person approves it.
1016. Visual consistency check: Checks that the visuals in one piece look
      consistent.
1017. Visual source and rights record: Keeps a record of the source and rights
      for every visual.

### Audio and video production

1018. Video scripts: Writes scripts for videos.
1019. Short-form video scripts: Writes scripts for short-form videos.
1020. Podcast episode outlines: Writes outlines for podcast episodes.
1021. Podcast intros and outros: Writes the opening and closing words for
      podcast episodes.
1022. Timed voice-over scripts: Writes voice-over scripts with timing.
1023. Video storyboards: Makes storyboards for videos.
1024. Shot lists: Makes lists of the shots to film.
1025. B-roll from member media: Suggests B-roll (extra background footage) from
      member media, with permission.
1026. Subtitle files from transcripts: Makes subtitle files in SRT and VTT
      formats from transcripts.
1027. Transcript clean-up: Cleans up transcripts by removing filler words and
      adding speaker labels.
1028. Audio versions of articles: Makes audio versions of articles with a local
      text-to-speech model.
1029. Livestream run-of-show: Writes the running order for a livestream.
1030. Webinar scripts: Writes scripts for webinars.
1031. Video FAQ sections: Writes FAQ sections for videos.
1032. Guest bios: Writes short bios of guests for episodes.
1033. Sponsor read scripts: Writes scripts for sponsor messages read on air.
1034. Trailer scripts: Writes scripts for trailers.
1035. Social audio snippets: Makes audio snippets for social media, with
      captions.
1036. End-screen and call-to-action text: Writes end-screen text and calls to
      action for videos.
1037. Subtitle translation: Translates subtitles.
1038. Audio description scripts: Writes audio description scripts that describe
      what is on screen.
1039. Recording checklists: Makes checklists for recording sessions.
1040. Episode content warnings: Writes content warnings for sensitive episodes.
1041. Episode announcement threads: Writes forum threads that announce new
      episodes.
1042. Video timestamp index: Makes a timestamp index that links moments in a
      video to forum threads.

### Email and messaging copy

1043. Site announcement emails: Writes emails that announce site news.
1044. Event invitations: Writes invitations to events.
1045. Event reminders and follow-ups: Writes reminders before events and
      follow-ups after them.
1046. Sponsor pitch emails: Writes emails that pitch to possible sponsors.
1047. Partnership proposals: Writes proposals for partnerships.
1048. Contributor thank-you messages: Writes thank-you messages to contributors.
1049. Moderator warning templates: Writes respectful templates for moderator
      warnings.
1050. Suspension notices: Writes suspension notices that include how to appeal.
1051. Rule-change notifications: Writes notices that tell members about rule
      changes.
1052. Alert and notification copy: Writes the text for alerts and notifications.
1053. Staff conversation templates: Writes templates for staff private
      conversations. [XF]
1054. Canned replies: Writes ready-made replies to common questions.
1055. Contact form replies: Writes replies to messages sent through the contact
      form.
1056. Apology and incident emails: Writes apology emails and emails about
      incidents.
1057. Survey invitations: Writes invitations to take part in surveys.
1058. Contest winner announcements: Writes announcements of contest winners.
1059. Membership upgrade messages: Writes messages about membership upgrades and
      renewals.
1060. Donation thank-you notes: Writes thank-you notes for donations.
1061. Speaker invitations: Writes invitations to speakers.
1062. Member interview requests: Writes requests to interview members.
1063. Media enquiry replies: Writes replies to enquiries from the media.
1064. Volunteer recruitment messages: Writes messages that recruit moderators
      and volunteers.
1065. Moderator onboarding emails: Writes emails that welcome and guide new
      moderators.
1066. Inactive moderator check-ins: Writes check-in messages to moderators who
      have gone quiet.
1067. Sponsor report cover emails: Writes the cover emails sent with sponsor
      reports.

### Commerce and sponsor content

1068. Sponsored articles: Writes sponsored articles with the disclosure built
      in.
1069. Sponsored threads: Writes sponsored forum threads with the disclosure
      built in.
1070. Original product descriptions: Writes original product descriptions
      instead of copying the manufacturer's text. [WP]
1071. Product bundle descriptions: Writes descriptions of product bundles. [WP]
1072. Coupon and offer posts: Writes posts about coupons and offers, with their
      terms.
1073. Affiliate roundups: Writes affiliate roundups with disclosure.
1074. Deal alert posts: Writes posts that alert members to deals.
1075. Merchandise descriptions: Writes descriptions of community merchandise.
1076. Sponsor profile pages: Writes profile pages for sponsors.
1077. Sponsor case studies: Writes case studies about sponsors.
1078. Disclosure wording library: Keeps a library of disclosure wording.
1079. Search ad copy: Writes search ad copy for the owner's ad accounts.
1080. Social ad copy: Writes ad copy for social media.
1081. Banner ad variants: Writes different versions of the text for banner ads.
1082. Classified listing templates: Writes templates that members use for
      classified listings.
1083. Marketplace safety guides: Writes safety guides for buyers and sellers in
      the marketplace.
1084. Membership tier descriptions: Writes descriptions of each membership tier.
1085. Pricing page copy: Writes the text for pricing pages. [WP]
1086. Donation campaign pages: Writes pages for donation campaigns.
1087. Crowdfunding updates: Writes update posts for crowdfunding campaigns.
1088. Own product launch posts: Writes launch announcements for the owner's own
      products.
1089. Return and refund policies: Drafts return and refund policies. [WP]
1090. Shipping information pages: Writes pages with shipping information. [WP]
1091. Product care guides: Writes guides on caring for products.
1092. Warranty claim guides: Writes guides to making warranty claims, drawn from
      forum experience.

### Site copy and UX writing

1093. Clearer navigation labels: Writes clearer labels for site navigation.
1094. Button text variants: Writes different versions of button and
      call-to-action text.
1095. Empty-state messages: Writes messages shown when a page or list has
      nothing in it yet.
1096. 404 page copy: Writes the text for the page shown when a page is not
      found.
1097. Maintenance page copy: Writes the text for the page shown during
      maintenance.
1098. Registration page copy: Writes the text for the registration page.
1099. Login help text: Writes help text for logging in.
1100. Platform wording overrides: Writes overrides for XenForo phrases and
      WordPress interface wording.
1101. System email subject lines: Writes subject lines for the platform's system
      emails.
1102. Form labels and help: Writes labels and help text for forms.
1103. Search page hints: Writes hints for the search page.
1104. Footer text: Writes the text for the site footer.
1105. Site tagline options: Suggests options for the site's tagline.
1106. Section taglines: Writes taglines for site sections.
1107. Site wording consistency check: Checks that wording is consistent across
      the site.
1108. Inclusive site copy check: Checks site copy for inclusive language.
1109. Plain-English interface text: Rewrites interface text in plain English.
1110. Notification tone pass: Adjusts the tone of notification text.
1111. Cookie banner text: Writes the text for the cookie banner.
1112. Confirmation box copy: Writes the text for confirmation boxes.
1113. Success messages: Writes messages shown when an action succeeds.
1114. Loading and progress messages: Writes messages shown while something loads
      or runs.
1115. Seasonal wording variants: Writes seasonal versions of site wording.
1116. Translated wording packs: Makes packs of translated interface wording.
1117. Approved phrase library: Keeps a library of approved phrases.

### Editing operations

1118. Shorten to target length: Shortens a draft to a target length.
1119. Expand thin sections: Expands thin sections with detail that cites its
      sources.
1120. Simplify jargon: Simplifies jargon and adds links to the glossary.
1121. Restructure into steps: Restructures text into steps or lists.
1122. Add examples: Adds examples to abstract explanations.
1123. Add analogies: Adds analogies to explanations.
1124. Shift tone: Shifts the tone of a draft while staying within the brand
      voice.
1125. Remove repetition: Removes repeated content from a draft.
1126. Tighten introductions: Makes introductions shorter and sharper.
1127. Clear next steps: Ends a piece with clear next steps.
1128. Add transitions: Adds transitions between sections.
1129. Convert to Q&A: Converts a draft into question-and-answer format.
1130. Convert to a table: Converts content into a table.
1131. Merge two drafts: Merges two drafts into one.
1132. Split one draft: Splits one draft into two.
1133. Refresh outdated sections: Refreshes only the sections that are out of
      date.
1134. Source unsupported claims: Adds sources to claims that have none.
1135. Remove unsupportable claims: Removes claims that cannot be supported.
1136. Balance pass: Balances arguments that are one-sided.
1137. Inclusive article language: Checks articles for inclusive language and
      fixes them.
1138. Spoilers and content warnings: Inserts spoiler tags and content warnings.
1139. BBCode clean-up: Cleans up BBCode formatting.
1140. WordPress block clean-up: Cleans up the block structure of WordPress
      posts. [WP]
1141. Pull quotes: Chooses pull quotes from a piece.
1142. Embedded media captions: Writes captions for embedded media.
1143. Article tables of contents: Adds tables of contents to articles. [WP]
1144. Key takeaways boxes: Adds boxes that sum up the key takeaways.
1145. Tip and warning callouts: Adds callouts for tips and warnings.
1146. Footnote formatting: Formats footnotes.
1147. Consistent numbers and units: Makes numbers and units consistent.
1148. Consistent dates and times: Makes date and time formats consistent.
1149. Consistent names: Makes names of people and products consistent.
1150. Trim quotes faithfully: Trims quotes without changing their meaning.
1151. Line editing: Edits a draft sentence by sentence.
1152. Second-assistant copy-edit: A second assistant copy-edits the draft
      independently.

### Quality gates (a draft cannot be submitted for approval until it passes)

1153. Citation coverage: A draft cannot go for approval until every factual
      claim has a source.
1154. Source freshness: A draft cannot go for approval until its sources pass a
      freshness check.
1155. Unsupported claims: Statements not found in any source are flagged, and a
      draft cannot go for approval until this check passes.
1156. Recalculated numbers: A draft cannot go for approval until its numbers are
      recalculated from the source data and match.
1157. Exact quotes: A draft cannot go for approval until its quotes match their
      source posts exactly.
1158. Expert review for sensitive topics: A draft on a medical, legal or
      financial topic cannot go for approval until an expert reviews it.
1159. Dangerous instructions: A draft with dangerous instructions cannot go for
      approval until it has warnings and an expert review.
1160. Defamation risk: A draft cannot go for approval until claims about named
      people or companies pass a defamation check.
1161. Copyright: A draft cannot go for approval if it copies text beyond short
      quotes.
1162. Image rights: A draft cannot go for approval until the rights to its
      images are checked.
1163. Member privacy: A draft cannot go for approval if it holds a member's
      personal data without their consent.
1164. Sponsor disclosure: A sponsored or affiliate draft cannot go for approval
      until it carries a disclosure.
1165. AI-assistance labels: A draft cannot go for approval without an
      AI-assistance label where the owner's policy requires one.
1166. Scaled content: Bulk-made, near-identical pages are blocked and cannot go
      for approval.
1167. Added value: A draft cannot go for approval unless it adds something not
      already on the site.
1168. Search intent match: A draft cannot go for approval until it matches the
      search intent.
1169. Readability threshold: A draft cannot go for approval until it meets the
      readability threshold for its audience.
1170. Accessibility: A draft cannot go for approval until it has alt text,
      proper headings and readable visuals.
1171. No broken links: A draft cannot go for approval while it has broken links.
1172. No links to hidden pages: A draft cannot go for approval while it has
      internal links to noindexed or redirected pages.
1173. Brand voice threshold: A draft cannot go for approval until it meets the
      brand voice threshold.
1174. House style compliance: A draft cannot go for approval until it follows
      the house style.
1175. Spoilers and warnings in place: A draft cannot go for approval until its
      spoiler tags and content warnings are in place.
1176. Regional legal rules: A draft cannot go for approval until it meets
      regional legal rules, for example on giveaways, alcohol and gambling.
1177. Age-appropriateness check: A draft cannot go for approval until it is
      checked as suitable for the ages of its readers.
1178. Topic not already covered: A draft cannot go for approval if the site
      already covers its topic.
1179. Minimum substance: A draft cannot go for approval until it has enough
      substance.
1180. No keyword stuffing: A draft cannot go for approval if it is stuffed with
      keywords.
1181. Rich result preview: A draft cannot go for approval until a preview shows
      whether it is eligible for rich results in search.
1182. Translation quality: A translated draft cannot go for approval until its
      quality passes a back-translation check.
1183. Plain-English score: A help page draft cannot go for approval until it
      meets the plain-English score.
1184. No policy contradictions: A draft cannot go for approval if it contradicts
      the site's own policies.
1185. Second-assistant check: A draft cannot go for approval until a second
      assistant checks it independently.
1186. Human sign-off: A draft cannot go for approval until a person signs it off
      against a checklist.
1187. After publishing: A check confirms that the live page matches the approved
      draft.

### Variants and personalisation

1188. Beginner and expert versions: Makes beginner and expert versions of a
      piece.
1189. Length versions: Makes short, standard and long versions of a piece.
1190. Audience versions: Makes versions for new members, experts and sponsors.
1191. Channel versions: Makes versions for a forum thread, a WordPress article
      and an email.
1192. Skimmable versions: Makes skimmable versions with bullet points.
1193. Regional versions: Makes regional versions with UK or US spelling and
      units.
1194. Seasonal versions: Makes seasonal versions of a piece.
1195. Easy-read versions: Makes easy-read versions of a piece.
1196. Printable versions: Makes printable versions of a piece.
1197. Section announcement versions: Makes versions of announcements for each
      site section.
1198. SEO split test variants: Makes variants for the SEO split tests in feature
      21.
1199. Persona introductions: Writes introductions tailored to each persona (a
      typical reader type).
1200. Expert sidebars: Adds advanced sidebars with notes for experts.
1201. Glossary-rich versions: Makes versions with many glossary terms for
      learners.
1202. Variant manager: Links every variant to one master piece.
1203. Variants follow the master: Variants are updated when the master changes.
      Each update goes live only after a person approves it.
1204. Variant performance comparison: Compares how well each variant performs.
1205. Regional terms: Uses the right regional terms, such as "tyre" or "tire".
1206. Formal and informal versions: Makes formal and informal versions for
      different channels.
1207. Channel summaries: Writes one-line and one-paragraph summaries for
      different channels.

### Localisation

1208. Cultural adaptation: Adapts examples and references to the local culture.
1209. Local formats: Uses local units, currencies and date formats.
1210. Local regulation notes: Adds notes on local regulations.
1211. Local examples: Swaps in local examples.
1212. Right-to-left checks: Checks formatting for right-to-left languages.
1213. Localised images: Localises the text inside images.
1214. Local search terms: Uses local search terms in translations.
1215. Human editing levels: Sets levels of human editing for machine
      translations.
1216. Style guide per language: Keeps a style guide for each language.
1217. Localised calls to action: Writes calls to action for each locale.
1218. Local holidays and events: Adds references to local holidays and events.
1219. Name transliteration rules: Keeps rules for writing names in other
      scripts.
1220. Language coverage tracker: Tracks which pieces exist in which languages.
1221. Locale disclaimers: Adds disclaimers specific to each locale.
1222. Native-speaker reviewers: Assigns a native speaker to review each
      translation.

### Collaboration

1223. Assign drafts: Assigns drafts to staff writers and editors.
1224. Inline draft comments: Lets people add comments inside drafts.
1225. Suggestion mode: Offers a suggestion mode that tracks changes.
1226. Assistant handoff: Hands work from one assistant to another, for example
      Claude drafts and Codex checks.
1227. Commissioning briefs: Writes briefs that commission human writers.
1228. Guest author submissions: Takes in submissions from guest authors.
1229. Contributor agreement tracking: Tracks contributor agreements.
1230. Deadline reminders: Sends reminders about deadlines.
1231. Content board: Shows a board with columns for work yet to begin, work
      being written and finished work.
1232. Team draft templates: Keeps draft templates for each team.
1233. Reusable snippet library: Keeps a library of reusable snippets, such as
      paragraphs and disclaimers.
1234. Named piece owner: Gives each piece a named owner who maintains it.
1235. Review-by dates: Sets review-by dates for time-sensitive content.
1236. Owner review reminders: Reminds owners to review the content they own.
1237. Parallel section drafting: Several assistants draft sections at the same
      time, then the sections are merged.
1238. Writer feedback: Gives writers constructive feedback.
1239. Editorial notes history: Keeps a history of editorial notes.
1240. Bylines and acknowledgements: Adds bylines and acknowledgements for
      contributors.
1241. Member article requests: Lets members ask for an article.
1242. Member content votes: Lets members vote on requested content.

### Publishing

1243. Two-key publishing: Publishes a thread, post, article or page under a
      staff account through the two-key path, where an assistant drafts and a
      person approves.
1244. Scheduled publishing: Publishes content at a scheduled time.
1245. Multi-site drafts: Prepares a draft for each of the owner's sites, with
      correct canonical tags. Each draft is made in its own one-site MCP
      session, and a person approves and publishes each one on its own site.
1246. Save as WordPress draft: Saves content as a WordPress draft for a final
      human touch.
1247. One-step thread creation: Creates a thread with its prefix, tags and poll
      in one step. [XF]
1248. Full post creation: Creates a post with its categories, tags, featured
      image and excerpt. [WP]
1249. Changes approved as a diff: Changes to existing pages are approved as a
      diff that shows what changes.
1250. Unpublish or roll back: Proposes unpublishing or rolling back anything
      published through MCP. A person accepts it on the review page, and it
      can be undone.
1251. Embargoed release: Releases embargoed content at an exact time.
1252. Placement requests: Requests pinned or featured placement for content.
1253. Forum and blog cross-posts: Cross-posts announcements to the forum and the
      blog.
1254. Publishing checklists: Keeps a publishing checklist for each content type.
1255. After publishing: Requests internal links to the new page.
1256. After publishing: Requests that the piece be shared on social media.
1257. After publishing: Requests that the piece be included in the newsletter.
1258. Series navigation updates: Series navigation is updated automatically when
      a new part is published. Each update goes live only after a person
      approves it.
1259. Publishing permissions: Sets publishing permissions for each staff member
      and assistant.
1260. Publication log: Logs what was published, when, by which assistant and who
      approved it.
1261. Emergency unpublish switch: A switch the owner presses in the app
      unpublishes AI-assisted content. No assistant can press it.
1262. Media upload while publishing: Uploads media to the WordPress library or
      XenForo attachments while publishing.

### Measurement and learning

1263. Piece performance page: Shows a performance page for each piece.
1264. Assistant versus human content: Compares how assistant-made content
      performs against human-made content.
1265. Best recipes: Shows which recipes (saved content-making steps) produce the
      best content.
1266. Human edit amount: Measures how much people changed each AI draft, as a
      quality signal.
1267. Rejection reasons: Records the reasons drafts were rejected.
1268. Idea to publication time: Measures the time from idea to publication.
1269. Reader feedback: Gathers reader feedback from replies and comments.
1270. Winning patterns: Feeds winning patterns back into the recipes.
1271. Series performance: Measures how content series perform.
1272. AI cost per piece: Measures the AI cost of each piece.
1273. Member reactions: Measures members' reactions to staff content.
1274. New questions after publishing: Tracks new questions raised after a piece
      is published.
1275. Value estimate: Estimates the value of each piece.
1276. Visual engagement: Shows which visuals get engagement.
1277. Template copy performance: Measures how email and social copy performs, by
      template.
1278. Failing quality gates: Shows which quality gates fail most often.
1279. Writer and assistant scorecards: Keeps scorecards for writers and
      assistants.
1280. First 30 days search performance: Measures a new piece's search
      performance in its first 30 days.
1281. Portfolio balance: Shows the balance of the content portfolio by type,
      topic and freshness.
1282. Pieces to retire: Finds created pieces that should be retired.

### Safety and policy guardrails

1283. No member impersonation: Assistants never impersonate members. AI-assisted
      content appears only under labelled staff or system accounts.
1284. Historical reference. See 1283, No member impersonation.
1285. Historical reference. See 1165, AI-assistance labels.
1286. Prompt-injection defence: Forum text used as a source can never give the
      assistant instructions.
1287. Untrusted text as data: Every MCP resource marks untrusted text as data.
1288. Sensitive topic blocklist: The owner sets a blocklist of sensitive topics.
1289. Hate and harassment block: Blocks hate and harassment in drafts.
1290. Safe messaging on self-harm: Follows safe-messaging guidelines for
      self-harm topics.
1291. Political content rules: The owner sets rules for political and election
      content.
1292. Daily publishing limits: Sets daily limits on how much is published.
1293. AI content kill switch: A kill switch stops all AI content creation.
1294. Legal AI labels: Adds labels required by law for AI-generated content
      where they apply, for example under the EU AI Act. [Public]
1295. Children's data protection: Blocks content that collects or targets
      children's data.
1296. Trademark guidance: Gives guidance on using trademarks in content.
1297. Disclaimer library: Keeps a library of medical and financial disclaimers.
1298. AI content takedown: Gives a takedown process for complaints about
      AI-assisted content.
1299. Nightly red-team tests: Tests red-team prompts (attacks made on purpose)
      against the content tools every night.
1300. Secret and data filtering: Filters output for secrets and personal data.
1301. Approved models: Sets which models are approved for each content type.
1302. Assistant permission revocation: Revokes the permissions of any assistant.

### Staff and contributor access

1303. Assistant profile per writer: The owner sets up an assistant profile for
      each staff writer, with scoped content permissions. The owner starts
      every session, and sessions run only on the owner's PC (AGENTS.md
      section 5.5).
1304. Personal style memory: Keeps a personal style memory for each writer,
      within the brand voice.
1305. Writer dashboards: Shows each writer their drafts and deadlines in the
      site's admin pages.
1306. Personal drafts space: Gives each writer a personal space for drafts.
1307. Writer source packs: Makes source packs for each writer's sections.
1308. Trusted member writers: Gives trusted member writers draft-only access
      through site-account permissions, never the MCP server.
1309. Expiring guest access: Gives guest contributors site-account access that
      expires, through the platform's own permissions, with a draft-only role.
      A person grants it on the review page, and that Accept covers removing
      the access when it expires. It never opens the MCP server to them.
1310. Sponsor content intake: Takes in sponsor-supplied content as drafts, with
      disclosure enforced. Only the owner's own sessions do this.
1311. Agency writer access: The client connects its site to the agency edition
      with its own site account. Each agency writer works under their own
      draft-only account on that site. MCP sessions run only on the agency
      owner's PC, and each sees one site only.
1312. Writer offboarding: Removes a writer's assistant profile and site-account
      access, and reassigns their drafts.

## Range 4: features 1313 to 1357

These features came from a check of four well-known search and analytics
tools against this list. Each line says what the feature does, in our own
words. The analytics features work only on server logs, darshak's own
first-party counts, signed-in member activity and imports such as Search
Console and GA4. They need no third-party tracking, no cross-device
identity and no customer records.

### Crawler and audit additions

1313. Sitemap file draft: Drafts a clean XML sitemap file from the crawl,
      so staff can compare it with the one the platform makes.
1314. Mobile usability audit: Finds tap targets that are too small, text
      that is too small to read, and content wider than the screen.
1315. AMP validation: Where an AMP plugin is on, checks AMP pages for
      errors and for a correct link back to the main page. [WP]
1316. Structured data validation: Checks the structured data on every
      page against schema.org and Google's public rich result rules.
1317. Owner prompts per page: Runs a prompt the owner writes on each
      crawled page, with the local model or through MCP.
1318. Signed-in test crawl: Crawls member-only pages as a test account
      the owner makes, to check them. These pages are never link targets.
1319. Command-line crawls: Starts, exports and compares crawls from the
      command line, with no window open.
1320. Head HTML validity: Finds broken head sections, such as body tags
      inside the head, which can hide titles and canonical links.
1321. Page carbon estimate: Estimates the carbon cost of each page from
      its weight, with an open, cited method.
1322. Off-topic page finder: Finds pages whose meaning is far from the
      main topics of their section.
1323. Crawl settings as a file: Saves crawl settings as one file that can
      be shared and loaded again.
1324. Tunable issue rules: Lets the owner switch an issue type off, or
      change its threshold or its priority.
1325. Adaptive crawl pace: Slows the crawl down at once when the server
      answers slowly or with errors, and speeds up again with care.

### Monitoring

1326. Checks by page importance: Re-checks the most important pages most
      often, all through the day.
1327. Page change history: Keeps a dated history of each page's title,
      headings, text, canonical link and status.
1328. CDN log streams: Reads bot and visitor logs from the owner's CDN,
      such as Cloudflare, Fastly, Akamai or CloudFront. [Own account]
1329. Counter coverage check: Checks that darshak's first-party counter
      works on every page template.
1330. IndexNow pings: Tells search engines that use the open IndexNow
      protocol at once when a page is published, changed or removed.
      [Public]
1331. Browser page check: Shows a page's Sutradhar facts and issues while
      the owner looks at that page in their own browser.
1332. Journey stage labels: Labels pages and queries by the reader's
      stage: learning about a need, comparing choices, or ready to act.
      It adds the reader's stage to feature 71's intent labels, which it
      reuses, and is shown beside them.

### Bots, indexing and AI readiness

1333. Search funnel per section: Shows, for each section, the share of
      pages that are crawled, can be indexed, are visited by Googlebot
      and earn search visits.
1334. Bot crawl by device and file type: Splits bot visits into phone and
      desktop crawlers, and into pages and files, with the speed served
      to bots.
1335. Agent-readable site guide: Drafts an llms.txt file and a plain map
      of the best hubs for AI agents. Every visitor gets the same file.
      [Public]

### Analytics from logs and own counts

1336. Breakdown tables: Breaks any measure down by any field, such as
      section, then source, then device.
1337. Visit segment builder: Builds groups of page views, visits or
      signed-in members from rules the owner sets.
1338. Sequential segments: Finds visits that saw one page and then
      another, in that order.
1339. Segment comparison: Shows the biggest differences between two
      groups of visits.
1340. Segment overlap: Shows how much two or three groups of visits
      overlap.
1341. Attribution model comparison: Compares first, last, even and
      time-decay credit for registrations, within a window the owner
      sets.
1342. Contribution analysis: When a measure jumps, lists the sections,
      sources and pages that explain the change.
1343. Reader flow: Shows the paths readers take before and after a page.
1344. Custom fallout: Lets the owner pick any steps, such as search page,
      thread and sign-up, and shows how many readers drop at each step.
1345. Retention cohorts: Groups signed-in members by their first action
      and shows how many come back, how many stop, and how long they
      take to act, as totals only.
1346. Classification tables: Maps addresses, referrers and sources to the
      owner's own groups by rules, used in every report.
1347. Log processing rules: Cleans and renames incoming log fields by
      rules before any report uses them.
1348. Channel rules: Lets the owner define traffic channels, such as
      search, AI assistant, social and email.
1349. Filtered views: Makes filtered views of the data, each with its own
      visit timeout and fields.
1350. Live spreadsheet workbooks: Keeps a local spreadsheet file in an
      open format whose tables refresh from Sutradhar's data.
1351. Time-of-day views: Shows any measure by hour, weekday and weekend.
1352. Period comparison: Compares any report with an earlier period or
      the same period last year.
1353. Return frequency bands: Shows how often signed-in members come
      back, as totals only.
1354. Embedded media plays: Counts plays of embedded videos and audio
      with the site's own counter. [Public]
1355. Content trait analysis: Finds which page traits, such as steps,
      tables, images or length, go with engaged visits.

### Content workflow through MCP

1356. Target queries per draft: Gives each draft one main search query
      and up to four more, which the draft's checks then use.
1357. Draft auto-save: Saves draft edits as they happen, so no work is
      lost.

## Clean-room notes

These features come close to what another product does. Each gets our own
design, as set out here.

- 670 to 686: each names an idea only, and no product it comes close to;
  nothing from those products is copied: no code, formulas, issue lists,
  scores, screens, wording, data or trademarked names.
- 1314 Mobile usability audit: built from Google's public guidance, with
  our own thresholds set in the lab; no other tool's list of issues.
- 1316 Structured data validation: checked against schema.org and
  Google's public rich result documents, with our own Rust shape checks.
- 1321 Page carbon estimate: an openly licensed, published way of
  estimating, named on the screen; no other tool's rating scale.
- 1322 Off-topic page finder: our own measure, the distance of a page's
  embedding from the centre of its section, using our own embedder.
- 1324 Tunable issue rules: our own issue catalogue, with custom checks as
  Rhai user scripts (feature 185); no copied issue names or default
  thresholds.
- 1326 Checks by page importance: importance is our own score, made from
  the need inputs of ganit job 324 and the page's share of search; no
  other product's method.
- 1333 Search funnel per section: our own stage names and meanings, in
  plain English; no product's trademarked measure names.
- 1339 Segment comparison: our own test in ganit, standardised
  differences with control of false discoveries; the other method is
  not public and is not guessed.
- 1341 Attribution model comparison: only the textbook models (first,
  last, even, time decay); no data-driven model from any vendor.
- 1342 Contribution analysis: our own split of a change by field, built
  in ganit; no other product's method is used or guessed.
- 1343 Reader flow and 1344 Custom fallout: our own chart designs, drawn
  with spec 0017's visual layer; no copied screen layouts.
- 1350 Live spreadsheet workbooks: open spreadsheet formats with our own
  refresh; no add-in for another company's office programs.
- 1355 Content trait analysis: the traits come from shodhan's own text
  measures; no vendor's list of traits.
- 36 Blog briefs: the minimum source count comes from our own lab runs.
- 1330 IndexNow pings and 1335 Agent-readable site guide: open, public
  protocols, built from their published specifications.

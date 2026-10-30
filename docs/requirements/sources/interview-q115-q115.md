# Interview source: Q115

Part of the [Q091–Q120 transcript](interview-091-120.md). Status, provenance and
redaction notes are on that page.

## Q115

### Question

`````text
115. If the administrator has explicitly set Sad or Angry to neutral, should Zen G honour that choice or still count it as a downvote?
`````

### Options offered

`````text
1. Honour the administrator (Recommended)
Use the configured vote meaning; the learning engine can still observe the reaction type separately.
`````

`````text
2. Count it as a downvote
Give Sad and Angry a negative meaning inside Zen G even when XenForo scores them as neutral, and clearly explain that difference.
`````

### User answer

`````text
count it as a downvote also:

--------------

. Here are 250 signals for it to measure or check. Google doesn’t publish a list of ranking factors, so treat these as health checks and as inputs for zen-g features (linking, cannibalization, noindexing), not as a recipe. all should be built

1. Crawling and indexing (zen-g crawler)
★ HTTP status code of every URL
★ Redirect chains with more than one hop
Redirect loops
Temporary (302) redirects that should be permanent (301)
★ Error and no-permission pages that return 200 instead of 403/404 (soft 404s)
URLs blocked by robots.txt
robots.txt reachable, valid, and under Google’s 500 KiB limit
★ noindex in meta robots tags or X-Robots-Tag headers
Pages that are noindexed but also blocked in robots.txt, so Google never sees the noindex
★ Canonical tag on every indexable page
Canonical pointing to itself or to another URL
Canonical target returns 200 and is indexable
http/https and www/non-www variants that don’t redirect
Trailing-slash and uppercase URL variants
★ Sort and filter parameters (?order=, ?prefix_id=) creating duplicate listing pages
Tracking parameters (utm_*) in internal URLs
★ Thread pages 2+ (/page-2) indexable and self-canonical, not canonicalized to page 1
★ Click depth from the homepage
Utility URLs (log in, register, reply, report) linked from every page and crawled
Infinite crawl spaces (search results, member lists, “What’s new” feeds)
Valid, reciprocal hreflang tags (multilingual forums only)
Main content present in the raw HTML, not only after JavaScript runs
2. Server logs (web server access logs)
★ Googlebot requests per day
Crawl share by URL type (threads, forums, members, search, attachments)
★ Share of crawling spent on non-indexable URLs
Last Googlebot visit per thread
Time from thread creation to the first Googlebot visit
Fake Googlebots (failing the reverse-DNS or IP-range check)
Bingbot and other search crawler activity
AI crawler requests (GPTBot, OAI-SearchBot, ClaudeBot, PerplexityBot)
5xx errors served to bots
429 and 503 responses served to bots (rate limiting, maintenance)
Response time to bots
Sudden crawl spikes or drops
3. Sitemaps and discovery (crawler, Search Console, Bing)
★ Sitemap reachable and valid XML
Sitemap listed in robots.txt
Sitemap URL count vs. indexable URL count
★ Non-200, redirected, noindexed, or canonicalized URLs in the sitemap
lastmod matching the last real change (new reply or edit)
How long new threads take to appear in the sitemap (XenForo rebuilds it on a schedule)
Sitemap submitted in Search Console, and when Google last read it
Indexable URLs missing from the sitemap
IndexNow pings for new and updated threads (Bing and Yandex use them, Google doesn’t)
RSS feeds valid and discoverable
4. Search Console (Search Analytics and URL Inspection APIs)
★ Clicks per page
★ Impressions per page
Click-through rate per page
★ Average position per page
★ Queries per page
★ Cannibalization: queries where several of your pages rank
★ Striking-distance queries (positions 8–20)
Click-through rate far below normal for the position (weak title or snippet)
Pages losing impressions month over month (content decay)
Pages with zero impressions in 90 days
★ Index status per URL (indexed, crawled but not indexed, discovered but not indexed)
Google-chosen canonical that differs from the declared one
Rich-result status for forum and Q&A markup
Discover clicks and impressions
Image search clicks
Country split per page
Device split per page
Branded vs. non-branded query share
Question queries (how, why, what) per page
New queries that weren’t there last month (emerging topics)
★ Queries with impressions but no good matching page (content gaps)
Week-over-week click changes per forum
Last crawl date reported by URL Inspection
Bing Webmaster Tools clicks, impressions, and queries
5. Analytics and engagement (GA4 and XenForo)
★ Organic visits per landing thread
Engagement rate of organic visitors
Average engagement time per thread
★ Organic visitors who register
Organic visitors who post
Pages per organic visit
Very short organic visits (a rough sign the page didn’t answer the query)
Scroll depth on long threads
Returning-visitor rate
Clicks on internal links zen-g added
On-site search queries (copy XenForo’s search table regularly, because it gets pruned)
★ On-site searches with no results
Referral visits from AI assistants (ChatGPT, Perplexity, Claude, Gemini)
Traffic share by device
Thread views over time (XenForo’s view counter)
6. Thread content quality (XenForo database and zen-g)
★ Vague titles (“Help!”, “Question”, “Problem”)
Titles in all caps or with too much punctuation
Duplicate titles across threads
★ Title contains the main query people search for (from Search Console)
First-post length
Total thread length
★ Reply count
Number of unique participants
★ Accepted solution present (question threads)
Time to first reply
★ Unanswered threads
Total reaction score
Last activity date
Outdated content (old years or product versions in the title or first post)
Thread language
Readability
Spelling error rate
Share of the page made of quoted text
★ Low-value replies (“+1”, “thanks”, “bump”)
Spam posts still live in the thread
★ Near-duplicate threads
How well the thread fits its forum’s topic
Products, brands, and places mentioned
Replies from experts (staff, high-karma members, members with many accepted solutions)
Thread state (locked, moderated, deleted, redirect)
7. Titles and meta tags (crawler)
★ <title> present on every page
Title length (risk of being cut off in results)
★ Duplicate titles across pages
Page number in the titles of pages 2+
Site name in titles, in a consistent format
★ Meta description present
Meta description length
Duplicate meta descriptions
Meta description taken from the first post, not boilerplate
★ One H1 per page, matching the thread title
Logical heading structure (no skipped levels)
Open Graph tags (og:title, og:description, og:image, og:url)
og:image set per thread (first image attachment)
X/Twitter card tags
<html lang> matching the content language
8. Page layout and HTML (crawler)
★ Viewport meta tag (mobile-friendly)
How much navigation and widget HTML comes before the main content
Share of the page taken by sidebars, widgets, and signatures vs. posts
Identical widget content repeated on every page (latest posts, online users)
Visible breadcrumbs
Visible post dates
Visible author names linking to profiles
★ Intrusive pop-ups (cookie walls or sign-up prompts covering content for search visitors)
Ads above the main content
Favicon present and valid (it shows in search results)
Invalid elements in <head> that make Google stop reading it early, so later tags are missed
Empty or broken links (href="#", javascript:)
“Load more” buttons hiding content that has no paginated URL
Helpful 404 page with search and popular threads
★ Bots and guests see the same content (no cloaking)
9. Images and media (crawler and XenForo attachments)
★ Alt text on image attachments
Image width and height set (prevents layout shift)
Lazy loading for images below the fold
Modern image formats (WebP, AVIF)
Image file size
Descriptive image file names
Uploaded images vs. images loaded from other sites
Broken embedded images
Video embeds with thumbnails and titles
Large images (1,200 px wide or more) and max-image-preview:large for Discover
10. Structured data (crawler)
★ DiscussionForumPosting markup on thread pages
Required fields present: author name, publish date, and text or headline
Author URL pointing to the member profile
Like and reply counts (interactionStatistic)
Replies included as comment entries
★ QAPage markup with acceptedAnswer on solved question threads
Accurate dateModified
BreadcrumbList markup
WebSite and Organization markup on the homepage (site name and logo in results)
ProfilePage markup on member profiles
Images from posts included in the markup
Validation errors and warnings
Markup that matches visible content (counts, dates, names)
Deleted and moderated posts left out of the markup
No duplicate or conflicting markup from other add-ons
11. Internal links (zen-g link graph)
★ Internal links pointing to each thread
Internal links going out of each thread
★ Orphan threads (reachable only through forum listings, or not at all)
★ Links inside posts vs. navigation and listing links
Anchor text variety per target
Generic anchors (“here”, “this thread”)
★ Internal PageRank
Links to redirecting URLs (/posts/123/, /goto/post links in quotes, old slugs)
★ Broken internal links
Links to noindexed or robots-blocked pages
Links using non-canonical URL variants
nofollow on internal links
Hub threads linking to their topic cluster
Cluster threads linking back to their hub
Relevance of the related-threads widget
Internal links to your most important threads (by traffic or sign-ups)
Pagination links as real <a href> links
How many listing pages a crawler must go through to reach old threads
Tag pages working as topic hubs
Links per post (too many looks spammy)
zen-g suggestion accept rate per target
Threads linked only from old, rarely crawled pages
Links inside quotes and signatures (repeated and low value)
Links from high-traffic threads to related low-traffic ones (where links help most)
Internal links added and removed over time
12. Outbound links (crawler and XenForo posts)
External links per thread
★ Broken external links
Redirected external links
rel="ugc" or nofollow on member links, with exceptions for trusted domains
★ Links to malware or phishing sites (Google Safe Browsing API)
Affiliate links without rel="sponsored"
Links to expired domains that spammers may have bought
Bursts of links posted by new accounts
Links in signatures and profiles of low-activity accounts
Most-linked external domains
13. Backlinks and mentions (Ahrefs or similar)
Referring domains
New and lost referring domains
★ Backlinks per thread
Most-linked threads
Backlink anchor text
★ Backlinks pointing to 404 pages (redirect them)
Spammy backlink patterns (watch only; Google ignores most of them)
Domain Rating trend (an Ahrefs metric, not a Google signal)
Unlinked brand mentions
Backlinks from sites in your niche
Threads shared or discussed on Reddit and social media
Keywords competitors rank for that you don’t
14. Performance (CrUX API, PageSpeed Insights, real-user monitoring)
★ Largest Contentful Paint (LCP), from real visitors
★ Interaction to Next Paint (INP)
★ Cumulative Layout Shift (CLS)
Time to first byte
First Contentful Paint
Core Web Vitals pass rate per page type (thread, forum list, profile)
Mobile vs. desktop Core Web Vitals
Guest page cache hit rate (XenForo’s page cache)
CDN cache hit rate
Page weight
JavaScript size
Third-party scripts (ads, analytics, embeds)
Render-blocking CSS and JavaScript
Web font loading
Uptime and downtime minutes
15. Security and trust (crawler, Safe Browsing, XenForo)
★ HTTPS everywhere, with no mixed content
HSTS header
TLS certificate expiry
Safe Browsing status of your own site
★ Spam posts left up for more than an hour
Hacked content or injected links
Privacy policy, terms, and contact pages
About page naming who runs the forum
Time taken to act on reported posts
Ad density
`````

# 0145: Audit check and field catalogue

The audit suite uses versioned rule and field IDs in
[spec 0140](0140-unified-site-audit-suite.md) and the
[Site Health views](0143-audit-grid-findings-and-local-preview.md).
This catalogue defines 153 general audit checks and 253 URL-grain
fields, with 32 link fields and 54 detailed drawer fields. The
list of defined IDs gives no admission. Field and rule versions
remain independent of renderer, source and model availability.

## Capability trace

| Public behaviour or user request | Owning design and exact verification family |
|---|---|
| Scoped crawl, list, saved compare, segments | 0141 CR-01–04, cases CR-01-A to CR-04-A. 0143 AT-AG-01/02 covers report and comparable result. |
| Technical SEO issues and render facts | 0141 CR-05–09/CR-11, exact CR acceptance and each applicable CR/HT/LK/PF/AX row below. |
| Custom extract, post-crawl search, enrichment, Model Hub | 0141 CR-10/12/13 and their CR-A cases, 0143 AT-AG-03, 0144 A144-03. |
| Log acquisition, bot observations, crawl frequency | 0141 LG-01–03 and their LG-A cases, AUD-0145-LG01–03. |
| Robots, bot access, rewrites, raw/render and identity comparison | 0141 TB-01–05 and their TB-A cases, AUD-0145-AI family and sourced bot inventory below. |
| Mobile, hreflang/locale, structured data, XML and feed | 0141 TB-06–09 and their TB-A cases, AUD-0145-IN/MK families and per-predicate schema inventory. |
| Snippet, paid local search, AMP and updates | 0141 TB-10/TB-11 and their cases, 0143 AT-LP-01–05. |
| Lab metrics, structure, traces, frames, profiles and comparison | 0142 PLAB-01–09 and AUD-0145-PF family. Additional region and viewport evidence is supplied. |
| Category review, progress, explorer, filters, staging and export | 0143 AT-AG-01–04, 0141 CR-03-A, and field certificate and fixtures below. |
| Corpus segmentation, measures, semantic and historical mining | 0144 A144-01–10 and AUD-0145-CT family. Private exact-name display is separately scoped. |

## Built-in audit check catalogue

Each row is one designed rule. Its ID survives changes to wording and evidence
versions. The default severity is Error (E), Warning (W) or Notice (N). Check
outcomes are finding, clear, unknown, or not applicable. Unknown includes
partial, withdrawn, unqualified or incomparable inputs and carries the source
reason. A known inapplicable subject is excluded. A clear result needs
complete eligible facts for that predicate and scope. Absence requires a
complete eligible scope for the relevant subject. The rule ID and its operands
define the behaviour. No count here represents shipment or a health score.

Every check uses the full ID shown in its table. Each fact name is a private
typed evidence projection, or an exact shared `url.` or `link.` field. A
producer owns the cited capture, stage, admission and evidence type. It emits
`(site, subject, scope, generation, fact_key, typed value, coverage, receipt
reference)` with current rights. A specialist enum lists exactly the compared
values and any other admitted value belongs to its own producer schema. Flags
reflect an actual parsed observation. Counters are unsigned and report units
from the field name. In-range date, sample, rule and profile checks happen
before evaluation. Qualified budgets and vocabularies come from the owning
rules and are pinned on the input. No checker acquires evidence, broadens
permissions or treats a missing measurement as zero. An unavailable projection
or unqualified rule remains unknown with its exact reason.

Each row gives a synthetic trigger and clear pair separated by `→`, in the
same order as its named operands. The fixture generator makes three
independent cases named `FX-0145-<suffix>-ON`, `FX-0145-<suffix>-CLEAR` and
`FX-0145-<suffix>-UNKNOWN`. The suffix is the final four characters of the
full check ID. Choose the left or right values with all preconditions
qualified, then repeat the trigger with the listed predicate's first required
fact unknown and all other facts unchanged. Conjunctions and disjunctions also
test each deciding operand and applicable combinational boundaries. Array
examples are complete synthetic arrays. Set membership uses canonical key
equality. When a scope requires a graph, manifest or capture, the generator
creates a complete sealed local equivalent. A deterministic sibling reference
can stand in for its signed source. Replay keeps result, rule, scope and
source-receipt lineage stable under both admitted machine classes.

### Crawlability and index observations

Producer 0141 CR-01–08 and TB-01, with 0044/0082/0115 facts. The subjects
below are admitted own-site scopes. HTTP class applies only to an observed
response. Use plan purpose where stated. Respect robots holds independently of
all content findings.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-CR01 | E | `url.robots_state = disallowed` for a page selected in the crawl plan | disallowed → allowed |
| AUD-0145-CR02 | W | `crawl.robots_fetch = unreachable` on the in-scope declared host, with verified attempt outcome | unreachable → valid |
| AUD-0145-CR03 | W | `crawl.robots_parse = rejected` for an acquired robots document with complete grammar evidence | rejected → valid |
| AUD-0145-CR04 | E | `url.index_directive = noindex` on a page classified by the plan as intended for indexing | noindex → index |
| AUD-0145-CR05 | W | `crawl.header_index_directive != crawl.html_index_directive`, both explicit | noindex,index → index,index |
| AUD-0145-CR06 | W | `crawl.header_canonical_key != crawl.html_canonical_key`, both explicit valid addresses | A,B → A,A |
| AUD-0145-CR07 | W | `crawl.canonical_candidate_count > 1` in a single HTML declaration surface | 2 → 1 |
| AUD-0145-CR08 | W | `crawl.canonical_target_scope = off_site` for an explicit canonical | off_site → same_site |
| AUD-0145-CR09 | E | `crawl.canonical_target_indexability = excluded` with a current observed target | excluded → eligible |
| AUD-0145-CR10 | W | `crawl.canonical_route = chain` where a target declares another distinct key | chain → direct |
| AUD-0145-CR11 | W | `crawl.sitemap_membership = absent` for an intended, eligible index page and complete matching sitemap scope | absent → present |
| AUD-0145-CR12 | E | `crawl.sitemap_member_indexability = excluded` for a listed page whose response was observed | excluded → eligible |
| AUD-0145-CR13 | E | `crawl.sitemap_document_status = rejected` from the admitted XML parser for the selected sitemap | rejected → valid |
| AUD-0145-CR14 | E | `crawl.redirect_trace = loop` from complete observed hops | loop → terminal |
| AUD-0145-CR15 | W | `crawl.soft_404 = supported` for an observed 2xx page and the owning qualified soft-404 rule | supported → cleared |

### HTTPS and transport observations

Producer 0141 CR-05/CR-08/TB-04 and 0107 protected transport and 0115 broker
receipts. Subject is a stored admitted own-site transport or renderer
observation as specified. These rules perform no requests, emulation, exploit
reproduction or active security probes. Handshake diagnoses need an exact
typed owning reason. Trust evidence remains local.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-HT01 | E | `tls.document_scheme = http` for the observed terminal main document | http → https |
| AUD-0145-HT02 | E | `tls.redirect_transition = https_to_http` from the first recognised redirect decision | https_to_http → https_to_https |
| AUD-0145-HT03 | E | `tls.expiry_decision = expired` from the certificate validity check at receipt time | expired → valid |
| AUD-0145-HT04 | E | `tls.start_date_decision = not_yet_valid` from the certificate validity check at receipt time | not_yet_valid → valid |
| AUD-0145-HT05 | E | `tls.name_decision = mismatch` from verification against the exact connected name | mismatch → match |
| AUD-0145-HT06 | E | `tls.chain_decision = untrusted` from the chain check against the pinned trust profile | untrusted → trusted |
| AUD-0145-HT07 | E | `tls.protocol_admission = refused` where the negotiation receipt identifies a protocol outside the allowed profile | refused → allowed |
| AUD-0145-HT08 | W | `tls.hsts_header_count = 0` on a complete successful HTTPS header capture | 0 → 1 |
| AUD-0145-HT09 | W | `tls.hsts_parse = invalid` for a present HSTS response header | invalid → valid |
| AUD-0145-HT10 | W | `tls.hsts_max_age_seconds = 0` for a valid HSTS header | 0 → 86400 |
| AUD-0145-HT11 | E | `tls.mixed_script_refs > 0` for HTTP script references on an observed HTTPS page, regardless of later broker refusal | 1 → 0 |
| AUD-0145-HT12 | W | `tls.mixed_image_refs > 0` for HTTP image references on an observed HTTPS page | 1 → 0 |
| AUD-0145-HT13 | E | `tls.insecure_form_action_refs > 0` for observed HTTPS-page forms with resolved HTTP destinations | 1 → 0 |
| AUD-0145-HT14 | W | `tls.insecure_canonical_refs > 0` for HTTPS pages explicitly declaring HTTP canonicals | 1 → 0 |
| AUD-0145-HT15 | W | `tls.insecure_style_refs > 0` for HTTP stylesheet references on HTTPS pages | 1 → 0 |

### Performance and structure observations

Producer 0142 qualified Performance Lab, 0115 rendering facts and 0044/0141
capture references. A metric row needs its exact measured mode, unit, profile
and admitted upper goal; read each synthetic time value in the unit suffixed
on its field, and each ratio in basis points. Convert microseconds to
milliseconds as an exact rational with denominator 1,000 before a comparison
in milliseconds. Never infer a different measure from a unit conversion. The
local painted progression measure and scripted step-to-paint keep their 0142
names and cannot be interpreted as the provider Speed Index or field INP. Use
only actually qualified field or lab operands for the metric stated. A trace
rule needs eligible full coverage for the claimed field, never a lab inference
about field users.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-PF01 | W | `perf.largest_visible_paint_ms > perf.largest_visible_paint_goal_ms` | 301,300 → 299,300 |
| AUD-0145-PF02 | W | `perf.first_contentful_paint_ms > perf.first_contentful_paint_goal_ms` | 251,250 → 249,250 |
| AUD-0145-PF03 | W | `perf.visual_progress_time_ms > perf.visual_progress_time_goal_ms` | 301,300 → 299,300 |
| AUD-0145-PF04 | W | `perf.local_load_window_blocking_us > perf.local_load_window_goal_us` | 51,50 → 49,50 |
| AUD-0145-PF05 | W | `perf.scripted_step_to_paint_ms > perf.scripted_step_to_paint_goal_ms` for the admitted synthetic encounter only | 151,150 → 149,150 |
| AUD-0145-PF06 | W | `perf.cls_bp > perf.cls_goal_bp` | 1001,1000 → 999,1000 |
| AUD-0145-PF07 | W | `perf.ttfb_ms > perf.ttfb_goal_ms` | 101,100 → 99,100 |
| AUD-0145-PF08 | W | `perf.critical_blocking_stylesheet_count > 0` in the trace's measured critical path | 1 → 0 |
| AUD-0145-PF09 | W | `perf.critical_blocking_script_count > 0` in the trace's measured critical path | 1 → 0 |
| AUD-0145-PF10 | N | `perf.unused_style_bytes > perf.unused_style_goal_bytes` for coverage within the pinned encounter | 101,100 → 99,100 |
| AUD-0145-PF11 | N | `perf.unused_script_bytes > perf.unused_script_goal_bytes` for coverage within the pinned encounter | 101,100 → 99,100 |
| AUD-0145-PF12 | W | `perf.image_encoded_bytes > perf.image_size_goal_bytes` for an admitted display-size class and format | 101,100 → 99,100 |
| AUD-0145-PF13 | N | `perf.image_format_opportunity = measured` where the qualified local codec comparison gave a materially smaller equivalent rendition | measured → none |
| AUD-0145-PF14 | W | `perf.image_reserved_space = absent` for a laid-out image known to need it | absent → present |
| AUD-0145-PF15 | W | `perf.static_cache_policy = below_goal` for a cache-eligible static resource observed by the owning parser | below_goal → meets_goal |
| AUD-0145-PF16 | N | `perf.text_compression = absent` for an eligible complete text response | absent → present |
| AUD-0145-PF17 | N | `perf.style_critical_import_count > 0` for a measured blocking CSS import dependency | 1 → 0 |
| AUD-0145-PF18 | N | `perf.font_display_policy = blocking_qualified` on a qualified font use during paint | blocking_qualified → nonblocking |
| AUD-0145-PF19 | N | `perf.dom_node_count > perf.dom_node_goal` on the eligible stable DOM stage | 101,100 → 99,100 |
| AUD-0145-PF20 | N | `perf.connection_reuse = repeat_observed` where complete same-origin eligible traces prove missed reuse under the qualified client profile | repeat_observed → reused |

### Internal links and navigation

Producer 0141 CR-05/CR-07/CR-11, 0044, 0047 and 0082. Compute graph facts only
for complete matching graph slices. Link purpose is independently observed
from the eligible source and required where referenced.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-LK01 | E | `link.destination_http_status = 404` on an internal edge | 404 → 200 |
| AUD-0145-LK02 | E | `links.href_parse = invalid` on a declared internal navigation target | invalid → valid |
| AUD-0145-LK03 | E | `links.destination_status_family = server_error` on an internal edge with observed response | server_error → success |
| AUD-0145-LK04 | W | `link.redirect_hops > 1` for a documented finite internal chain | 2 → 1 |
| AUD-0145-LK05 | W | `links.destination_after_redirect = off_site` for an observed or explicitly resolved internal navigation hop | off_site → same_site |
| AUD-0145-LK06 | W | `links.fragment_resolution = absent` in the matching eligible target DOM | absent → present |
| AUD-0145-LK07 | W | `url.internal_inlinks = 0` on an intended index page in a closed graph (exclude selected roots) | 0 → 1 |
| AUD-0145-LK08 | N | `url.internal_outlinks = 0` on an eligible navigable content page | 0 → 1 |
| AUD-0145-LK09 | W | `url.click_depth > links.click_depth_goal` on a reachable eligible page with a complete rooted shortest-path view | 4,3 → 2,3 |
| AUD-0145-LK10 | N | `links.text_anchor_state = blank` on a visible text-only internal link | blank → meaningful |
| AUD-0145-LK11 | N | `links.anchor_specificity = generic_qualified` in an admitted language and purpose profile | generic_qualified → specific |
| AUD-0145-LK12 | W | `links.anchor_conflict = distinct_targets` for a qualified descriptive label with multiple eligible destinations in exact navigational scope | distinct_targets → same_target |
| AUD-0145-LK13 | N | `links.resolved_destination = self` on an ordinary internal navigation edge, excluding meaningful fragment links | self → different |
| AUD-0145-LK14 | W | `links.group_reachability = isolated` for a closed nonroot component with an eligible internal cycle/edge | isolated → root_reachable |
| AUD-0145-LK15 | W | `link.rel_nofollow = true` for the explicit internal navigation purpose where the plan expects traversable links | true → false |
| AUD-0145-LK16 | W | `links.route_only_seen_after_script = true` for a selected necessary navigation edge with complete matched raw and rendered evidence | true → false |
| AUD-0145-LK17 | N | `links.required_sitemap_route = missing_link` for an expected site-map navigation edge declared in the approved site plan and complete reviewed page | missing_link → present |
| AUD-0145-LK18 | N | `links.destination_canonical_relation = points_elsewhere` on an observed internal target with a qualified canonical | points_elsewhere → self |
| AUD-0145-LK19 | W | `links.breadcrumb_graph_consistency = parent_conflict` for eligible breadcrumb observations and a versioned curated parent graph | parent_conflict → consistent |
| AUD-0145-LK20 | N | `links.pagination_edge_reciprocity = absent` on a complete qualified previous/next pair, without a search-engine interpretation | absent → present |

### International

Producer 0047, 0044 and 0141 CR-07/TB-07. Existing eligible observations and
supplied locale captures only. Tokens follow the dated rule registry; absolute
keys use same owning canonicalisation. Matching language does not imply
territory.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-IN01 | W | `intl.language_subtag = invalid` for an explicit hreflang token | invalid → valid |
| AUD-0145-IN02 | W | `intl.region_subtag = invalid` for a recognised language plus an explicit territory | invalid → valid |
| AUD-0145-IN03 | W | `intl.script_subtag = invalid` for a recognised language plus an explicit script in an admitted syntax profile | invalid → valid |
| AUD-0145-IN04 | W | `intl.duplicate_locale_relation = conflicting_targets` inside one observed declaration set | conflicting_targets → unique |
| AUD-0145-IN05 | W | `intl.self_reference = absent` for a locally applicable, explicitly declared locale cluster with full evidence | absent → present |
| AUD-0145-IN06 | W | `intl.return_edge = absent` when a complete eligible counterpart exists | absent → present |
| AUD-0145-IN07 | E | `intl.target_http_acceptability = excluded` for an admitted observed alternate response | excluded → eligible |
| AUD-0145-IN08 | W | `intl.target_index_directive = noindex` on an observed alternate | noindex → index |
| AUD-0145-IN09 | W | `intl.target_canonical_alignment = points_elsewhere` for the observed eligible alternate | points_elsewhere → self |
| AUD-0145-IN10 | W | `intl.html_http_cluster_relation = conflicts` on the same locale and revision | conflicts → agrees |
| AUD-0145-IN11 | W | `intl.html_xml_cluster_relation = conflicts` on the same locale and compatible capture | conflicts → agrees |
| AUD-0145-IN12 | N | `intl.x_default_relation = conflicting_targets` across eligible sources for an explicit default | conflicting_targets → agrees |
| AUD-0145-IN13 | W | `intl.target_reference_shape = relative` when the qualified hreflang profile requires an absolute URL | relative → absolute |
| AUD-0145-IN14 | W | `intl.locale_target_language_match = contradicted` only for admitted comparable language observations | contradicted → matches |
| AUD-0145-IN15 | N | `intl.supplied_locale_main_response_relation = differs` for approved supplied same-page snapshots that differ on the declared locale axis | differs → equal |
| AUD-0145-IN16 | W | `intl.locale_cluster_cross_edge = conflicting_locale` where the closed graph proves a link into a different competing cluster | conflicting_locale → intended_cluster |

### Markup and structured data

Producer 0044 F-1316, 0115 RN-22 and 0141 TB-08/TB-11. Grammar and type
vocabularies are dated and local. Search guidance and visible-evidence rules
apply only to the relevant qualified type profile. A deficiency has no
implication for eligibility on a live search engine.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-MK01 | E | `markup.jsonld_syntax = invalid` for a present JSON-LD block | invalid → valid |
| AUD-0145-MK02 | W | `markup.context = unsupported` for explicit context in a syntax-valid block | unsupported → admitted |
| AUD-0145-MK03 | W | `markup.entity_type = unknown_term` in the pinned vocabulary | unknown_term → recognised |
| AUD-0145-MK04 | W | `markup.property_name = unknown_term` in the pinned vocabulary | unknown_term → recognised |
| AUD-0145-MK05 | W | `markup.property_domain = disallowed` under an exact type-property rule | disallowed → allowed |
| AUD-0145-MK06 | W | `markup.property_range = wrong_type` under an exact value-range rule | wrong_type → allowed |
| AUD-0145-MK07 | W | `markup.required_profile_property = absent` on the eligible requested type profile | absent → present |
| AUD-0145-MK08 | W | `markup.entity_id_relation = contradictory_identity` for two separate local definitions of the same admitted ID | contradictory_identity → consistent |
| AUD-0145-MK09 | W | `markup.local_reference_resolution = absent` for a required declared local entity on a closed graph | absent → present |
| AUD-0145-MK10 | W | `markup.marked_url_alignment = different_entity_target` against authorised canonical evidence | different_entity_target → aligned |
| AUD-0145-MK11 | W | `markup.visible_name_alignment = contradicts` on a matched visible name | contradicts → agrees |
| AUD-0145-MK12 | W | `markup.visible_currency_price_alignment = contradicts` on the same visible product, currency and price basis | contradicts → agrees |
| AUD-0145-MK13 | W | `markup.visible_availability_alignment = contradicts` only for a qualified 0115 availability fact and the same captured offer and variant | different admitted states → same state |
| AUD-0145-MK14 | W | `markup.visible_rating_alignment = contradicts` on the same visible rating scale and population | contradicts → agrees |
| AUD-0145-MK15 | W | `markup.visible_date_alignment = contradicts` on the same visible event and timezone | contradicts → agrees |
| AUD-0145-MK16 | W | `markup.visible_faq_alignment = contradicts` only for a qualified 0115 FAQ fact with the same question key, normalised question and answer role | different exact answer text → same text |
| AUD-0145-MK17 | E | `markup.amp_document_profile = invalid` for a selected complete AMP document and admitted offline validator profile | invalid → valid |
| AUD-0145-MK18 | W | `markup.amp_canonical_link_alignment = conflicts` on an AMP/canonical pair with eligible evidence | conflicts → agrees |

### AI bot policy and evidence

Producer 0141 TB-01/TB-02 with 0082 LV-R01/LV-R02/LV-R10, 0093 and 0135. Bot
tokens are used only in local policy simulation. Every disposition is
conditioned on the page owner's stated checklist purpose. Serving facts come
only from the declared Leviathan identity admitted for that route. Counts here
make no claim about vendor bot inventory.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-AI01 | W | `bot.discoverability_plan_match = search_denied` for a selected discovery purpose with a currently qualified robots simulation | search_denied → search_allowed |
| AUD-0145-AI02 | W | `bot.training_policy_match = train_allowed_unexpected` for the distinct owner plan to disallow training in the qualified simulation | train_allowed_unexpected → train_denied |
| AUD-0145-AI03 | W | `bot.user_assisted_policy_match = assistant_denied` for the distinct owner plan allowing that purpose | assistant_denied → assistant_allowed |
| AUD-0145-AI04 | W | `bot.source_header_reservation = reserves` for a qualified source response-header reservation conflicting with the selected discovery checklist | reserves → permits |
| AUD-0145-AI05 | W | `bot.source_meta_reservation = reserves` for a qualified HTML-meta reservation conflicting with the selected discovery checklist | reserves → permits |
| AUD-0145-AI06 | W | `bot.header_meta_policy_alignment = conflicting` on the same identity observation and distinct purpose-qualified explicit directives | conflicting → agrees |
| AUD-0145-AI07 | W | `bot.main_text_stage_visibility = render_only` for a complete comparable raw/script pair | render_only → both |
| AUD-0145-AI08 | N | `bot.primary_resource_policy = blocked` when the complete qualified resource inventory and rule prove a primary content dependency is denied | blocked → allowed |
| AUD-0145-AI09 | N | `bot.raw_main_text_state = no_main_content` from a complete admitted raw main-content projection on a page designated editorial | no_main_content → present |
| AUD-0145-AI10 | N | `bot.disclosure_policy_conflict = simulation_allow_observed_reservation` when policy simulation allows a purpose but its separate eligible response-level evidence reserves it | simulation_allow_observed_reservation → consistent |
| AUD-0145-AI11 | W | `bot.helpful_path_access = challenge` for an observed honest-identity main endpoint and expected guest access | challenge → served |
| AUD-0145-AI12 | N | `bot.citation_inventory_match = no_matching_owned_url` for an owner-supplied eligible AI answer file expected to cite owned pages, with all extracted citations covered | no_matching_owned_url → matching_owned_url |

### Accessibility observations

Producer 0141 CR-09 and 0115 owning DOM, style, layout and accessibility
projections. Only rules directly measurable at the required stage can be
clear. This catalogue does not certify accessibility conformance.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-AX01 | E | `access.informative_image_alt = absent` on an image classified informative under the qualified rule | absent → provided |
| AUD-0145-AX02 | W | `access.decorative_image_accessible_exposure = unexpected` on an image explicitly classified decorative | unexpected → suppressed |
| AUD-0145-AX03 | E | `access.control_accessible_name = absent` on a qualified interactive form field | absent → present |
| AUD-0145-AX04 | W | `access.button_accessible_name = absent` on an actionable button | absent → present |
| AUD-0145-AX05 | W | `access.navigation_link_accessible_name = absent` on an eligible nontext navigation link | absent → present |
| AUD-0145-AX06 | W | `access.aria_role_token = invalid` on an explicit role declaration | invalid → valid |
| AUD-0145-AX07 | W | `access.required_role_property = absent` on an admitted valid role that requires that property | absent → present |
| AUD-0145-AX08 | W | `access.name_reference_resolution = absent` on a complete tree with an explicit accessible-name reference | absent → present |
| AUD-0145-AX09 | N | `access.heading_level_transition = skips` in an eligible section traversal | skips → consecutive |
| AUD-0145-AX10 | W | `access.main_landmark_count = 0` where the qualified page profile requires a main landmark | 0 → 1 |
| AUD-0145-AX11 | W | `access.repeated_landmark_labels = ambiguous` for full same-role named navigation scope | ambiguous → distinct |
| AUD-0145-AX12 | W | `access.document_language_declaration = absent` on a complete document root | absent → present |
| AUD-0145-AX13 | E | `access.text_contrast = below_goal` under a qualified rendered text context and threshold class | below_goal → meets_goal |
| AUD-0145-AX14 | W | `access.ui_indicator_contrast = below_goal` for a relevant visible UI boundary with admitted paint evidence | below_goal → meets_goal |
| AUD-0145-AX15 | W | `access.target_geometry = below_goal` for a non-exempt measurable input target with applicable spacing | below_goal → meets_goal |
| AUD-0145-AX16 | W | `access.horizontal_reflow = unintended_overflow` for a qualified reflow viewport and non-exempt page content | unintended_overflow → fits |
| AUD-0145-AX17 | E | `access.focusable_inside_hidden = true` in the qualified accessibility structure | true → false |
| AUD-0145-AX18 | W | `access.table_header_relation = absent` on a complete data-table structure with qualified header applicability | absent → mapped |

### Content and editorial observations

Producer 0044, 0041, 0121, 0141 CR-06/09 and 0144. Use approved local content
and the exact language, cohort, model, protected revision and coverage.
Editorial candidates remain advisory within their admitted purpose.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-CT01 | W | `text.title_present = false` on a complete selected document | false → true |
| AUD-0145-CT02 | W | `text.description_present = false` on a complete selected document | false → true |
| AUD-0145-CT03 | N | `text.title_pixel_fit = over` in a qualified font and viewport preview | over → within |
| AUD-0145-CT04 | N | `text.description_pixel_fit = over` in a qualified font and viewport preview | over → within |
| AUD-0145-CT05 | W | `text.same_title_distinct_pages = true` for the pinned duplicate-title scope | true → false |
| AUD-0145-CT06 | W | `text.same_description_distinct_pages = true` for the pinned duplicate-description scope | true → false |
| AUD-0145-CT07 | W | `text.primary_heading_count = 0` on a full document with a declared primary-heading requirement | 0 → 1 |
| AUD-0145-CT08 | W | `text.primary_heading_count > 1` on a full document whose profile calls for one primary heading | 2 → 1 |
| AUD-0145-CT09 | N | `text.exact_corpus_copy = true` for two distinct eligible canonical documents and compatible normalised main text | true → false |
| AUD-0145-CT10 | N | `text.disjoint_near_copy = true` for a qualified same-scope nonidentical pair at or above the owning similarity cutoff | cutoff-equal confirmed pair → below-cutoff pair |
| AUD-0145-CT11 | N | `text.spelling_candidates > 0` from the admitted language, dictionary, exclusions and revision | 1 → 0 |
| AUD-0145-CT12 | N | `text.grammar_candidates > 0` from the separately admitted grammar rule set and language | 1 → 0 |
| AUD-0145-CT13 | N | `text.readability_band = outside_plan` under the exact applicable profile and declared reading audience | outside_plan → within_plan |
| AUD-0145-CT14 | N | `text.boilerplate_share = above_goal` under the exact admitted segmentation and denominator scope | above_goal → within_goal |
| AUD-0145-CT15 | N | `text.source_freshness_need = qualified_overdue` only for an explicitly dated review expectation from the owning plan | qualified_overdue → within_window |
| AUD-0145-CT16 | N | `text.observed_locale_mismatch = high_support_conflict` when eligible declared and locally detected text language disagree under the qualified model and profile | high_support_conflict → agrees |

The preceding nine families have 150 designed behaviours. A rule can remain
eligible for design verification
with synthetic inputs while its production route reports unavailable.

### Verified crawl activity

Producer 0141 LG-03 through qualified 0034/0044 aggregate evidence.
Subject is an exact comparable own-site page or segment and a closed,
eligible date window. Review controls set the absence and error
intent. Respect disclosure and hold all values with unproved coverage.

| ID | Sev | Exact predicate and required field | Synthetic trigger → clear |
|---|---|---|---|
| AUD-0145-LG01 | W | log.verified_page_5xx_count > 0 in the closed measured population | 1 → 0 |
| AUD-0145-LG02 | N | log.verified_page_hits = 0 on a selected eligible high-priority page and one complete verified source window | 0 → 1 |
| AUD-0145-LG03 | N | log.verified_page_hits > 0 and graph.internal_incoming = 0 in an eligible complete link inventory of discovered own pages | 1,0 → 1,1 |

## Typed field catalogue

The 0143 URL Explorer, Link Explorer and drawer use a signed catalogue
of typed, independently attributable facts. Different labels, display
units, filters, render profiles, sources or variants of one field do
not create a new ID. A defined field still needs producer admission.

The URL subject is one site, capture, canonical page and stage. The
link subject adds a source occurrence ordinal and relation. The drawer
selects one scoped page, request, phase, frame, fact, change, or corpus
result at a time. There is no wide join from attempts, frames or term
rows back into URL rows. When multiple eligible inputs remain, require
the exact selected input or return unknown. Attach every value to the
0143 fact envelope, current read grant, subject, immutable source and
producer version, coverage and erasure generation. A derived count
also binds the exact input digest and eligible population. Withhold
opaque custody pointers. A safe action reference is local and inert.

| Type | Value and order |
| --- | --- |
| count, bytes | Non-negative integer, ordered numerically. Counts name their population. Transfer and decoded bytes are separate. |
| chars | Unicode scalar count under the admitted normalisation. Withhold when the source proves only another unit. |
| pos | Non-negative position in the pinned source coordinate system, ordered within that system. |
| ratio, measure | Exact rational with admitted nonzero denominator, or numeric with explicit metric version, unit and precision. Order only comparable evidence. A signed measure may be negative. |
| us, UTC | Integer microseconds with the exact clock or measured method, or UTC instant at original precision. An unknown supplied origin or ambiguous date stays unknown. |
| enum, bool | Closed, pinned producer enum order. Boolean false precedes true. |
| ID, label | A scoped inert reference, or admitted short scrubbed text. No path, raw payload or secret authority. |

A proved scalar has a value and no nullable cause. Refuse foreign,
revoked or erased rows before filtering, paging or counting. For an
eligible row use the 0143 safe reason for the first applicable cause:
withheld by policy, source missing, incompatible profile or stage,
capability unavailable, stale, partial, not applicable, or below the
source disclosure floor. Withheld field reasons omit source details.
A complete eligible observation can prove zero or false. Partial
counts can retain a safe observed lower bound in the envelope. They
cannot masquerade as complete values, including in a sort or export.

### Exact scalar fields

Prefix each comma-separated token with the printed prefix. P is 0044
or 0141 page/capture receipt. E is the 0044 or 0141 edge occurrence. S
is 0047 structure evidence. R is the admitted 0115 render fact, L is
0142 capture or trace, T is the 0144 corpus result seal and D is the
0116 sealed same-site diff. Nil follows the common causes and the
source conditions in each row. Mixed sources or revisions never
substitute without an admitted exact join.

| Prefix, producer and type | Exact leaf IDs | Source condition |
| --- | --- | --- |
| url. P ID | page_ref, capture_ref, native_object_ref, redirect_target_ref, canonical_target_ref | Safe bound pointers; last three require exact object proof. |
| url. P enum | stage, source_kind, fetch_outcome, coverage_state, http_status, media_kind, robots_state, index_directive, canonical_state, redirect_state | Observed HTTP status ordered by integer code. Denial and timeout are outcomes. Robots access, directives and observed index state are separate. |
| url. P UTC | captured_at, response_observed_at, robots_checked_at | Each named receipt supplies its own date. |
| url. P us | response_elapsed | Complete observed elapsed response, 0044 F-0142. Keep its timing method. |
| url. P count | title_elements, description_elements, canonical_elements, redirect_hops, observed_headings, sitemap_memberships, internal_inlinks, internal_outlinks, external_outlinks | Complete named head, hop, document, membership or occurrence population. No replay duplicates. |
| url. P chars | title_length, description_length | Exact admitted head slot and unit. Ambiguous duplicates give no single length. |
| url. P label | display_address, declared_language, declared_encoding | Scrubbed guest-safe address or exact eligible declaration. |
| url. P bool | header_noindex, meta_noindex, header_nofollow, meta_nofollow, directives_conflict, has_raw_canonical, has_viewport_declaration, viewport_declaration_valid | Complete source-specific declaration and pinned rule. |
| url. S ID | layout_ref, section_ref | Exact assigned group or site section. |
| url. S enum | page_kind, navigation_only_inlink_state, sitemap_robots_state, whole_html_cache_state, image_edge_state, main_frame_state, redirect_policy_state, duplicate_state | Eligible 0047 scoped finding, with the underlying proof still current. |
| url. S count | click_depth, layout_peer_pages, confirmed_duplicate_peers, required_alternates, observed_alternates, broken_referenced_images, insecure_external_links | Finite depth from the admitted start and counts only for complete named populations. |
| url. 0116 measure | internal_link_flow | LS-15 admitted local graph measure; no search ranking claim. |
| url. R count | dom_elements, open_shadow_roots, closed_shadow_roots, frame_elements, script_added_links, script_removed_links, unsupported_visible_markup_facts, accessibility_findings, blocked_resources | Actual admitted tree or diff population. Visited roots and proved closed markers only. |
| url. R enum | soft_404_confidence, content_visibility_state, decoder_outcome, raw_render_compare_state | Keep renderer confidence and coverage apart from response status. |
| url. R bool | script_changed_title, script_changed_description, script_changed_canonical, script_changed_headings, script_changed_structured_data | Complete matched raw and finished pair under RN-18. |
| url. D enum | snapshot_change_state | Sealed comparable same-site captures; unread or withdrawn does not prove removal. |
| url. D bool | status_changed, robots_changed, title_changed, link_set_changed, layout_changed | Comparable old/new evidence for the exact field. |
| url. P bytes | literal_path_octets | F-0132 count of encoded path bytes, before any decoding. Refuse if the path or even its length is protected. No path output. |
| url. P count | literal_path_segments, query_parameters, literal_path_underscores | F-0132 uses the admitted parser and exact literal coordinates. Count nonempty slash-delimited path segments, admitted query occurrences and literal underscores. No decoded separator, deduplication or query material. Withhold protected counts. |
| url. P count | duplicate_title_peer_pages, duplicate_description_peer_pages | F-0148 and the 0044 duplicate-head evidence, based on exact admitted comparison and named complete capture scope. Count other distinct eligible pages; zero needs complete coverage. Keep title and description groups separate. |
| url. P measure | title_preview_css_px, description_preview_css_px | F-0146/F-0148 local width estimates in CSS pixels using admitted glyph, script, font and viewport profile. No live search result claim. |
| url. P count | literal_http_internal_outlinks, tracking_parameter_outlinks, mixed_internal_variant_groups, linked_noindex_pages | F-0133–0136; respectively observed literal HTTP occurrences, occurrences with a known tracking key, proven target groups with at least two variants, and distinct current internal destinations with observed noindex. No target read, key or value output; unresolved or stale evidence withholds the affected field. |
| url. P enum | www_alias_state, trailing_slash_variant_state, path_case_variant_state, final_response_scheme | F-0129–0131 use independently admissible exact identity pairs. The separate scheme field is the verified final response's transport scheme; it is not a scheme inferred from an href or a guessed redirect. |
| url. R count | visible_placeholder_occurrences, empty_visible_anchor_occurrences, generic_visible_anchor_occurrences, safe_reveal_only_link_occurrences | F-0137/F-0138/F-0152; count only eligible visible exact occurrences under the admitted rule or matched safe reveal pair. Accessible names are separate. Partial render cannot prove absence. |
| url. S count | reciprocal_alternates, missing_return_alternates, malformed_alternate_declarations, alternate_404_endpoints | F-0564 in the own-site Audit sample. Count distinct eligible page alternates for paired outcomes and declaration occurrences for malformed codes. Both current comparable endpoints are needed for returns or 404; incomplete coverage withholds the affected count. |
| url. D bool | headings_changed, canonical_address_changed | LS-14 exact same-site matched diff. Neither field follows from an unknown new page. |
| url. D count | added_link_occurrences, removed_link_occurrences | LS-14 exact same-site stable mapping and complete comparable edge scope. An omitted or withdrawn edge supplies no removal. |
| url. 0116 bool | observed_graph_orphan, observed_graph_broken_hub | LS-14/15 exact admitted discovery root and scope. Orphan requires known page and complete eligible inbound scope. Broken hub needs a separately admitted count threshold; no default. |
| url. S UTC | observed_declared_publication_time, observed_declared_modification_time | F-0087 compatible exact 0044/0115 chosen date type. Require an unambiguous timezone and verified source precision. Sitemaps and retrieval time cannot substitute for declared publication. |
| link. E ID | source_ref, destination_ref, capture_ref, previous_redirect_ref, final_redirect_ref | Exact occurrence or eligible observed target. |
| link. E enum | purpose, destination_scope, destination_scheme, destination_http_status, relation_kind, occurrence_stage, destination_fetch_outcome, destination_robots_state, destination_canonical_state, fragment_state | Each target fact needs its matched observation. No proxy from source anchor or inferred index state. |
| link. E count | redirect_hops, source_occurrence_ordinal, repeated_target_occurrences | Retain distinct positions; deduplicate replay. Repeated target count needs its complete population. |
| link. E pos | source_occurrence_position | Safe position in the admitted occurrence coordinates. |
| link. E UTC | observed_at, destination_observed_at | Preserve distinct source and target observation times. |
| link. E chars | anchor_length | Exact eligible complete anchor count in the defined unit. |
| link. E label | source_address, destination_display | Current safe redacted text only; foreign address inert. |
| link. E bool | rel_nofollow, rel_ugc, rel_sponsored, anchor_empty, anchor_is_image, has_fragment, rendered_only, destination_redirects | True and false both need the specific observed slot. Never coalesce an unread target. |
| drawer. L enum | capture_source, capture_completion, clock_domain, attempt_kind, attempt_cache_state, attempt_protocol, attempt_outcome, phase_kind, phase_state, frame_state | One 0142 subject per view; label supplied claims distinctly. |
| drawer. L count | capture_sent_attempts, capture_refused_before_send, capture_replayed_attempts, capture_cache_only_hits, captured_frame_count, dropped_frame_count, attempt_sequence, frame_sequence | Exact 0142 scoped population or partial lower bound kept separately. |
| drawer. L us | first_byte_tick, last_byte_tick, phase_start_tick, phase_end_tick, frame_presentation_tick, first_contentful_paint, largest_visible_paint, visual_progress_time, time_to_first_byte, scripted_step_to_paint | Exact versioned eligible measure and clock bounds. No lab INP. |
| drawer. L us | local_load_window_blocking_duration | 0142 local FCP-to-settle window on one real clock. Sum intersections of each eligible post-50 ms task interval with that window. Both boundaries and complete long tasks are required. |
| drawer. L us | total_blocking_time | Requires a separately admitted reference-equivalent TBT definition and fixture. The initial 0142 local load-window value never fills this field. Retain exact version, interval, engine evidence and units. |
| drawer. L bytes | attempt_transfer_size | One verified attempt, not page total. |
| drawer. L measure | eligible_layout_stability | Exact admitted method and units. Field CLS and simulated movement remain separate types of evidence. |
| drawer. L ID | frame_preview_ref, visual_event_ref, calibration_ref | Safe admitted presentation referent; never a vault handle. |
| drawer. T count | eligible_page_tokens, eligible_corpus_pages, term_page_count, term_document_frequency, term_corpus_frequency, eligible_sentences, eligible_syllables, keyphrase_page_support | Exact 0144 operation scope, term subject, language pane and result seal. |
| drawer. T bytes | editorial_text_bytes, decoded_html_bytes | Same eligible projection and input capture. |
| drawer. T ratio | content_to_html, confirming_similarity | Exact 0144 ratio profile or confirmed pair. Withhold unsupported or suppressed pairs. |
| drawer. T measure | tf_idf_weight, bm25_weight, reading_ease, reading_grade, collocation_g_squared | Require named admitted formula and unit. Reading grade describes one defined reading measure only. |
| drawer. T enum | corpus_result_state, corpus_language_state | Safe approved operation state; no inference about people. |

### Finite field families

Each tuple expands into the exact printed dotted path. Expand only
the listed tokens. Each field is a separately meaningful count or
byte quantity with numeric order and the common nullable causes.

**Endpoint counts.** Internal inbound uses the observed referring
endpoint. Internal outbound uses the observed destination. Require
an eligible matched same-site capture or comparable stored
observation. External counts use only the eligible stored 0044
outbound destination observation. No external inbound reconstruction.
Require a complete eligible occurrence population and use one
observed 0044 purpose and one result bucket per occurrence. An
unproved purpose cannot be allocated to a guessed bucket.

| Pattern | Exact axes | Field count |
| --- | --- | ---: |
| url.internal_edge_count.{direction}.{purpose}.{result} | direction in, out; purpose post, navigation, listing; result below | 72 |
| url.external_out_edge_count.{purpose}.{result} | purpose post, navigation, listing; result below | 36 |

| Exact result token | Observed endpoint result |
| --- | --- |
| success | 200 through 299 |
| permanent_redirect | 301 or 308 |
| temporary_redirect | 302, 303 or 307 |
| not_modified | Observed 304; no newly observed complete response implied |
| other_redirect | Remaining 300 through 399 |
| not_found | 404 |
| gone | 410 |
| refused_response | Observed 401 or 403; a local policy refusal is separate |
| rate_limited | 429 |
| other_client_error | Remaining 400 through 499 |
| server_error | 500 through 599 |
| unresolved | No eligible final result, including local refusal, unknown or interim-only |

An unresolved count is the number of eligible occurrences without a
final observed endpoint status. No address details accompany that
count. The twelve bins share the same population and sum to it when
all are eligible. Withhold the breakdown when totals and differences
could disclose protected or suppressed facts. A partial population
has no complete bucket total. Preserve the eligible occurrence
identity to distinguish one replay from true repeated positions.

**Relation counts.** Expand
url.internal_rel_count.{direction}.{purpose}.{relation} across
direction in, out; purpose post, navigation, listing; and relation
nofollow, ugc, sponsored. There are 18 occurrence-count fields.
Use 0044/0141 exact relation attributes. An occurrence can contribute
once to each applicable relation, so these counts overlap. Withhold
on incomplete relevant relation or occurrence scope.

**Page resource totals.** Expand url.resource.{kind}.{quantity}
across html, script, stylesheet, image and transfer_bytes,
decoded_bytes, observed_transfers. There are 12 fields. Use 0044
F-0143 and admitted 0115/0142 bytes and attempts. Each physical
transfer has one pinned kind. Deduplicate replay receipts while
retaining actual repeated transfers and exact attribution. Cache
reuse cannot invent a transfer. Withhold totals unavailable from raw,
denied or partial input. Observed transfers count completed actual
transfers with admitted evidence, separate from the report's wider
request count. Bytes in different bases are never summed together.

### Selection, channel rights and release

Each manifest entry holds field ID, stable UUID, title, owning
producer and version, row type, value type/unit, applicability,
nullable cause mapping, admission and permission conditions, order
rules and named proof. A changed unit or meaning takes a new ID.
A compatible adapter change retains the ID in a new manifest
revision. Keep the sealed manifest digest on saved views and audits.

Sort the field selector by its display name under the pinned British
English collation, then field UUID. Sort complete comparable row
values by their declared order, nulls last in either direction, then
safe display name and row UUID. Withhold value sort for pointers,
non-comparable measures and refused values. The scoped cursor pins
actor, authority generation, manifest, sealed evidence revision,
filter, sort and field selector version. A stale cursor cannot widen
scope or preserve a formerly visible count.

The desktop-only fields are url.native_object_ref and
drawer.frame_preview_ref. They may be used only inside the authorised
local desktop view. Withhold them in MCP and every file or report
export, including derived fields, filters, manifests and proof
attachments. An otherwise matching grant or allowlist entry cannot
override this fence. A forged desktop token fails at the channel gate.

All fields here are protected app candidates. MCP and local export
each require a separate destination allowlist entry binding exact
field version, approved site, actor, purpose and safe projection
under 0020 or 0094/0100. No wildcard follows from the catalogue. No
vault pointers, raw bodies or headers, private text, unredacted
restricted addresses or raw trace through an export, MCP or filter.
0104 Python can read only its separately admitted immutable broker
projection. It receives no live catalogue store or ambient access.
Permission checks run before sorting, counts, paging or delivery.

The release manifest verifies the complete expansion, producer and
granularity per field, unique IDs and UUIDs, typed position and unit,
nullable cases, sort semantics and current channel admission. Report
defined fields, working adapters and channel admissions separately.
Keep unavailable or omitted fields out of the coverage numerator.
This catalogue defines 253 URL fields, 32 link fields and 54 drawer
fields. The URL count is 115 scalar fields and 138 family members.
These are design counts.

| Planned proof | Fixture and expected result |
| --- | --- |
| IT-0145-FIELD-MANIFEST | Expand every exact tuple and prove 253 URL, 32 link and 54 drawer IDs, with disjoint keys. Reject duplicate ID/UUID, unknown axis or missing source/type. Render or labels give no new ID. |
| IT-0145-ENDPOINT-BINS | All status buckets, 304, local denial, unseen endpoint, partial population and repeated occurrence versus replay. Check complete disjoint sums and no hidden disclosure. |
| IT-0145-RELATION-RESOURCES | Multiple attributes, shared-resource attribution, repeat receipts, separate actual transfer, cache and lost byte source. Show only admitted supported totals. |
| BDD-0145-TYPED-ORDER | Equal labels and UUID tie, mixed clocks, missing value and observed zero. Keep deterministic order and invalidate a cursor on revoked scope. |
| IT-0145-CHANNEL-SCOPE | Test a supported protected app field, foreign scope, missing destination allowlist and revoked source. Attempt API, MCP, export, Python, drawer and list filters; release only the exact scoped projection. |
| IT-0145-EXTENDED-URL | Compare sealed page/diff records with distinct path, head, alternate, visible-content and graph facts. Unknown alternates, partial captures, private count inputs and unread dates stay unavailable. A count of newly added occurrences is distinct from a known removed count. |
| PL-0145-DESKTOP-ONLY | With explicit otherwise-valid grants and field allowlists, show eligible native reference and safe frame preview only to desktop. MCP, local export, expressions, report attachments and forged channel return neither values nor existence. |
| IT-0145-BLOCKING-METHOD | A complete approved local 0142 trace yields only the FCP-to-settle measure. A reference-qualified trace supplies TBT only under its own method. Try a sparse deadline, missing clock or cross-method sort; withhold the affected output. |

## Structured-data rules and bot policy coverage

The Toolbox reports the exact installed revision, qualified case counts and
source evidence dates. The expanded Toolbox requires a verified inventory of
at least 190 distinguishable structured-data checks and at least 250 distinct
supported bot identities with policy cases. Verify each Toolbox inventory
against its own signed case evidence. Spec 0064 owns feature and variant
qualification and spec 0055 owns separate method admission. An inventory
entry alone supplies no method or capability credit.

### Structured-data layers

Each outcome belongs to one layer. A document parser handles JSON-LD,
Microdata or RDFa through an admitted 0115 profile. Vocabulary checks use the
approved local Schema.org release. A selected search platform pack evaluates
its own documented rules. An approved finished-page comparator matches markup
to visible evidence from the same site, page and verified revision. A result
never fills a missing result in another layer. The local JSON-LD builder may
create an inert proposal from the same admitted types and fields. It has no
publication route.

| Family | Stable semantic key | Rule and result |
|---|---|---|
| SD-DOC | Syntax form and structural predicate | Report a syntax failure with complete admitted input. Expand only a pinned compatible local context. Missing or unsupported contexts give unknown. Never resolve remote contexts, terms, entities or references through the parser. |
| SD-TERM | Vocabulary role and predicate | Test the admitted vocabulary role for type, property or enumeration. Recognise an approved extension only through its separate inventory. A term outside the selected vocabulary can be reported. Unknown extension semantics cannot invalidate a whole document. Generic recognition counts once per role, never once per hypothetical unknown term. |
| SD-DOM | Property IRI and domain predicate | Check whether any declared subject type or published ancestor matches the union of the property's domainIncludes classes. A proved mismatch is vocabulary advice under the pinned profile. An unresolved extension or incomplete typing yields unknown. |
| SD-RNG | Property IRI and range predicate | Check each instance or literal against the union of the property's rangeIncludes classes, datatypes and enumerations. Apply documented Schema.org text/URL conventions and admitted parser mappings before claiming a mismatch. A proved mismatch is advice; an unresolved value gives unknown. |
| SD-CARD | Syntax collection requirement, or target profile and exact clause | Schema.org generally allows repeated values. No generic absent-field or multiple-value penalty follows from domain or range. Enforce structure only for the explicit data syntax, or enforce documented minimum, maximum or conditional cardinality in its separately qualified profile. |
| SD-POL | Platform, feature and exact clause key | Evaluate only a documented requirement or recommendation explicitly admitted for the selected target. Keep required, recommended and untestable policies distinct. Report local evidence and limits without a predicted rich display, ranking or eligibility. |
| SD-VIS | Visible claim and exact field | Compare an eligible declared claim with an admitted matching visible observation. Resolve the subject, role and units first. If complete qualified visible coverage proves an absence, report no visible support under RN-22. Missing comparison yields cannot tell. Neither result establishes concealment or platform policy breach. |

### Deterministic rule inventory

Pin the versioned official Schema.org machine-readable inventory, bytes and
hash, licence, role/subclass mappings and supported categories. Schema.org
30.1 is a documented candidate, subject to the normal repository admission.
Never run against a moving latest URL. Exclude pending, superseded and
extension terms from the core unless a separate approved pack covers them. Pin
each platform policy by official target feature and general guideline, capture
edition, date and bytes/hash, exact source locator, requirement versus
recommendation, applicability, executable predicates, witnesses, reviewer and
withdrawal/expiry. An illustrative example never becomes a rule. Product,
reviews and merchant eligibility remain under 0103/QR-058.

Form the canonical tuple (rule family, origin namespace, semantic subject key,
predicate kind, condition discriminator) using approved canonical encoding and
SHA-256. Use the family and digest as the check key and sort the tuple fields
bytewise in UTF-8. Keep rule IDs when only the source locator, publication
revision or wording changes. Assign a different key and retire the old key
only when the actual predicate or its applicability changes. Preserve the
versioned relationship in the manifest. Do not count a retired key, a
spelling/alias, changed threshold, device variant, equivalent requirement or
an additional encoding of the same predicate.

Each property yields at most one domain and one range check over its complete
admitted unions. A distinct check needs a distinct named semantic subject and
a meaningful positive and negative witness under the exact allowed input type.
Do not count each union member, repeated assertion, descendant, term or
materialised CSV row. A separately qualified platform clause needs its own
observable predicate or independently effective applicability. A repeated
platform predicate can share the generic check and attach another source.
De-duplicate by exact normalised predicate and by equivalent admitted
witnesses in independent review. Bind the 153 general audit check catalogue's
aggregate markup findings to exact check outcomes and count a shared check
only once. This inventory does not add method or capability credit.

For every purported counted key generate stable FX-SD cases ON, CLEAR and
UNKNOWN. For a domain case use a valid property with a subject proved outside
its allowed union and then a subject inside, keeping the value fixed. For a
range case use known outside/inside values with the same valid subject. For
conditional/cardinality cases cover the applicable boundary, one outside,
condition false and an unknown prerequisite. For a visible comparison use a
complete agreeing pair, a complete contradictory pair, a complete candidate
scope proving no visible support, and a missing or unmatched visible
observation. UNKNOWN always withholds a judgement. Include independently
checked tests for aliases, repeated values and inheritance. Reject a counted
row with no distinct bounded fixture.

Sign the inventory and its exact measured count with the pin, rule keys,
provenance, qualification version, deduplication, cases, results and reverse
links from every counted rule to current evidence. Require identical
deterministic enumeration and fixture outcomes across the admitted Linux and
Windows Bazel profiles. Unknown, expired, unqualified, incompatible or
unproved rows do not count. Fewer than 190 qualifying rows blocks the expanded
Toolbox claim.

### Initial visible comparison scope

Spec 0115's Documents, feeds, media and structured data clause identifies six
visible comparison families under RN-22 and UT-RN-20. Report a declared fact
with no visible support when eligible finished-page coverage proves that
absence. Mark a missing, withdrawn or partial observation as cannot tell. This
gives an exact local starting scope. It does not prove six completed
comparators or a platform policy pack. QR-045 is a question for 0114 database
coverage, independent of markup. Use 0115 and QR-050 for the existing render
route, EX-02 and QR-029 for the necessary exact rendering/cage evidence, and
0103 QR-058 for commercial search guidance. Any unsupported source stage
remains unavailable. Google's Article documentation has no required
properties, so an applicable omission can be a documented recommendation only.
The independently documented Breadcrumb profile has its own applicability and
constraints.

| Family | Exact evidence match before comparison |
|---|---|
| Price | Same eligible offer, currency, price role and units. Compare admitted exact decimal values after documented display normalisation. |
| Rating | Same eligible subject, rating population, role and admitted scale. Use exact values on compatible scales only. |
| Review count | Same eligible subject and explicit review count basis. Keep rating count separate. Use exact counts. |
| Breadcrumb | Same eligible page and breadcrumb path. Compare ordered admitted anchor destinations and available labels in the same positions. |
| Date | Same eligible entity, date role, precision and timezone. Ambiguous precision or timezone yields cannot tell. |
| Name | Same eligible entity and name role, using admitted whitespace and Unicode normalisation equally without fuzzy matching. |

### Qualification of the offer and FAQ comparisons

Use 0115's Local visible offer and FAQ evidence contract and the one
0044 F-1316 validator. Read only the matching typed fact for the exact
own-site current page revision and capture. Show each rule as
unavailable until its own signed producer profile, rule and fixture
evidence are current. Keep these cases outside the six initial visible
comparison families. Do not count either row towards the structured
data inventory until it independently qualifies.

Given complete, equal-scope facts, MK13 reports a mismatch only when
the two stock states differ. Otherwise it clears. MK16 first requires
the exact question key, matching normalised question and accepted
answer role. Compare the complete normalised answer digests. Different
digests show “The marked FAQ answer differs from the visible answer.”
Same digests clear. A different question text, absent match or
inadmissible evidence gives unknown with the safe producer reason.
For a hidden or partial saved answer, show “This saved view did
not show the full FAQ answer.” These findings describe only this
render. They do not prove market
availability, factual contradiction, search policy or rich appearance.

| Fixture | Exact synthetic case and observable result |
|---|---|
| FX-SD-MK13 | Put two offers with distinct known variants on one admitted page revision. Mark both offers as in stock. A has a visible “Out of stock” span and B has “In stock”. Report A’s mismatch and B’s clear result. Keep B unchanged and change A's visible span to “In stock” to clear. Remove or hide A's visible span, substitute the variant key or offer A's old render, introduce two visible conflicting A spans and use an unmapped status in separate runs. Each leaves A unknown. An offer with only a cart button gives no stock state. |
| FX-SD-MK16 | Match one synthetic question key and the normalised question “When does the hall open?” Mark its answer “At nine.” and show “At ten.” to report exact wording mismatch. Show “At nine.” to clear, including equivalent NFC and whitespace fixtures. For separate runs show two matching question regions, duplicate answer regions, a closed answer whose DOM contains the same text, a different question under the same key and an answer from an earlier revision. Each gives unknown, never a false clear or finding. |
| Both, via UT-RN-20 and TB-08-A | Verify the same permitted outcome in the Toolbox and its audit table. Check exact and one over the producer slot, key and text caps, partial or unproved regions, wrong site, changed revision, lack of either independent profile and rights withdrawal after staging. Hold the affected row without using another field, older capture or different device. The replay starts no fetch, interaction, edit, publication or ranking change. |

### Bot policy registry

The approved registry is inert, local and versioned. Each row contains stable
bot key, operator key, kind (crawler, fetcher, policy-only), exact documented
identity token or claim pattern for local classification, optional documented
robots control token, documented roles and aliases, documented robots
applicability (matched policy, exception or unknown), dated official source or
independently reviewed owner-supplied evidence, source digest, allowed case
classes, reviewer and expiry/withdrawal. An owner-supplied bundle is countable
only where its entries have independently reviewable sources for the exact
names and behaviour. A freestanding list or unsupported user-agent string is
reference material only.

Apply RFC 9309 matching and normalisation only within the owning parser and
version. The same identity with different casing, versions, mobile/desktop
forms, IPs, aliases, roles or alternative claims counts once. A proprietary
policy-only control may have a separate sourced QR-025 simulation and never
expands the REP matcher or counts as a network bot. Different named actual
fetchers count separately only when the operator evidence and qualified
offline semantics support independent identities. An operator-documented
exception to ordinary policy may count with its explicitly scoped offline
cases. An unqualified or unknown policy cannot count.

Simulate against an approved local draft or saved live robots capture. Attach
registry revision, source and matching rule/line when eligible. Results
distinguish allowed by simulated policy, disallowed by simulated policy,
outside policy scope and unknown. Independently report actual permitted
Leviathan status, qualified headers and raw/main-text evidence for its
declared identity only. Display render differences only from compatible
admitted evidence. Never present a simulated policy as observed service
access, HTTP response, render, identity proof or observed visit for that bot.
A bot choice creates no request; live declared-profile comparison cannot use
registry tokens in a request. Actual opt-out enforcement and log provenance
remain with 0082/QR-025 and 0141 LG-03 respectively.

| Documentary seed, pending admission | Offline case boundary |
|---|---|
| Googlebot | One product-token entry for Smartphone and Desktop per official documentation. |
| Applebot and Bingbot | Two independently documented search crawler identities. Apple's published fallback, if admitted, receives a separate policy fixture rather than another identity. |
| OAI-SearchBot, GPTBot and ChatGPT-User | Distinct operator-documented search, model-development and user-request classes. User requests may have different robots applicability; expose the documented scope or unknown without inferring network behaviour. |
| Claude-SearchBot, ClaudeBot and Claude-User | Distinct documented search, model-development and user-request identities. Use the exact dated operator policy of each. |
| Google-Extended and Applebot-Extended | Policy controls. Excluded from the actual bot identity count. |

For every counted matched-policy identity require synthetic FX-BOT cases
ALLOW, DENY and UNKNOWN. Exercise exact group, wildcard fallback, case
equivalence, duplicate group, URI match boundary and withdrawn source. An
admitted exception requires EXCEPTION, OUTSIDE-SCOPE and UNKNOWN cases for the
exact conditions stated in its source. Every case must prove zero requests and
independent status/display separation. Use only inert strings and synthetic
sites. Sign registry keys, provenance/revisions, admitted roster, excluded
duplicates/policy-only/unknown entries, fixture outcomes and reverse links.
Fewer than 250 independently qualified identities blocks the 250+ claim. This
draft identifies nine documentary crawler/fetcher candidates and two
policy-only controls. It establishes no qualified roster or verified 250+
total.

### Inventory boundary cases

Recheck installed rule and registry versions, case and source
manifests and current authority before displaying qualified counts
or supplying release evidence under 0064. A missing, stale or
wrong-candidate or wrong-scope inventory receipt shows “Inventory not verified”
for that inventory and supplies no qualified count. A valid
receipt below the required count shows the known count and holds
qualification of that expanded Toolbox requirement. Under 0140
and 0064, both exact current passing receipts are needed for
expanded Toolbox scope. Individual tools retain their own
eligibility. QR-235 tracks the exact required case and source
evidence.

| Fixture | Exact synthetic case and expected result |
|---|---|
| IT-0145-SD-INVENTORY-BOUNDARY | Give a same-version valid synthetic inventory with 189 distinct qualified checks. Try one schema alias and one unqualified visible-rule profile. The count stays 189 and the structured-data requirement is held. Replace those supplements with one distinct fully proved check for 190. Only this inventory check passes. With a mismatched rule pin or invalid receipt, show the integrity hold without a qualified count. |
| IT-0145-BOT-INVENTORY-BOUNDARY | Give a same-version valid synthetic inventory with 249 distinct policy-qualified crawler or fetcher identities. Offer an alias, a policy-only control and an unreviewed user-agent claim. The count stays 249 and the bot registry requirement is held. Give one further fully proved independent identity and it passes this check at 250. An expired row supplies no credit. Rederive the count only from current verified rows. A wrong registry revision or an unproved ledger gives the integrity hold without a qualified count. None of these inputs proves an observed visit or starts a request. |

Keep one inventory current and passing in each shortage and
missing-proof test of the other. Repeat with the roles reversed.
Both hold expanded Toolbox scope. Two passing exact receipts
supply only that scope’s evidence to 0064.

### Official sources and remaining parity gaps

Checked 21 November 2026. [Schema.org data
model](https://schema.org/docs/datamodel.html), [Schema.org version
30.1](https://schema.org/version/30.1/) and [machine-readable
sources](https://schema.org/docs/developers.html). [Google structured-data
guidelines](https://developers.google.com/search/docs/appearance/structured-data/sd-policies),
[supported
features](https://developers.google.com/search/docs/appearance/structured-data/search-gallery),
[Article](https://developers.google.com/search/docs/appearance/structured-data/article)
and
[BreadcrumbList](https://developers.google.com/search/docs/appearance/structured-data/breadcrumb).
[RFC 9309](https://www.rfc-editor.org/rfc/rfc9309.html), [Google common
crawlers](https://developers.google.com/crawling/docs/crawlers-fetchers/google-common-crawlers),
[Googlebot](https://developers.google.com/search/docs/crawling-indexing/googlebot),
[Applebot](https://support.apple.com/en-gb/119829),
[Bingbot](https://blogs.bing.com/webmaster/2022/4/Announcing-user-agent-change-for-Bing-crawler-bingbot/),
[OpenAI crawlers](https://developers.openai.com/api/docs/bots) and [Anthropic
crawlers](https://support.claude.com/en/articles/8896518-does-anthropic-crawl-data-from-the-web-and-how-can-site-owners-block-the-crawler).

TechnicalSEO.com's public directory describes the JSON-LD generator and
separate bot, locale and mobile utilities. It does not establish a 190-rule
validator, a roster of 250 documented identities, offline NGINX/Caddy
simulation, all requested console and resource-stage details, an exact
hreflang graph or the requested local-search integration. Screaming Frog's
catalogue advertises an issue count. It does not verify independent Sutradhar
checks, arbitrary post-crawl search across all three surfaces, a native Matomo
join or all modern cloud log dialects. Keep these as independent scope or
evidence gaps, not confirmed vendor parity. [TechnicalSEO.com
index](https://technicalseo.com/tools/) · [Utility
descriptions](https://technicalseo.com/tools/pages/home.php) · [SEO
Spider](https://www.screamingfrog.co.uk/seo-spider/) · [Issue
catalogue](https://www.screamingfrog.co.uk/seo-spider/issues/) · [Log
formats](https://www.screamingfrog.co.uk/log-file-analyser/tutorials/introduction-to-log-files/)

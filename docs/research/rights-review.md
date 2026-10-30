# Rights and clean-room review

Status: admission policy draft. No catalogue candidate has rights clearance.

Sutradhar uses independently written first-party logic based on approved
specifications. Reviewed third-party dependencies are allowed under their
own terms. The repository's MIT licence does not replace those terms.

## Separate the questions

The review considers the source text, proposed implementation, dependencies,
model weights, datasets, assets, names and patents separately. An article
being readable on the web does not grant permission to copy its expression.
A permissive software licence does not by itself resolve every patent,
dataset or model issue.

For context, the US Copyright Office distinguishes methods and ideas from
their protected written or visual expression. The UK guidance identifies
copyright, patents, designs and trade marks as separate types of protection.
These are starting points for review, not a clearance for a particular
implementation. [US Copyright Office, Circular 33](https://www.copyright.gov/circs/circ33.pdf),
[UK Intellectual Property Office overview](https://www.gov.uk/intellectual-property-an-overview).

The review covers the intended UK, EU and US distribution. Do not infer a
blanket software exception from a short summary of patent law. Record the
actual claims, applicable territories and current legal status for a
relevant patent family. [European Patent Convention, Article 52](https://www.epo.org/en/legal/epc/2020/a52.html),
[USPTO patent management guidance](https://www.uspto.gov/patents/basics/manage).

## Required process

1. Identify the primary sources and the permission to access them. Do not
   use leaked, confidential or unauthorised material.
2. Write an independent specification of behaviour and mathematics. Keep
   a record of which sources informed each part. Avoid copied prose,
   diagrams, tables, examples, code and translated source implementations.
3. Record relevant rights questions and evidence in the admission record.
   Independent writing is not a workaround for an unresolved patent claim.
4. Review a proposed dependency, model or dataset separately. Record the
   exact version, licence, required notices, redistribution terms and
   relevant restrictions. Do not assume the parent repository's licence
   also covers bundled weights or training data.
5. Obtain owner approval for the specification before implementation. Any
   additional specialist review required by the evidence remains a block
   until it is resolved.
6. Review the resulting change for provenance, copied expression and
   retained notices. Record the reviewer and exact version.

Only reviewed terms compatible with the agreed permissive commercial
distribution policy may be admitted. An unresolved active or uncertain
patent concern excludes the method from implementation and release
qualification. Expiry claims require a dated record of the relevant
jurisdictions, claims and family members; a publication date alone is not
that record. Changed evidence reopens admission review.

## What research status means

The initial catalogue contains descriptions, citations and proposed tests.
It contains no copied third-party implementation, downloaded paper, model
weight or dataset. Source access levels are recorded honestly; reading an
abstract is not a full algorithm review.

Some older papers in the ACL Anthology display non-commercial and
share-alike publication terms, while newer papers display different terms.
This catalogue links to those papers and uses independently written short
research descriptions. It does not relicense or redistribute the papers.
See the notices on the [TextTiling source page](https://aclanthology.org/J97-1003/)
and [Sentence-BERT source page](https://aclanthology.org/D19-1410/).

Pixie and other named systems are research candidates, not claims to reuse
their code, brand, operational performance or patent rights. The requested
Google information-gain concept remains outside admission while its
specific rights questions are unresolved. Generic statistical information
measures must not be described as an implementation of a proprietary
ranking system without evidence.

## Public records

Publish only scrubbed citations, independent specifications and safe review
outcomes. Do not commit confidential advice, access credentials, forum
identity, private corpus material or identifying infrastructure details.
Where evidence must remain private, the public record states its scope and
status without exposing that evidence. A missing or unverifiable private
receipt cannot be treated as a passing public gate.

No document or agent can guarantee the absence of every possible legal
claim. The enforceable promise is to follow the approved admission process,
exclude unresolved concerns and avoid claiming clearance without evidence.

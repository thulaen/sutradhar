"""Owner-approved exceptions and path classes (decisions of 30 Oct 2026).

Exceptions: a line holding one unbreakable link and nothing else may be up
to 200 code points, provided the text beside the address fits in 80; lines
inside historical quotation code blocks under docs/requirements/sources/
keep their exact length. File length and forbidden characters have no
exceptions. Lockfiles, binary files and third-party material are
inventoried but not line-measured.
"""
import bisect
import glob
import math
import posixpath
import re

from tools.limits import counting

LINK_LINE_LIMIT = 200
QUOTATION_ROOT = "docs/requirements/sources/"
CLASSES = ("text", "data", "lockfile", "binary", "third_party")
_MEASURED = ("text", "data")

_LINK = re.compile(
    r"(?:\[[^\]]*\]|[^\[\]()]*\])\((\S+)\)[.,;:]?|<?(https?://\S+?)>?[.,;:]?")
# A newline, then a fence line; searched in "\n" + text so that the
# literal newline lets the regular-expression engine skip ahead quickly.
_FENCE_AT = re.compile(r"\n {0,3}(`{3,}|~{3,})([^\n]*)")


def is_link_only(line):
    """True when the line is a single link, link tail or bare address."""
    return _LINK.fullmatch(line.strip()) is not None


def fence_after(marks, info, opener):
    """The open fence after a fence line (None when outside any block).

    CommonMark: a backtick opener's info string may not contain a
    backtick; a closer has the same mark, is at least as long as the
    opener and has no info string.
    """
    if opener is None:
        if marks.startswith("`") and "`" in info:
            return None
        return marks
    closes = marks.startswith(opener)
    return None if closes and not info.strip() else opener


def line_limit(body, quoted):
    """The length limit for one line under the owner's exceptions."""
    if quoted:
        return math.inf
    match = _LINK.fullmatch(body.strip())
    if match:
        address = match.group(1) or match.group(2)
        if len(body) - len(address) <= counting.MAX_LINE_LENGTH:
            return LINK_LINE_LIMIT
    return counting.MAX_LINE_LENGTH


def quoted_ranges(norm):
    """Ranges of line numbers strictly inside fenced code blocks."""
    ranges, opener = [], None  # opener: (marks, line) when open
    for line, match in counting.numbered(_FENCE_AT, "\n" + norm):
        marks = None if opener is None else opener[0]
        after = fence_after(match.group(1), match.group(2), marks)
        if opener is None:
            opener = None if after is None else (after, line)
        elif after is None:
            ranges.append(range(opener[1] + 1, line))
            opener = None
    if opener is not None:
        ranges.append(range(opener[1] + 1, counting.line_count(norm) + 1))
    return ranges


def check_file(path, text):
    """Violations for a measured text or data file, with exceptions.

    Quoted regions are worked out only when a long line needs them.
    """
    norm = counting.normalise(text)
    in_sources = path.startswith(QUOTATION_ROOT)
    cache = []

    def quoted(line):
        if not cache:
            ranges = quoted_ranges(norm)
            cache.append((ranges, [block.start for block in ranges]))
        ranges, starts = cache[0]
        index = bisect.bisect_right(starts, line) - 1
        return index >= 0 and line in ranges[index]

    def limit_of(line, body):
        return line_limit(body, in_sources and quoted(line))
    return counting.scan_normalised(path, norm, limit_of)


def load_classes(text):
    """Parse 'class pattern' rules; first matching rule wins."""
    rules = []
    for number, raw in enumerate(counting.split_lines(text), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) != 2:
            raise ValueError(f"line {number}: expected 'class pattern'")
        if parts[0] not in CLASSES:
            raise ValueError(f"line {number}: unknown class {parts[0]}")
        rules.append((parts[0], parts[1]))
    return rules


def _matches(pattern, path):
    target = path if "/" in pattern else posixpath.basename(path)
    regex = glob.translate(pattern, recursive=True, include_hidden=True)
    return re.fullmatch(regex, target) is not None


def classify(path, classes):
    """The class of the first rule matching path, or None."""
    for cls, pattern in classes:
        if _matches(pattern, path):
            return cls
    return None


def measured(cls):
    """Whether files of this class are measured against the limits."""
    return cls in _MEASURED

"""Physical-line counting and per-line checks (SPEC-007 counting rule).

Every physical line counts, including blank and comment lines. LF, CRLF
and a lone CR are each one line break. Length is measured in Unicode
code points. Control characters other than LF and CR, Unicode format
characters (category Cf, which includes zero-width and bidirectional
controls), NEL and the Unicode line and paragraph separators are refused,
because they can hide content from a reader or from the line count.

Scanning works on the whole text with C-level primitives and only visits
long lines and forbidden characters individually. Files over a size cap
are refused before scanning, and each file reports at most MAX_REPORTED
violations plus a count of the rest.
"""
import bisect
import codecs
import collections
import itertools
import re
import unicodedata

MAX_FILE_LINES = 300
MAX_LINE_LENGTH = 80
MAX_FILE_CHARS = 1_000_000
MAX_REPORTED = 50

Violation = collections.namedtuple("Violation", "path line kind value")

# Printable ASCII plus LF and CR; every other ASCII byte is forbidden.
_ALLOWED_ASCII = bytes(range(0x20, 0x7F)) + b"\n\r"
_NON_ASCII = re.compile("[^\\x00-\\x7f]")
_SEPARATORS = tuple(map(chr, (0x85, 0x2028, 0x2029)))  # NEL, LS, PS
# Unicode Default_Ignorable_Code_Point ranges (inclusive) that the format
# category (Cf) does not already cover: characters that render as nothing,
# such as variation selectors, Hangul fillers and reserved ignorables.
_IGNORABLE = (
    (0x034F, 0x034F), (0x115F, 0x1160), (0x17B4, 0x17B5),
    (0x180B, 0x180F), (0x2065, 0x2065), (0x3164, 0x3164),
    (0xFE00, 0xFE0F), (0xFFA0, 0xFFA0), (0xFFF0, 0xFFF8),
    (0xE0000, 0xE0FFF))
_IGNORABLE_STARTS = [start for start, _end in _IGNORABLE]


def is_forbidden(char):
    """Whether one non-ASCII character is refused.

    ASCII characters are decided by _ALLOWED_ASCII instead.
    """
    code = ord(char)
    index = bisect.bisect_right(_IGNORABLE_STARTS, code) - 1
    ignorable = index >= 0 and code <= _IGNORABLE[index][1]
    return (code <= 0x9F or char in _SEPARATORS or ignorable
            or unicodedata.category(char) == "Cf")


def forbidden_present(norm):
    """The distinct forbidden characters in normalised text, sorted."""
    ascii_bytes = codecs.ascii_encode(norm, "ignore")[0]
    bad = set(ascii_bytes.translate(None, _ALLOWED_ASCII).decode())
    if not norm.isascii():
        bad.update(c for c in set(_NON_ASCII.findall(norm))
                   if is_forbidden(c))
    return sorted(bad)


def normalise(text):
    """Text with every line break written as LF."""
    return text.replace("\r\n", "\n").replace("\r", "\n")


def split_lines(text):
    """Return the physical lines of text, without their line breaks."""
    lines = normalise(text).split("\n")
    if lines[-1] == "":
        lines.pop()
    return lines


def line_count(norm):
    """Physical lines in normalised text."""
    if not norm:
        return 0
    return norm.count("\n") + (0 if norm.endswith("\n") else 1)


def numbered(pattern, norm):
    """(line number, match) for each match, counting lines incrementally."""
    line, pos = 1, 0
    for match in pattern.finditer(norm):
        line += norm.count("\n", pos, match.start())
        pos = match.start()
        yield line, match


def decode(data):
    """Decode first-party text, which must be UTF-8."""
    try:
        return data.decode()  # Python 3 decodes as strict UTF-8
    except UnicodeDecodeError as error:
        raise ValueError(f"not UTF-8 at byte {error.start}") from error


def long_lines(norm):
    """(line number, text) for lines over the default limit.

    Lengths are compared in C (map and compress), not a Python loop.
    """
    lines = norm.split("\n")
    longer = map(MAX_LINE_LENGTH.__lt__, map(len, lines))
    for number in itertools.compress(itertools.count(1), longer):
        yield number, lines[number - 1]


def _per_line(path, norm, limit_of):
    per = {}
    present = forbidden_present(norm)
    if present:
        pattern = re.compile("[" + "".join(map(re.escape, present)) + "]")
        for line, match in numbered(pattern, norm):
            per.setdefault(line, [Violation(
                path, line, "forbidden_character",
                f"U+{ord(match.group()):04X}")])
    for line, text in long_lines(norm):
        if len(text) > limit_of(line, text):
            per.setdefault(line, []).append(
                Violation(path, line, "line_length", len(text)))
    return [v for line in sorted(per) for v in per[line]]


def cap(path, found):
    """At most MAX_REPORTED violations, then one count of the rest."""
    if len(found) <= MAX_REPORTED:
        return found
    extra = len(found) - MAX_REPORTED
    return found[:MAX_REPORTED] + [
        Violation(path, None, "more_violations", extra)]


def _default_limit(_line, _text):
    return MAX_LINE_LENGTH


def scan_normalised(path, norm, limit_of=_default_limit):
    """Violations for normalised text; limit_of gives long lines' limits."""
    if len(norm) > MAX_FILE_CHARS:
        return [Violation(path, None, "file_size", len(norm))]
    count = line_count(norm)
    found = []
    if count > MAX_FILE_LINES:
        found.append(Violation(path, None, "file_lines", count))
    return found + _per_line(path, norm, limit_of)


def check_text(path, text):
    """Violations for a text file with no exceptions applied."""
    return cap(path, scan_normalised(path, normalise(text)))

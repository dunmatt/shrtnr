# shrtnr
Shrtnr is a smart string shortener, suitable for command prompts, title bars, and other cases where
a long string might need to fit into a single short line.  It can be used either as a library or as
a stand alone CLI application.

## Status
I'm just starting development, definitely not ready for use yet.

## Raison D'Etre
There are a few things that sets `shrtnr` apart from other string shortening tools.  First, it is
general, the same function can be used on a file path, a URL, a political manifesto, or any
combination thereof.  And secondly, `shrtnr` doesn't overdo it, if it only needs to remove two
characters to hit its target length, then it will only remove two characters (this is in contrast
to many file path shorteners that reduce each directory to a single letter even when they don't
need to).

## Contributions
Contributions, both in the form of PRs and Issues are very welcome!

## Example Shortenings
TODO
(also, see the test cases)

## Simpler String Shortening Solutions
* shorten-url
* tico
* others? (if you know of more feel free to send a PR that updates this readme!)

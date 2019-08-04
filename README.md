# scripter
[![Build Status](https://travis-ci.org/jstasiak/scripter.svg?branch=master)](https://travis-ci.org/jstasiak/scripter)
[![Coverage Status](https://coveralls.io/repos/github/jstasiak/scripter/badge.svg?branch=master)](https://coveralls.io/github/jstasiak/scripter?branch=master)

A screenplay compiler.

## What does this do?

There's a nice [screenplay LaTeX package](https://www.ctan.org/pkg/screenplay)
but using it is not particularly convenient, as the code gets quite verbose.

Instead let's use an ad-hoc minimal script language and transpile to TeX.

## The rules of the language

* Whitespace at beginning and end of lines are ignored
* Lines with only whitespace in them are ignored
* The first line is the title
* The second line is the author(s)
* Empty lines are ignored
* Lines beginning with `INT. ` or `EXT. ` are treated as sluglines, what
  comes after `INT. ` and `EXT. ` is free-form
* Lines beginning with whitespace are assumed to contain dialogue. That requires
  them to contain at least one `:` character. The part before the first colon is
  is the character that's speaking, the second is what's being said. Parts in
  parentheses are treated as parentheticals. Colon characters other than the
  first are treated as plain text.
* Other lines are treated as description

## Example

Input:

```
The Alienant, version 1
Firstname Lastname

INT. SPACE STATION

Dark corridor. Something lurks in the shadows.

EXT. MILITARY BASE -- DAY

COLONEL SMITH smokes a cigarette. Looks up as CAPTAIN PARKER approaches.

CAPTAIN PARKER doesn't look too well.

	SMITH: So, it's begun.
	PARKER: Yes. (inhales) Yes, it has.
	SMITH: I was afraid it'd come to this. Look: how about we –
	PARKER: No.
```

TeX output:

```tex
\documentclass{screenplay}
\usepackage[T1]{fontenc}
\usepackage[polish]{babel}
\usepackage[utf8]{inputenc}
\title{The Alienant, version 1}
\author{Firstname Lastname}
\begin{document}
\coverpage
\fadein
\intslug{SPACE STATION}

Dark corridor. Something lurks in the shadows.

\extslug{MILITARY BASE -- DAY}

COLONEL SMITH smokes a cigarette. Looks up as CAPTAIN PARKER approaches.

CAPTAIN PARKER doesn't look too well.

\begin{dialogue}{SMITH}So, it's begun.\end{dialogue}

\begin{dialogue}{PARKER}Yes. \paren{inhales} Yes, it has.\end{dialogue}

\begin{dialogue}{SMITH}I was afraid it'd come to this. Look: how about we –\end{dialogue}

\begin{dialogue}{PARKER}No.\end{dialogue}

\fadeout
\theend
\end{document}
```

[Output PDF](test_script.pdf)

# rekordbox-fix

`rekordbox-fix` is a simple command line tool which:

- analyses an exported rekordbox collection in XML format 
- searches the folder on disk where your music files live
- produces a new rekordbox XML file containing the fixed tracks with their correct file locations, original cue points and loops, and the playlists they belong to. This XML file is generated containing your entire collection, including the tracks which the tool was able to fix.

The generated XML can be loaded into rekordbox via its "rekordbox xml" area.

## Motivation

I was looking for a similar tool and before rushing into writing code by myself, I did a quick search on Github. I stumbled upon [rekordbox-repair](https://github.com/edkennard/rekordbox-repair) and it looked fantastic, but it has been written in Scala. Instead, I was looking for something more _light & portable_ that I could eventually point my friends to.

Clearly, this task could have been achieved with much less effort if I'd have used something like Ruby + Nokogiri with which I'm already familiar. But I wanted a simple portable binary, and I thought that one day it might also have a GUI. So, once again, I have a lot of plausible excuses to use Rust.

## Status

**WORK IN PROGRESS**

## Credits
`rekordbox-fix` is heavily inspired/ported from [rekordbox-repair](https://github.com/edkennard/rekordbox-repair) which deserves all the credits for the idea and execution. 

---
©2023 a pavonz joint



# maf2bed

Used to convert multiple alignment format (MAF) files to a bed tabix-y style
format. Used in jbrowse mafviewer plugin
https://github.com/cmdcolin/jbrowse-plugin-mafviewer

## Usage

Make sure to specify the 'assembly name' being used as the reference for the bed
file as the first argument to maf2bed

```
zcat file.maf.gz | maf2bed assembly_name > file.bed
bgzip file.bed
tabix file.bed
```

## Footnote

Converted to rust from perl as a coding exercise mostly, gaining a modest
speedup on the way https://twitter.com/cmdcolin/status/1719608993310486883

## Motivation

I wanted to try using the bigMaf (bigBed based) format ecosystem with large MAF
files but bedToBigBed doesn't support streaming or reading compressed files(?),
so that requires reading big files on disk and in memory. in contrast, MAF tabix
type approach like implemented here can be compressed and streaming which allows
much low memory usage and disk space

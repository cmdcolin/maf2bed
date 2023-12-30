# maf2bed

Used to convert multiple alignment format (MAF) files to a bed tabix-y style
format. Used in jbrowse mafviewer plugin
https://github.com/cmdcolin/jbrowse-plugin-mafviewer

## Install

```
cargo install maf2bed
maf2bed --help
```

## Usage

Make sure to specify the 'assembly name' being used as the reference for the
bed file as the first argument to maf2bed

Example

```
export LC_ALL=c # improves sorting speed

#parallel compression/decompression
pigz -dc file.maf.gz | maf2bed hg19 | sort -k1,1 -k2,2n | bgzip -@8 > file.bed.gz

tabix file.bed.gz
```

Might be able to remove the sort/LC_ALL=c in some cases, but sorting ensures
that it will be ready for tabix

## Footnote

Converted to rust from perl as a coding exercise mostly, gaining a modest
speedup on the way https://twitter.com/cmdcolin/status/1719608993310486883

## Motivation

I wanted to try using the bigMaf (bigBed based) format ecosystem with large MAF
files but bedToBigBed doesn't support streaming or reading compressed files(?),
so that requires reading big files on disk and in memory. in contrast, MAF
tabix type approach like implemented here can be compressed and streaming which
allows much lower memory usage and disk space

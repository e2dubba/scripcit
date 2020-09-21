# scripcit 

A tool for finding possible scripture citations in free text, normalizing the 
name, and group the text according to their ranges. 

if the text has the following citation: `1 Cor. 4:3, 5, 6-7; 5:1-4`, the results will be:

```
	1 Corinthians 4: 3
	1 Corinthians 4: 5
	1 Corinthians 4: 6 - 4: 7
	1 Corinthians 5: 1 - 5: 4
```

The structs for these formats could easily be wrapped up in TEI, or someother form of Markup.

## Installing and Running

To run this: clone this repo, navigate to the directory and run `cargo run -- /path/to/file.txt`.
The scripture citations will be printed in the terminal. 

## To Do 

Things that still need to be finished off.

- [ ] Validate the number range to be certain it isn't impossible (like Exodus 50:234, etc)
- [ ] Prepackage the data Structures (the book library), so that the script can be run in any directory 
- [ ] Tweak and test the scripture regular expression scripts so that it works more broadly (especially older German, French and Spanish material)
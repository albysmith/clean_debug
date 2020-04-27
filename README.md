# X4 Debug Parser/Filterer

### Program takes up to 3 arguments:

#### --log (or -l) : full path to x4 debug log file (including file name) 
 - if you don't provide a path, it will look in the same folder as the exe  
#### --output (or -o) : folder where you want to print your filtered log file  
 - if you don't provide a path, it will place it in the same folder as the exe  
#### --tags (or -t) : tags you want to appear in your initial filtered log file  
 - need to be lowercase versions of tags, i.e. economy_verbose is [Economy_Verbose]
 - if you don't provide any tags, it will show all [=ERROR=] tags  
 - you will be able to re-filter after the initial pass; program will give you options for all available tags
 - each time you filter, it will update to include any new debug text that has been written to the file since your last filter

#### Currently only allows filtering debug messages; doesn't really parse yet.

#### Supports custom message tags in the following format:
[CUSTOMTAG] 23.21 print_your_message_here  

#### In mdscript, it might look something like this:  
<debug_text text="'[TPWAR] %s %s %s %s'.[player.age, $Ship, $Ship.knownname, $Ship.macro]" />


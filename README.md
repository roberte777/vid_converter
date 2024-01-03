# Getting this to work
## MacOS
I had to install the binaries like it said to in the docs.

I then added everything to my path with this command:
export PATH="/Library/Frameworks/GStreamer.framework/Versions/1.0/bin${PATH:+:$PATH}"

Additionally, for some reason it couldn't find some of the libraries it needed. I fixed this by 
doing this: 

export DYLD_FALLBACK_LIBRARY_PATH="/Library/Frameworks/GStreamer.framework/Versions/1.0/Libraries"

I think this may be a mac only issue

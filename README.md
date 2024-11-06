This is the repo for my PNG library. Currently it is just an implementation of the (fantastic) PngMe project but I plan to make it support decoding and encoding images, a functionality I will porobably use lateron. 

To run this code, git clone the repository with `git clone "https://github.com/jamdotjar/pngme"` and run `cargo run` to run the app.

The demo supports 4 commands (replace `pngmsg` with `cargo run` for all commands) :
  -  `pngmsg encode ~/path-to-file.png cHnk "message text"`: "hides" message text at the end of the provided png, in a chunk with the type cHnk ( chynk type can be any 4 letter code )
  -  `pngmsg decode ~/path-to-file.png cHnk`: Finds and returns the first occurance of a message within a chunk with type cHnk in the provided png.
  -  `pngmsg remove ~/path-to-file.png cHnk`: Removes the first chunk with type cHnk in the provided png.
  -  `pngmsg print ~/path-to-file.png`: Prints the chunks of the provided PNG

*note: because the decode and remove commands affect the first message found and encode adds to the end of the file, decode/remove will always affect the oldest message present.*


  

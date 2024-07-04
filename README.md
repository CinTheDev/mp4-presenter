# MP4 Presenter

A simple program written in Rust made for mp4-based presentations.

The crate `ffmpeg-next` is used for reading the input video files.

The video player is based on bevy.

## Functionality

Create a directory called `vid` in the same place as the executable, and place all
mp4 files inside it.

Note: The order in which the videos will be played is alphabetical. Prefix your files
with `0_...`, `1_...`, etc. to ensure the correct order for videos is maintained.

After a video has finished, the player will not continue with the next video
automatically, but will instead show the last frame of the current video. This is
mandatory so you have control over when animations will be played. Otherwise it would
just play out like a single video.

To play the next video, press on the right arrow key. You also can go back to the
previous video with the left arrow key, although it shouldn't be necessary in normal
cases.

If your videos are seamless between each other (last frame of some video = first frame
of next video) it should also play seamlessly with this player. (And that's the whole
reason I started this project).

## Helpful ressources

Here is a collection of useful online ressources for implementing a media player.

### Blog article about media player implementation

<https://999eagle.moe/posts/rust-video-player-part-1/>

Pretty low level using only ffmpeg as opengl. I wouldn't want to go that low since
this project is supposed to be very simple and straightforward.

Might give useful insights for specific things anyway.

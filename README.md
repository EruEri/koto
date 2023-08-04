# Koto

A little program to handle some of my music related action such as:

## Search

Search to the spotify api

```
$ koto search --help
Search for an item

USAGE:
    koto search [OPTIONS] <ITEM>
    koto search <SUBCOMMAND>

ARGS:
    <ITEM>    search item

OPTIONS:
    -a, --artist             search for an artist
        --album              search for an album
    -g, --graphic            Display graohic result (cover, picture, etc ...)
    -h, --help               Print help information
        --limit <LIMIT>      limit the result MAX Value : 50
        --market <MARKET>    market to look for
        --offset <OFFSET>    offset the result
    -t, --track              search for an track

SUBCOMMANDS:
    album     Search content related to an artist
    artist    Search content related to an artist
    help      Print this message or the help of the given subcommand(s)
    track     Search content related to an artist
```

## Edit

Edit metadata of music file such as title, images, etc ...

```
$ koto edit --help
Edit mp3 and flac file

USAGE:
    koto edit [OPTIONS] --type <FILE_TYPE> <FILE>

ARGS:
    <FILE>    Audio file

OPTIONS:
        --album <ALBUM>                      Set the album name
        --artist <ARTIST>                    Set the track artist name
        --artist-album <ARTIST_ALBUM>        Set the album artist
        --bpm <BPM>                          Set bpm
    -h, --help                               Print help information
        --images <IMAGES>                    Add images
    -o <OUTPUT>                              Output the
    -t, --title <TITLE>                      Set the music title
        --track-position <TRACK_POSITION>    Set track position Set track position
        --type <FILE_TYPE>                   [possible values: mp3, flac]
        --year <YEAR>
```

## CueSheet

Create cuesheet, either by fetch the information from the spotify api or by giving the information

```
$ koto cue-sheet --help

koto-cue-sheet
Create the cue sheet

USAGE:
    koto cue-sheet <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    fetch    Create the cue sheet by fechting the requiered information on the spotify api
    help     Print this message or the help of the given subcommand(s)
    make     Create cuesheet by giving the timestamp in a wizard
```

## CreateM3u

Create create-m3u file

```
$ koto create-m3u --help

koto-create-m3u
Create M3U playlist

USAGE:
    koto create-m3u [OPTIONS] [DIRECTORIES]...

ARGS:
    <DIRECTORIES>...

OPTIONS:
    -e, --exclude-extension <EXCLUDE_EXTENSION>
            Exclude files from being matched

    -h, --help
            Print help information

    -i, --include-extension <INCLUDE_EXTENSION>
            Include files By default, matched files are [mp3, aiff, flac, wav, alac, ogg]

    -o, --output <OUTPUT>
            By default: Print to the standard output
```

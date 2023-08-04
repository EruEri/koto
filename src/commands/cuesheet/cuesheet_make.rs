// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
// This file is part of Koto: A holdall music program                                         //
// Copyright (C) 2023 Yves Ndiaye                                                             //
//                                                                                            //
// Koto is free software: you can redistribute it and/or modify it under the terms            //
// of the GNU General Public License as published by the Free Software Foundation,            //
// either version 3 of the License, or (at your option) any later version.                    //
//                                                                                            //
// Koto is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;          //
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR           //
// PURPOSE.  See the GNU General Public License for more details.                             //
// You should have received a copy of the GNU General Public License along with Koto.         //
// If not, see <http://www.gnu.org/licenses/>.                                                //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

use std::io::{stdin, stdout, Write};

use clap::{ArgEnum, Parser};
use cuesheet_rs::{CueDuration, CueTrack, DurationFormat};

use super::cuesheet_fetch::CueFileFormatLocal;

#[derive(Parser)]
/// Create cuesheet by giving the timestamp in a wizard
pub struct CueSheetMake {
    // /// strategie used for the track timestamp
    // #[clap(short, long, arg_enum)]
    // mode: CueSheetTimestampMode,
    /// Output file
    #[clap(short, long, help = "Output file [stdout if not present]")]
    output: Option<String>,

    /// catalog
    #[clap(short, long)]
    catalog: Option<String>,

    /// Album title
    #[clap(short, long)]
    title: String,

    #[clap(short, long)]
    performer: String,

    #[clap(long, alias = "cfn", default_value = "")]
    cue_file_name: String,

    #[clap(short, long, arg_enum)]
    format: CueFileFormatLocal,

    /// cuesheet tracks' name
    tracks_name: Vec<String>,
}

#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum CueSheetTimestampMode {
    Sum,
    Set,
}

pub struct CueDurationFormatError;

pub struct DurationFormatLocal(DurationFormat);
pub struct CueDurationLocal(CueDuration);

macro_rules! time_segment {
    ($iter:expr) => {{
        let next = $iter.next();
        match next {
            Some(u32) => u32.map_err(|_| CueDurationFormatError)?,
            None => return Err(CueDurationFormatError),
        }
    }};
    ($iter:expr => $d:expr) => {
        match $iter.next() {
            Some(u32) => Some(u32.map_err(|_| CueDurationFormatError)?),
            None => $d,
        }
    };
}

impl std::str::FromStr for DurationFormatLocal {
    type Err = CueDurationFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":").map(|time| u32::from_str(time.trim()));
        let minute = time_segment!(iter);
        let seconde = time_segment!(iter);
        let millieme = time_segment!(iter => None);

        let cueduration = match millieme {
            Some(mil) => DurationFormat::MinSecMil(minute, seconde, mil),
            None => DurationFormat::MinSec(minute, seconde),
        };

        Ok(DurationFormatLocal(cueduration))
    }
}

impl CueSheetMake {
    fn readline(prompt: &str) -> String {
        let () = print!("{}", prompt);
        let _ = stdout().flush();
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        buffer
    }

    fn ask_track(index: u32, track: &str) -> Option<CueTrack> {
        let () = println!("\n{}:", track);
        let index_str = Self::readline("timestamp (MM:SS) or (MM:SS:MM) : ");
        let duration_format = index_str.parse::<DurationFormatLocal>();
        let duration = duration_format.ok()?.0;
        let performer = Self::readline("Perfomer : ");
        let performer = performer.trim();
        let composer = Self::readline("Composer : ");
        let composer = composer.trim();

        let mut cuetrack = CueTrack::new(index, cuesheet_rs::CueTrackMode::AUDIO);
        let _ = cuetrack.add_index(1, duration);
        let () = match performer.is_empty() {
            true => (),
            false => {
                cuetrack.add_performer(&performer);
            }
        };

        let () = match composer.is_empty() {
            true => (),
            false => {
                cuetrack.add_composer(&composer);
            }
        };
        Some(cuetrack)
    }

    pub fn run(self) {
        let Self {
            output,
            catalog,
            title,
            performer,
            cue_file_name,
            format,
            tracks_name,
        } = self;
        let mut cuesheet = cuesheet_rs::CueSheet::new(&cue_file_name, format.to_cuefileformat());
        let mut tracks = vec![];
        let () = for (index, name) in tracks_name.into_iter().enumerate() {
            let cueopt = Self::ask_track(index as u32, &name);
            let () = match cueopt {
                Some(cue) => tracks.push(cue),
                None => {
                    let () = println!("Failing to parse for {}", &name);
                    return;
                }
            };
        };
        let _ = cuesheet.add_title(&title).add_performer(&performer);
        let () = match catalog {
            Some(cata) => {
                let _ = cuesheet.add_catalog(&cata);
            }
            None => (),
        };

        let _ = tracks.into_iter().for_each(|cuetrack| {
            cuesheet.add_track(cuetrack);
        });

        let () = match output {
            Some(output) => match cuesheet.export(true, &output) {
                Ok(()) => (),
                Err(e) => {
                    let () = println!("{}", e);
                    return;
                }
            },
            None => {
                let s = cuesheet.repr(true);
                println!("\n\n{}", s)
            }
        };
    }
}

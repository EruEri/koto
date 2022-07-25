use std::ffi::CStr;

type value = std::os::raw::c_long;


extern "C" {
    pub fn caml_wrapper_starup(argv: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn drt_zero_frame() -> duration;

    pub fn drt_minuts_seconde_format(
        minutes: ::std::os::raw::c_int,
        secondes: ::std::os::raw::c_int,
    ) -> duration;

    pub fn drt_minuts_seconde_milliemes_format(
        minutes: ::std::os::raw::c_int,
        secondes: ::std::os::raw::c_int,
        milliemes: ::std::os::raw::c_int,
    ) -> duration;

    pub fn drt_minuts_seconde_frames_format(
        minutes: ::std::os::raw::c_int,
        secondes: ::std::os::raw::c_int,
        frames: ::std::os::raw::c_int,
    ) -> duration;
}


extern "C" {
    pub fn string_of_duration(duration: duration) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn create_empty_track(
        track_position: ::std::os::raw::c_int,
        mode: cue_track_mode,
    ) -> cue_track;
}
extern "C" {
    pub fn cuetrack_add_pregap(track: *mut cue_track, duration: duration) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_postgap(track: *mut cue_track, duration: duration) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_iscr(
        track: *mut cue_track,
        iscr: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_index(track: *mut cue_track, duration: duration) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_arranger(
        sheet: *mut cue_track,
        arranger: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_composer(
        sheet: *mut cue_track,
        composer: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_disc_id(
        sheet: *mut cue_track,
        disc_id: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_genre(
        sheet: *mut cue_track,
        genre: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_message(
        sheet: *mut cue_track,
        message: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_performer(
        sheet: *mut cue_track,
        performer: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_songwriter(
        sheet: *mut cue_track,
        songwriter: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}
extern "C" {
    pub fn cuetrack_add_title(
        sheet: *mut cue_track,
        title: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;

    pub fn cuetrack_add_toc_info(
        sheet: *mut cue_track,
        toc_info: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;

    pub fn cuetrack_add_toc_info2(
        sheet: *mut cue_track,
        toc_info2: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;

    pub fn cuetrack_add_size_info(
        sheet: *mut cue_track,
        size_info: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;

    pub fn cuetrack_add_rem(
        sheet: *mut cue_track,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    ) -> *mut cue_track;
}

extern "C" {
    pub fn create_empty_sheet(
        file: *const ::std::os::raw::c_char,
        format: cue_file_format,
    ) -> cue_sheet;
}
extern "C" {
    pub fn string_of_cue_sheet(
        sheet: *mut cue_sheet,
        sum: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cuesheet_add_catalog(
        sheet: *mut cue_sheet,
        catalog: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_cd_text_file(
        sheet: *mut cue_sheet,
        filename: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_arranger(
        sheet: *mut cue_sheet,
        arranger: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_composer(
        sheet: *mut cue_sheet,
        composer: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_disc_id(
        sheet: *mut cue_sheet,
        disc_id: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_genre(
        sheet: *mut cue_sheet,
        genre: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_message(
        sheet: *mut cue_sheet,
        message: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_performer(
        sheet: *mut cue_sheet,
        performer: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_songwriter(
        sheet: *mut cue_sheet,
        songwriter: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_title(
        sheet: *mut cue_sheet,
        title: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_toc_info(
        sheet: *mut cue_sheet,
        toc_info: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_toc_info2(
        sheet: *mut cue_sheet,
        toc_info2: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_size_info(
        sheet: *mut cue_sheet,
        size_info: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_rem(
        sheet: *mut cue_sheet,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    ) -> *mut cue_sheet;
}
extern "C" {
    pub fn cuesheet_add_track(sheet: *mut cue_sheet, track: *mut cue_track) -> *mut cue_sheet;
}
extern "C" {
    pub fn cue_sheet_export(
        sheet: *mut cue_sheet,
        output: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct duration {
    value: value
}

impl duration {
    pub fn zero_frame() -> Self {
        unsafe {
            drt_zero_frame()
        }
    }

    pub fn minutes_seconde_format(minutes: i32, secondes: i32) -> Self {
        unsafe {
            drt_minuts_seconde_format(minutes, secondes)
        }
    }

    pub fn minuts_seconde_milliemes_format(minutes: i32, secondes: i32, milliemes: i32) -> Self {
        unsafe {
            drt_minuts_seconde_milliemes_format(minutes, secondes, milliemes)
        }
    }

    pub fn minuts_seconde_frames_format(minutes: i32, secondes: i32, frames : i32) -> Self {
        unsafe {
            drt_minuts_seconde_frames_format(minutes, secondes, frames)
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct cue_track {
    track: value
}

impl cue_track {
    pub fn new_empty_track(track_index: i32, mode: cue_track_mode) -> Self {
        unsafe { create_empty_track(track_index, mode) }
    }

    pub fn add_pregap(&mut self, duration: duration) -> &mut cue_track {
        unsafe {
            &mut *(cuetrack_add_pregap(self as *mut Self, duration))
        }
    }

    pub fn add_postgap(&mut self, duration: duration) -> &mut cue_track {
        unsafe {
            &mut *(cuetrack_add_postgap(self as *mut Self, duration))
        }
    }

    pub fn add_iscr(&mut self, iscr: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(iscr.as_ptr() as *const i8);
            &mut *(cuetrack_add_iscr(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_index(&mut self, duration: duration) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_index(self as *mut Self, duration))
        }
    }

    pub fn add_arranger(&mut self, arranger: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(arranger.as_ptr() as *const i8);
            &mut *(cuetrack_add_arranger(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_composer(&mut self, composer: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(composer.as_ptr() as *const i8);
            &mut *(cuetrack_add_composer(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_disc_id(&mut self, disc_id: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(disc_id.as_ptr() as *const i8);
            &mut *(cuetrack_add_disc_id(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_genre(&mut self, genre: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_genre(self as *mut Self, genre.as_ptr() as *const i8))
        }
    }

    pub fn add_message(&mut self, message: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_message(self as *mut Self, message.as_ptr() as *const i8))
        }
    }

    pub fn add_performer(&mut self, performer: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_performer(self as *mut Self, performer.as_ptr() as *const i8))
        }
    }

    pub fn add_songwritter(&mut self, songwritter: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_songwriter(self as *mut Self, songwritter.as_ptr() as *const i8))
        }
    }

    pub fn add_title(&mut self, title: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_songwriter(self as *mut Self, title.as_ptr() as *const i8))
        }
    }

    pub fn add_toc_info(&mut self, toc_info: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_toc_info(self as *mut Self, toc_info.as_ptr() as *const i8))
        }
    }

    pub fn add_toc_info2(&mut self, toc_info2: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_toc_info2(self as *mut Self, toc_info2.as_ptr() as *const i8))
        }
    }

    pub fn add_size_info(&mut self, size_info: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_size_info(self as *mut Self, size_info.as_ptr() as *const i8))
        }
    }

    pub fn add_rem(&mut self, key: &str, val: &str) -> &mut Self {
        unsafe {
            &mut *(cuetrack_add_rem(self as *mut Self, key.as_ptr() as *const i8, val.as_ptr() as *const i8))
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct cue_sheet {
    sheet: value
}

impl cue_sheet {

    pub fn new_empty_sheet(file_name: &str, format: cue_file_format) -> Self {
        unsafe {
            create_empty_sheet(file_name.as_ptr() as *const i8, format)
        }
    }

    pub fn to_format(&mut self, sum: bool) -> Option<String> {
        unsafe {
            let c_ptr = string_of_cue_sheet(self as *mut Self, sum as i32);
            let c_str = CStr::from_ptr(c_ptr);
            Some(c_str.to_str().ok()?.to_string())
        }
    }

    pub fn export(&mut self, output: &str) -> Option<()> {
        unsafe {
            let status = cue_sheet_export(self as *mut Self, output.as_ptr() as *const i8 );
            if status == 0 {
                Some(())
            } else {
                None
            }
        }
    }

    pub fn add_catalog(&mut self, catalog: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(catalog.as_ptr() as *const i8);
            &mut *(cuesheet_add_catalog(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_cd_text_file(&mut self, cd_text_file: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(cd_text_file.as_ptr() as *const i8);
            &mut *(cuesheet_add_cd_text_file(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_arranger(&mut self, arranger: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(arranger.as_ptr() as *const i8);
            &mut *(cuesheet_add_arranger(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_composer(&mut self, composer: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(composer.as_ptr() as *const i8);
            &mut *(cuesheet_add_composer(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_disc_id(&mut self, disc_id: &str) -> &mut Self {
        unsafe {
            let c = CStr::from_ptr(disc_id.as_ptr() as *const i8);
            &mut *(cuesheet_add_disc_id(self as *mut Self, c.as_ptr()))
        }
    }

    pub fn add_genre(&mut self, genre: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_genre(self as *mut Self, genre.as_ptr() as *const i8))
        }
    }

    pub fn add_message(&mut self, message: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_message(self as *mut Self, message.as_ptr() as *const i8))
        }
    }

    pub fn add_performer(&mut self, performer: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_performer(self as *mut Self, performer.as_ptr() as *const i8))
        }
    }

    pub fn add_songwritter(&mut self, songwritter: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_songwriter(self as *mut Self, songwritter.as_ptr() as *const i8))
        }
    }

    pub fn add_title(&mut self, title: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_songwriter(self as *mut Self, title.as_ptr() as *const i8))
        }
    }

    pub fn add_toc_info(&mut self, toc_info: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_toc_info(self as *mut Self, toc_info.as_ptr() as *const i8))
        }
    }

    pub fn add_toc_info2(&mut self, toc_info2: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_toc_info2(self as *mut Self, toc_info2.as_ptr() as *const i8))
        }
    }

    pub fn add_size_info(&mut self, size_info: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_size_info(self as *mut Self, size_info.as_ptr() as *const i8))
        }
    }

    pub fn add_rem(&mut self, key: &str, val: &str) -> &mut Self {
        unsafe {
            &mut *(cuesheet_add_rem(self as *mut Self, key.as_ptr() as *const i8, val.as_ptr() as *const i8))
        }
    }
}


#[allow(non_camel_case_types)]
#[repr(C)]
pub enum cue_file_format {
    BINARY = 0,
    MOTOROLA,
    AIFF,
    WAVE,
    MP3
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum cue_track_flag {
    PRE = 0,
    DCP,
    _4CH,
    SCMS
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum cue_track_mode {
    AUDIO = 0,
    CDG,
    MODE1_2048,
    MODE1_2352,
    MODE2_2336,
    MODE2_2352,
    CDI_2336,
    CDI_2352,
}
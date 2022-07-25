#ifndef CUE_SHEET_MAKER_H
#define CUE_SHEET_MAKER_H

#include "caml/misc.h"
#define CAML_NAME_SPACE
#include "caml/mlvalues.h"

typedef enum {
    BINARY,
    MOTOROLA,
    AIFF,
    WAVE,
    MP3
} cue_file_format;

typedef enum {
    AUDIO,
    CDG,
    MODE1_2048,
    MODE1_2352,
    MODE2_2336,
    MODE2_2352,
    CDI_2336,
    CDI_2352,
} cue_track_mode;

typedef enum {
    PRE,
    DCP,
    _4CH,
    SCMS
}cue_track_flag;

typedef struct { 
    value sheet;
} cue_sheet;

typedef struct {
    value sheet;
} duration;

typedef struct {
    value track;
} cue_track;

void caml_wrapper_starup(char** argv);

duration drt_zero_frame();
duration drt_minuts_seconde_format(int minutes, int secondes);
duration drt_minuts_seconde_milliemes_format(int minutes, int secondes, int milliemes);
duration drt_minuts_seconde_frames_format(int minutes, int secondes, int frames);
const char* string_of_duration(duration duration);


cue_track create_empty_track(int track_position, cue_track_mode mode);
cue_track* cuetrack_add_pregap(cue_track* track, duration duration);
cue_track* cuetrack_add_postgap(cue_track* track, duration duration);
cue_track* cuetrack_add_iscr(cue_track* track, const char* iscr);
cue_track* cuetrack_add_index(cue_track* track, duration duration);
cue_track* cuetrack_add_arranger(cue_track* sheet, const char* arranger);
cue_track* cuetrack_add_composer(cue_track* sheet, const char* composer);
cue_track* cuetrack_add_disc_id(cue_track* sheet, const char* disc_id);
cue_track* cuetrack_add_genre(cue_track* sheet, const char* genre);
cue_track* cuetrack_add_message(cue_track* sheet, const char* message);
cue_track* cuetrack_add_performer(cue_track* sheet, const char* performer);
cue_track* cuetrack_add_songwriter(cue_track* sheet, const char* songwriter);
cue_track* cuetrack_add_title(cue_track* sheet, const char* title);
cue_track* cuetrack_add_toc_info(cue_track* sheet, const char* toc_info);
cue_track* cuetrack_add_toc_info2(cue_track* sheet, const char* toc_info2);
cue_track* cuetrack_add_size_info(cue_track* sheet, const char* size_info);
cue_track* cuetrack_add_rem(cue_track* sheet, const char* key, const char* val);

cue_sheet create_empty_sheet( const char* file, cue_file_format format);
const char* string_of_cue_sheet(cue_sheet* sheet, int sum);
cue_sheet* cuesheet_add_catalog(cue_sheet* sheet, const char* catalog);
cue_sheet* cuesheet_add_cd_text_file(cue_sheet* sheet, const char* filename);
cue_sheet* cuesheet_add_arranger(cue_sheet* sheet, const char* arranger);
cue_sheet* cuesheet_add_composer(cue_sheet* sheet, const char* composer);
cue_sheet* cuesheet_add_disc_id(cue_sheet* sheet, const char* disc_id);
cue_sheet* cuesheet_add_genre(cue_sheet* sheet, const char* genre);
cue_sheet* cuesheet_add_message(cue_sheet* sheet, const char* message);
cue_sheet* cuesheet_add_performer(cue_sheet* sheet, const char* performer);
cue_sheet* cuesheet_add_songwriter(cue_sheet* sheet, const char* songwriter);
cue_sheet* cuesheet_add_title(cue_sheet* sheet, const char* title);
cue_sheet* cuesheet_add_toc_info(cue_sheet* sheet, const char* toc_info);
cue_sheet* cuesheet_add_toc_info2(cue_sheet* sheet, const char* toc_info2);
cue_sheet* cuesheet_add_size_info(cue_sheet* sheet, const char* size_info);
cue_sheet* cuesheet_add_rem(cue_sheet* sheet, const char* key, const char* val);
cue_sheet* cuesheet_add_track(cue_sheet* sheet, cue_track* track);
int cue_sheet_export(cue_sheet*sheet, const char* output);

#endif
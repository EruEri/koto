
#define CAML_NAME_SPACE

#include <stdio.h>
#include <stdlib.h>

#include "caml/mlvalues.h"
#include "caml/callback.h"
#include "caml/alloc.h"
#include "caml/memory.h"
#include "cue_sheet_maker.h"

#define ocaml_zero_frame "ocaml_zero_frame"
#define ocaml_string_of_duration "ocaml_string_of_duration"
#define ocaml_minute_seconde_format "ocaml_minute_seconde_format"
#define ocaml_minute_seconde_millieme_format "ocaml_minute_seconde_millieme_format"
#define ocaml_minute_seconde_frame_format "ocaml_minute_seconde_frame_format"

#define ocaml_string_of_cue_track "ocaml_string_of_cue_track"
#define ocaml_create_empty_track "ocaml_create_empty_track"
#define ocaml_track_add_index "ocaml_track_add_index"
#define ocaml_track_add_flag "ocaml_track_add_flag"
#define ocaml_track_add_pregap "ocaml_track_add_pregap"
#define ocaml_track_add_postgap "ocaml_track_add_postgap"
#define ocaml_track_add_arranger "ocaml_track_add_arranger"
#define ocaml_track_add_composer "ocaml_track_add_composer"
#define ocaml_track_add_disc_id "ocaml_track_add_disc_id"
#define ocaml_track_add_genre "ocaml_track_add_genre"
#define ocaml_track_add_iscr "ocaml_track_add_iscr"
#define ocaml_track_add_message "ocaml_track_add_message"
#define ocaml_track_add_performer "ocaml_track_add_performer"
#define ocaml_track_add_songwriter "ocaml_track_add_songwriter"
#define ocaml_track_add_title "ocaml_track_add_title"
#define ocaml_track_add_toc_info "ocaml_track_add_toc_info"
#define ocaml_track_add_toc_info2 "ocaml_track_add_toc_info2"
#define ocaml_track_add_size_info "ocaml_track_add_size_info"
#define ocaml_track_add_rem "ocaml_track_add_rem"

#define ocaml_create_empty_sheet "ocaml_create_empty_sheet"
#define ocaml_string_of_cue_sheet "ocaml_string_of_cue_sheet"
#define ocaml_sheet_export "ocaml_sheet_export"
#define ocaml_sheet_add_catalog "ocaml_sheet_add_catalog"
#define ocaml_sheet_add_cd_text_file "ocaml_sheet_add_cd_text_file"
#define ocaml_sheet_add_arranger "ocaml_sheet_add_arranger"
#define ocaml_sheet_add_composer "ocaml_sheet_add_composer"
#define ocaml_sheet_add_disc_id "ocaml_sheet_add_disc_id"
#define ocaml_sheet_add_genre "ocaml_sheet_add_genre"
#define ocaml_sheet_add_message "ocaml_sheet_add_message"
#define ocaml_sheet_add_performer "ocaml_sheet_add_performer"
#define ocaml_sheet_add_songwriter "ocaml_sheet_add_songwriter"
#define ocaml_sheet_add_title "ocaml_sheet_add_title"
#define ocaml_sheet_add_toc_info "ocaml_sheet_add_toc_info"
#define ocaml_sheet_add_toc_info2 "ocaml_sheet_add_toc_info2"
#define ocaml_sheet_add_size_info "ocaml_sheet_add_size_info"
#define ocaml_sheet_add_rem "ocaml_sheet_add_rem"

duration drt_zero_frame() {
    CAMLparam0();
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_zero_frame);
    duration d;
    d.sheet = caml_callbackN(*closure, 0, NULL);
    return d;
}

duration drt_minuts_seconde_format(int minutes, int secondes){
    CAMLparam0();
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_minute_seconde_format);
    duration d;
    d.sheet = caml_callback2(*closure, Val_int(minutes), Val_int(secondes));
    return d;
}

duration drt_minuts_seconde_milliemes_format(int minutes, int secondes, int milliemes){
    CAMLparam0();
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_minute_seconde_millieme_format);
    duration d;
    d.sheet = caml_callback3(*closure, Val_int(minutes), Val_int(secondes), Val_int(milliemes));
    return d;
}

duration drt_minuts_seconde_frames_format(int minutes, int secondes, int frames){
    CAMLparam0();
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_minute_seconde_frame_format);
    duration d;
    d.sheet = caml_callback3(*closure, Val_int(minutes), Val_int(secondes), Val_int(frames));
    return d;
}

const char* string_of_duration(duration duration){
    CAMLparam0();
    CAMLlocal1(str);
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_string_of_duration);
    str = caml_callback(*closure, duration.sheet);
    return String_val(str);
}

cue_track create_empty_track(int track_position, cue_track_mode mode) {
    CAMLparam0();
    CAMLlocal2(tuple, track);
    
    tuple = caml_alloc(2, 0);
    Store_field(tuple, 0, Val_int(track_position));
    Store_field(tuple, 1, Val_int(mode));
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_create_empty_track);
    track = caml_callback(*closure, tuple);
    cue_track sh;
    sh.track = track;
    return sh;
}

cue_track* cuetrack_add_pregap(cue_track* track, duration duration) {
    CAMLparam0();
    if (!track) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_pregap);
    track->track = caml_callback2(*closure, duration.sheet, track->track);
    return track;
}

cue_track* cuetrack_add_postgap(cue_track* track, duration duration) {
    CAMLparam0();
    if (!track) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_postgap);
    track->track = caml_callback2(*closure, duration.sheet, track->track);
    return track;
}

cue_track* cuetrack_add_index(cue_track* track, duration duration) {
    CAMLparam0();
    if (!track) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_index);
    track->track = caml_callback2(*closure, duration.sheet, track->track);
    return track;
}

cue_track* cuetrack_add_iscr(cue_track* track, const char* iscr) {
    CAMLparam0();
    if (!track || !iscr) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_iscr);
    track->track = caml_callback2(*closure, caml_copy_string(iscr), track->track);
    return track;
}

cue_track* cuetrack_add_arranger(cue_track* track, const char* arranger) {
    if (!track || !arranger) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_arranger);
    track->track = caml_callback2(*closure, caml_copy_string(arranger), track->track);
    return track;
}

cue_track* cuetrack_add_composer(cue_track* track, const char* composer) {
    CAMLparam0();
    if (!track || !composer) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_composer);
    track->track = caml_callback2(*closure, caml_copy_string(composer), track->track);
    return track;
}

cue_track* cuetrack_add_disc_id(cue_track* track, const char* disc_id) {
    CAMLparam0();
    if (!track || !disc_id) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_disc_id);
    track->track = caml_callback2(*closure, caml_copy_string(disc_id), track->track);
    return track;
}

cue_track* cuetrack_add_genre(cue_track* track, const char* genre) {
    CAMLparam0();
    if (!track || !genre) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_genre);
    track->track = caml_callback2(*closure, caml_copy_string(genre), track->track);
    return track;
}

cue_track* cuetrack_add_message(cue_track* track, const char* message) {
    CAMLparam0();
    if (!track || !message) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_message);
    track->track = caml_callback2(*closure, caml_copy_string(message), track->track);
    return track;
}

cue_track* cuetrack_add_performer(cue_track* track, const char* performer) {
    CAMLparam0();
    if (!track || !performer) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_performer);
    track->track = caml_callback2(*closure, caml_copy_string(performer), track->track);
    return track;
}

cue_track* cuetrack_add_songwriter(cue_track* track, const char* songwriter) {
    CAMLparam0();
    if (!track || !songwriter) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_songwriter);
    track->track = caml_callback2(*closure, caml_copy_string(songwriter), track->track);
    return track;
}

cue_track* cuetrack_add_title(cue_track* track, const char* title) {
    CAMLparam0();
    if (!track || !title) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_title);
    track->track = caml_callback2(*closure, caml_copy_string(title), track->track);
    return track;
}

cue_track* cuetrack_add_toc_info(cue_track* track, const char* toc_info) {
    CAMLparam0();
    if (!track || !toc_info) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_toc_info);
    track->track = caml_callback2(*closure, caml_copy_string(toc_info), track->track);
    return track;
}

cue_track* cuetrack_add_toc_info2(cue_track* track, const char* toc_info2) {
    CAMLparam0();
    if (!track || !toc_info2) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_toc_info2);
    track->track = caml_callback2(*closure, caml_copy_string(toc_info2), track->track);
    return track;
}

cue_track* cuetrack_add_size_info(cue_track* track, const char* size_info) {
    CAMLparam0();
    if (!track || !size_info) return track;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_size_info);
    track->track = caml_callback2(*closure, caml_copy_string(size_info), track->track);
    return track;
}

cue_track* cuetrack_add_rem(cue_track* track, const char* key, const char* val) {
    
    CAMLparam0();
    CAMLlocal1(tuple);

    if (!track || !key || !val) return track;

    tuple = caml_alloc(2, 0);
    Store_field(tuple, 0, caml_copy_string(key));
    Store_field(tuple, 1, caml_copy_string(val));
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_track_add_rem);
    track->track = caml_callback2(*closure, tuple, track->track);
    return track;
}


/// 

cue_sheet create_empty_sheet( const char* file, cue_file_format format) {
    
    CAMLparam0();
    CAMLlocal2(tuple, sheet);
    
    tuple = caml_alloc(2, 0);
    Store_field(tuple, 0, caml_copy_string(file));
    Store_field(tuple, 1, Val_long(format));
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_create_empty_sheet);
    sheet = caml_callback(*closure, tuple);
    cue_sheet sh;
    sh.sheet = sheet;
    return sh;
}

const char* string_of_cue_sheet(cue_sheet* sheet, int sum) {
    if (!sheet) return NULL;

    CAMLparam0();
    CAMLlocal1(str_value);
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_string_of_cue_sheet);
    str_value = caml_callback2(*closure, Val_int(sum ? 1 : 0), sheet->sheet);
    return String_val(str_value);
}

cue_sheet* cuesheet_add_catalog(cue_sheet* sheet, const char* catalog) {
    CAMLparam0();
    if (!sheet || !catalog) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_catalog);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(catalog), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_cd_text_file(cue_sheet* sheet, const char* filename) {
    CAMLparam0();
    if (!sheet || !filename) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_cd_text_file);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(filename), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_arranger(cue_sheet* sheet, const char* arranger) {
    if (!sheet || !arranger) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_arranger);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(arranger), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_composer(cue_sheet* sheet, const char* composer) {
    CAMLparam0();
    if (!sheet || !composer) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_composer);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(composer), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_disc_id(cue_sheet* sheet, const char* disc_id) {
    CAMLparam0();
    if (!sheet || !disc_id) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_disc_id);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(disc_id), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_genre(cue_sheet* sheet, const char* genre) {
    CAMLparam0();
    if (!sheet || !genre) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_genre);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(genre), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_message(cue_sheet* sheet, const char* message) {
    CAMLparam0();
    if (!sheet || !message) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_message);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(message), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_performer(cue_sheet* sheet, const char* performer) {
    CAMLparam0();
    if (!sheet || !performer) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_performer);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(performer), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_songwriter(cue_sheet* sheet, const char* songwriter) {
    CAMLparam0();
    if (!sheet || !songwriter) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_songwriter);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(songwriter), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_title(cue_sheet* sheet, const char* title) {
    CAMLparam0();
    if (!sheet || !title) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_title);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(title), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_toc_info(cue_sheet* sheet, const char* toc_info) {
    CAMLparam0();
    if (!sheet || !toc_info) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_toc_info);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(toc_info), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_toc_info2(cue_sheet* sheet, const char* toc_info2) {
    CAMLparam0();
    if (!sheet || !toc_info2) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_toc_info2);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(toc_info2), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_size_info(cue_sheet* sheet, const char* size_info) {
    CAMLparam0();
    if (!sheet || !size_info) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_size_info);
    sheet->sheet = caml_callback2(*closure, caml_copy_string(size_info), sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_rem(cue_sheet* sheet, const char* key, const char* val) {
    
    CAMLparam0();
    CAMLlocal1(tuple);

    if (!sheet || !key || !val) return sheet;

    tuple = caml_alloc(2, 0);
    Store_field(tuple, 0, caml_copy_string(key));
    Store_field(tuple, 1, caml_copy_string(val));
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_rem);
    sheet->sheet = caml_callback2(*closure, tuple, sheet->sheet);
    return sheet;
}

cue_sheet* cuesheet_add_track(cue_sheet* sheet, cue_track* track) {
    CAMLparam0();
    if (!sheet || !track) return sheet;
    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_add_rem);
    sheet->sheet = caml_callback2(*closure, track->track, sheet->sheet);
    return sheet;
}

int cue_sheet_export(cue_sheet*sheet, const char* output) {
    CAMLparam0();
    if (!sheet || !output) return -1;

    static const value* closure = NULL;
    if (!closure) closure = caml_named_value(ocaml_sheet_export);
    caml_callback2(*closure, caml_copy_string(output), sheet->sheet);
    return 0;
}
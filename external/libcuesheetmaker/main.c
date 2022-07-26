#include <stdio.h>
#include <stdlib.h>
#include "caml/callback.h"
#include "cue_sheet_maker.h"

int main(int argc, char_os** argv) {
    caml_startup(argv);

    duration d = drt_zero_frame();
    printf("duration %s\n", string_of_duration(d));
    cue_sheet sheet = create_empty_sheet("", WAVE);
    cuesheet_add_genre(&sheet, "J-ROCK");
    cuesheet_add_performer(&sheet, "Kitamura Eri");

    cue_track track = create_empty_track(1, AUDIO);
    cuetrack_add_index(&track, 1, drt_minuts_seconde_milliemes_format(3, 30, 300));

    // cuesheet_add_track(&sheet, &track);
    const char* str_sheet = string_of_cue_sheet(&sheet, 1);
    printf("%s\n", str_sheet);
}
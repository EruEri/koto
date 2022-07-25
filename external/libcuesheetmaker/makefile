RUNTIMEPATH := $(shell printf "%s/libasmrun.a" $(shell ocamlopt -where))

libcuesheetmaker.a: cuesheetmaker.o wrapcuesheet.o 
	cp $(RUNTIMEPATH) product/libcuesheetmaker.a && chmod +w product/libcuesheetmaker.a && ar r product/libcuesheetmaker.a tmp/cuesheetmaker.o tmp/wrapcuesheet.o

wrapcuesheet.o: cue_sheet_maker.c
	ocamlopt -c -o tmp/wrapcuesheet.o cue_sheet_maker.c

cuesheetmaker.o: cue_sheet_maker.mli cue_sheet_maker.ml register.ml
	ocamlopt -output-obj -o tmp/cuesheetmaker.o cue_sheet_maker.mli cue_sheet_maker.ml register.ml

main: main.c libcuesheetmaker.a
	cc -o main main.c product/libcuesheetmaker.a

clean:
	rm *.o tmp/*.o mod.a *.cm*
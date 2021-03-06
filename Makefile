# TODO(ryan): I've changed as/ld to native cross compiler ones.
# Could this cause dependency problems in the future?
AS=as -march=i386 --32
LD=ld -melf_i386 -nostdlib
QEMU=qemu-system-i386
TARGET=i686-unknown-linux-gnu
QEMUARGS=-device rtl8139,vlan=0 -net user,id=net0,vlan=0 -net dump,vlan=0,file=/tmp/rustos-dump.pcap
SRC=src/

.PHONY: all clean cleanproj run debug vb target/$(TARGET)/libstd*.a rustos

all: boot.bin

run: boot.bin
	$(QEMU) $(QEMUARGS) -kernel $<

debug: boot.bin
	$(QEMU) $(QEMUARGS) -S -gdb tcp::3333 -kernel $< &
	gdb $< -ex "target remote :3333" -ex "break _start" -ex "c"

vb: boot.iso
	virtualbox --debug --startvm rustos

rustos: target/$(TARGET)/debug/libstd*.a $(SRC)/*.rs


target/$(TARGET)/debug/libstd*.a: Cargo.toml libmorestack.a libcompiler-rt.a lib_context.a
	cargo rustc --features rustos --target $(TARGET) --verbose -- -L .

boot.bin: $(SRC)/arch/x86/link.ld boot.o target/$(TARGET)/debug/libstd*.a interrupt.o context.o dependencies.o
	$(LD) -o $@ -T $^

boot.iso: boot.bin
	cp boot.bin src/isodir/boot/
	grub-mkrescue -o boot.iso src/isodir

compiler-rt.o: $(SRC)/dummy-compiler-rt.s # needed for staticlib creation
	$(AS)  -o $@ $<

%.s: ../rust/src/rt/arch/i386/%.S
	$(CPP) -o $@ $<

%.o: $(SRC)/arch/x86/%.s
	$(AS)  -o $@ $<

%.o: $(SRC)/%.s
	$(AS)  -o $@ $<

lib%.a: %.o
	ar rcs $@ $<


clean: cleanproj
	cargo clean

cleanproj:
	cargo clean -p rustos
	rm -f *.bin *.img *.iso *.rlib *.a *.so *.o *.s target/$(TARGET)/debug/libstd*.a

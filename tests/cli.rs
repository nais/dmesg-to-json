// std-lib imports
use std::error::Error;

// Non-std lib imports
use assert_cmd::cmd::Command;
use indoc::indoc;
use predicates::prelude::predicate;

#[test]
fn long_help_works() -> Result<(), Box<dyn Error>> {
	let mut cli = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
	cli.arg("--help")
		.assert()
		.success()
		.stdout(predicate::str::contains(env!("CARGO_PKG_DESCRIPTION")));
	Ok(())
}

#[test]
fn skip_irrelevant_lines() -> Result<(), Box<dyn Error>> {
	let mut cli = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
	let inputs = indoc! {"
		2020-07-30T14:02:51,000000+00:00 Command line: BOOT_IMAGE=/boot/vmlinuz-5.3.0-1032-gcp root=LABEL=cloudimg-rootfs ro console=ttyS0
		2020-07-30T14:02:51,000000+00:00 KERNEL supported cpus:
		2020-07-30T14:02:51,000000+00:00   Intel GenuineIntel
		2020-07-30T14:02:51,000000+00:00   AMD AuthenticAMD
		2020-07-30T14:02:51,000000+00:00   Hygon HygonGenuine
		2020-07-30T14:02:51,000000+00:00   Centaur CentaurHauls
		2020-07-30T14:02:51,000000+00:00   zhaoxin   Shanghai
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x001: 'x87 floating point registers'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x002: 'SSE registers'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x004: 'AVX registers'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x008: 'MPX bounds registers'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x010: 'MPX CSR'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x020: 'AVX-512 opmask'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x040: 'AVX-512 Hi256'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Supporting XSAVE feature 0x080: 'AVX-512 ZMM_Hi256'
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[2]:  576, xstate_sizes[2]:  256
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[3]:  832, xstate_sizes[3]:   64
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[4]:  896, xstate_sizes[4]:   64
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[5]:  960, xstate_sizes[5]:   64
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[6]: 1024, xstate_sizes[6]:  512
		2020-07-30T14:02:51,000000+00:00 x86/fpu: xstate_offset[7]: 1536, xstate_sizes[7]: 1024
		2020-07-30T14:02:51,000000+00:00 x86/fpu: Enabled xstate features 0xff, context size is 2560 bytes, using 'compacted' format.
		2020-07-30T14:02:51,000000+00:00 BIOS-provided physical RAM map:
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000000000-0x0000000000000fff] reserved
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000001000-0x0000000000054fff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000055000-0x000000000005ffff] reserved
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000060000-0x0000000000097fff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000098000-0x000000000009ffff] reserved
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x0000000000100000-0x000000003e11dfff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e11e000-0x000000003e120fff] ACPI data
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e121000-0x000000003e121fff] ACPI NVS
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e122000-0x000000003e2d1fff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e2d2000-0x000000003e2d9fff] ACPI NVS
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e2da000-0x000000003e31afff] reserved
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003e31b000-0x000000003f39afff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003f39b000-0x000000003f3f2fff] reserved
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003f3f3000-0x000000003f3fafff] ACPI data
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003f3fb000-0x000000003f3fefff] ACPI NVS
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003f3ff000-0x000000003ffdffff] usable
		2020-07-30T14:02:51,000000+00:00 BIOS-e820: [mem 0x000000003ffe0000-0x000000003fffffff] reserved
		2020-07-30T14:02:51,000000+00:00 NX (Execute Disable) protection: active
	"};

	cli.write_stdin(inputs)
		.assert()
		.failure()
		.stdout(predicate::str::is_empty());
	Ok(())
}

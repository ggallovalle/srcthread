#!/usr/bin/env bun
// vim: set ft=typescript:
//MISE description="Build a platform tarball (mise-shaped layout)"
//USAGE arg "[triple]" help="Rust target triple (default: host)"
//USAGE arg "[variant]" help="Packslip variant" default="default" {
//USAGE   choices "default" "desktop"
//USAGE }
/**
 * Output under dist/:
 *   srcthread-$VER-$os-$arch.tar.gz
 *   srcthread-$VER-$os-$arch-desktop.tar.gz  (when variant=desktop)
 */

import { chmod, mkdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { resolveMiseEnv, type MiseEnv } from "~/mise-env.ts";
import { packslipArch, packslipOs, rustcHostTriple } from "~/rust.ts";
import { readCliVersion } from "~/version.ts";

/** Mirrors //USAGE above. */
export type Command = {
  triple: string | null;
  variant: "default" | "desktop";
};

export function parseCommand(
  env: NodeJS.ProcessEnv = process.env,
): Command | { error: string } {
  const variant = (env.usage_variant?.trim() || "default") as string;
  if (variant !== "default" && variant !== "desktop") {
    return { error: `unknown variant: ${variant} (expected default|desktop)` };
  }
  return {
    triple: env.usage_triple?.trim() || null,
    variant,
  };
}

export async function main(command: Command, mise: MiseEnv): Promise<number> {
  const root = mise.projectRoot;

  let triple = command.triple;
  if (!triple) {
    const host = await rustcHostTriple(root);
    if (typeof host !== "string") {
      console.error(host.error);
      return 1;
    }
    triple = host;
  }

  const os = packslipOs(triple);
  if (typeof os !== "string") {
    console.error(os.error);
    return 1;
  }
  const arch = packslipArch(triple);
  if (typeof arch !== "string") {
    console.error(arch.error);
    return 1;
  }

  const version = await readCliVersion(root);
  if (typeof version !== "string") {
    console.error(version.error);
    return 1;
  }

  const cargoArgs =
    command.variant === "desktop"
      ? (["--features", "desktop"] as const)
      : (["--no-default-features"] as const);
  const suffix = command.variant === "desktop" ? "-desktop" : "";
  const basename = `srcthread-${version}-${os}-${arch}${suffix}`;
  console.log(`Building ${basename} (${triple}, variant=${command.variant})`);

  await Bun.$`cargo build -p srcthread-cli --release --target ${triple} ${cargoArgs}`.cwd(
    root,
  );

  await mkdir(join(root, "man", "man1"), { recursive: true });
  if (!(await Bun.file(join(root, "man/man1/srcthread.1")).exists())) {
    await Bun.$`mise run render:usage`.cwd(root);
  }

  const stage = join(root, "dist", `stage-${basename}`);
  await rm(stage, { recursive: true, force: true });
  await mkdir(join(stage, "srcthread", "bin"), { recursive: true });
  await mkdir(join(stage, "srcthread", "man", "man1"), { recursive: true });

  let binSrc = join(root, "target", triple, "release", "srcthread");
  if (!(await Bun.file(binSrc).exists())) {
    // Host build may omit the triple directory.
    binSrc = join(root, "target", "release", "srcthread");
  }
  const binDst = join(stage, "srcthread", "bin", "srcthread");
  await Bun.write(binDst, Bun.file(binSrc));
  await chmod(binDst, 0o755);
  await Bun.write(
    join(stage, "srcthread", "man", "man1", "srcthread.1"),
    Bun.file(join(root, "man/man1/srcthread.1")),
  );
  await Bun.write(
    join(stage, "srcthread", "README.md"),
    Bun.file(join(root, "README.md")),
  );
  await Bun.write(
    join(stage, "srcthread", "LICENSE"),
    Bun.file(join(root, "LICENSE")),
  );

  await mkdir(join(root, "dist"), { recursive: true });
  const out = join(root, "dist", `${basename}.tar.gz`);

  const tarVer = await Bun.$`tar --version`.nothrow().text();
  if (tarVer.includes("bsdtar")) {
    await Bun.$`tar --uid 0 --gid 0 --uname root --gname root -C ${stage} -czf ${out} srcthread`.cwd(
      root,
    );
  } else {
    await Bun.$`tar --owner=0 --group=0 -C ${stage} -czf ${out} srcthread`.cwd(
      root,
    );
  }
  await rm(stage, { recursive: true, force: true });
  await Bun.$`ls -lh ${out}`.cwd(root);
  return 0;
}

if (import.meta.main) {
  const parsed = parseCommand();
  if ("error" in parsed) {
    console.error(parsed.error);
    process.exit(2);
  }
  process.exit(await main(parsed, resolveMiseEnv()));
}

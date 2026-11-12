#!/usr/bin/env bun
// vim: set ft=typescript:
//MISE description="Print the lockstep product version (workspace / srcthread-cli)"
/**
 * Prints the current product version to stdout.
 *
 * Prefer: mise run release:cli-version
 * From shell tasks: bun "$MISE_PROJECT_ROOT/mise/tasks/release/cli-version"
 */

import { resolveMiseEnv, type MiseEnv } from "~/mise-env.ts";
import { readCliVersion } from "~/version.ts";

export type { MiseEnv };

export async function main(
  _command: Record<string, never>,
  mise: MiseEnv,
): Promise<number> {
  const version = await readCliVersion(mise.projectRoot);
  if (typeof version !== "string") {
    console.error(version.error);
    return 1;
  }
  console.log(version);
  return 0;
}

if (import.meta.main) {
  process.exit(await main({}, resolveMiseEnv()));
}

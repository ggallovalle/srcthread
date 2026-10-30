import { join } from "node:path";

type CargoToml = {
  workspace?: { package?: { version?: string } };
};

const WORKSPACE_PACKAGE_RE = /^\[workspace\.package\][\s\S]*?(?=^\[|\Z)/m;
const WORKSPACE_VERSION_RE = /^version\s*=\s*"([^"]*)"/m;

/** Read [workspace.package].version from Cargo.toml. */
export async function readWorkspaceVersion(
  projectRoot: string,
): Promise<string | { error: string }> {
  const path = join(projectRoot, "Cargo.toml");
  const file = Bun.file(path);
  if (!(await file.exists())) return { error: `missing ${path}` };
  const parsed = Bun.TOML.parse(await file.text()) as CargoToml;
  const version = parsed.workspace?.package?.version?.trim();
  if (!version) {
    return {
      error: "could not find [workspace.package] version in Cargo.toml",
    };
  }
  return version;
}

/**
 * Prefer cargo metadata for srcthread-cli (authoritative packaged version);
 * fall back to workspace Cargo.toml when cargo is unavailable.
 */
export async function readCliVersion(
  projectRoot: string,
): Promise<string | { error: string }> {
  try {
    const meta = await Bun.$`cargo metadata --no-deps --format-version 1`
      .cwd(projectRoot)
      .json();
    const pkg = (
      meta as { packages: Array<{ name: string; version: string }> }
    ).packages.find((p) => p.name === "srcthread-cli");
    if (pkg?.version) return pkg.version;
  } catch {
    // fall through to Cargo.toml
  }
  return readWorkspaceVersion(projectRoot);
}

/** Surgical edit so we keep Cargo.toml formatting/comments. */
export async function writeWorkspaceVersion(
  projectRoot: string,
  version: string,
): Promise<string | null> {
  const cargoToml = join(projectRoot, "Cargo.toml");
  const file = Bun.file(cargoToml);
  const text = await file.text();
  let n = 0;
  const newText = text.replace(WORKSPACE_PACKAGE_RE, (block) => {
    n += 1;
    if (WORKSPACE_VERSION_RE.test(block)) {
      return block.replace(WORKSPACE_VERSION_RE, `version = "${version}"`);
    }
    return block.replace(
      /^(\[workspace\.package\]\n)/m,
      `$1version = "${version}"\n`,
    );
  });
  if (n !== 1) return "failed to update [workspace.package] version";
  await Bun.write(cargoToml, newText);
  return null;
}

export async function writeAndroidVersion(
  projectRoot: string,
  version: string,
  code: number,
): Promise<string | null> {
  const gradle = join(projectRoot, "apps", "android", "build.gradle.kts");
  const file = Bun.file(gradle);
  let text = await file.text();
  let n1 = 0;
  let n2 = 0;
  text = text.replace(
    /^(\s*)versionName\s*=\s*"[^"]*"/m,
    (_match, indent: string) => {
      n1 += 1;
      return `${indent}versionName = "${version}"`;
    },
  );
  text = text.replace(
    /^(\s*)versionCode\s*=\s*\d+/m,
    (_match, indent: string) => {
      n2 += 1;
      return `${indent}versionCode = ${code}`;
    },
  );
  if (n1 !== 1 || n2 !== 1) {
    return `failed to update Android version (versionName=${n1}, versionCode=${n2})`;
  }
  await Bun.write(gradle, text);
  return null;
}

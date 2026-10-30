const SEMVER_RE = /^([0-9]+)\.([0-9]+)\.([0-9]+)$/;

export type SemverParts = [number, number, number];

export function parseSemver(
  text: string,
): SemverParts | { error: string } {
  const m = SEMVER_RE.exec(text);
  if (!m) {
    return {
      error: `version must be X.Y.Z digits (got ${JSON.stringify(text)})`,
    };
  }
  return [Number(m[1]), Number(m[2]), Number(m[3])];
}

export function formatSemver(parts: SemverParts): string {
  return `${parts[0]}.${parts[1]}.${parts[2]}`;
}

/** Android versionCode encoding: major*10000 + minor*100 + patch. */
export function versionCode(parts: SemverParts): number {
  const [major, minor, patch] = parts;
  return major * 10000 + minor * 100 + patch;
}

export function bumpSemver(
  current: string,
  kind: "major" | "minor" | "patch",
): string | { error: string } {
  const parsed = parseSemver(current);
  if ("error" in parsed) return parsed;
  let [major, minor, patch] = parsed;
  if (kind === "major") {
    major += 1;
    minor = 0;
    patch = 0;
  } else if (kind === "minor") {
    minor += 1;
    patch = 0;
  } else {
    patch += 1;
  }
  return formatSemver([major, minor, patch]);
}

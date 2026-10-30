/** Host triple from `rustc -vV` (`host: …`). */
export async function rustcHostTriple(
  cwd: string,
): Promise<string | { error: string }> {
  try {
    const text = await Bun.$`rustc -vV`.cwd(cwd).text();
    const m = /^host:\s*(\S+)/m.exec(text);
    if (!m?.[1]) return { error: "could not parse rustc host triple" };
    return m[1];
  } catch (err) {
    return { error: `rustc -vV failed: ${String(err)}` };
  }
}

export function packslipOs(triple: string): string | { error: string } {
  if (triple.includes("-apple-darwin")) return "macos";
  if (triple.includes("-linux-")) return "linux";
  return { error: `unsupported OS in triple: ${triple}` };
}

export function packslipArch(triple: string): string | { error: string } {
  if (triple.startsWith("aarch64-")) return "arm64";
  if (triple.startsWith("x86_64-")) return "x64";
  return { error: `unsupported arch in triple: ${triple}` };
}

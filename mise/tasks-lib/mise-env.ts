import { join } from "node:path";

export type MiseEnv = {
  /** Prefer MISE_PROJECT_ROOT, then MISE_CONFIG_ROOT, else walk from this file. */
  projectRoot: string;
  taskFile: string;
  taskDir: string;
  originalCwd: string;
};

export function resolveMiseEnv(env: NodeJS.ProcessEnv = process.env): MiseEnv {
  const projectRoot =
    env.MISE_PROJECT_ROOT?.trim() ||
    env.MISE_CONFIG_ROOT?.trim() ||
    join(import.meta.dir, "..", "..");
  return {
    projectRoot,
    taskFile: env.MISE_TASK_FILE?.trim() || import.meta.path,
    taskDir: env.MISE_TASK_DIR?.trim() || import.meta.dir,
    originalCwd: env.MISE_ORIGINAL_CWD?.trim() || process.cwd(),
  };
}

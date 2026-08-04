// ponytail: dedup openUrl fallback, 2 callsites in ProjectCard
import { openUrl } from '@tauri-apps/plugin-opener'

export async function openExternal(url: string): Promise<void> {
  try {
    await openUrl(url)
  } catch {
    window.open(url, '_blank')
  }
}

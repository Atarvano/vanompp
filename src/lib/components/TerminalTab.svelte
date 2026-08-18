<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { services } from '$lib/stores/services'
  import { t } from '$lib/utils/messages'

  $: tt = $t
  $: mysqlOn = $services.mysql
  $: mysqlPort = $services.mysqlPort ?? 3306

  let sql = ''
  let dbName = ''
  let out = ''
  let err = ''
  let loading = false
  let history: { sql: string; out?: string; err?: string }[] = []

  const quick = [
    'SHOW DATABASES;',
    'SHOW TABLES;',
    'SELECT 1;',
    'SELECT DATABASE();'
  ]

  async function run() {
    const q = sql.trim()
    if (!q) return
    loading = true
    err = ''
    out = ''
    try {
      const res = await invoke<string>('exec_sql', {
        sql: q,
        mysql_port: mysqlPort,
        mysqlPort: mysqlPort,
        db: dbName.trim() || null
      } as any)
      out = res
      history = [...history, { sql: q, out: res }].slice(-20)
    } catch (e) {
      const msg = typeof e === 'string' ? e : String(e)
      err = msg
      history = [...history, { sql: q, err: msg }].slice(-20)
    } finally {
      loading = false
    }
  }

  function useQuick(q: string) {
    sql = q
  }

  function onKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault()
      run()
    }
  }
</script>

<div class="space-y-3">
  {#if !mysqlOn}
    <div class="rounded-xl border border-amber-200 bg-amber-50 px-3 py-2 text-[11px] font-mono text-amber-800">
      {tt('mysqlOff')} — Terminal butuh MySQL ON
    </div>
  {/if}

  <div class="flex flex-wrap gap-1.5">
    {#each quick as q}
      <button
        class="rounded-full border border-zinc-200 bg-white px-2.5 py-1 text-[11px] font-mono text-zinc-700 hover:bg-zinc-50"
        on:click={() => useQuick(q)}
      >{q}</button>
    {/each}
  </div>

  <div class="flex items-center gap-2">
    <input
      bind:value={dbName}
      placeholder="DB (optional)"
      class="h-8 w-[140px] rounded-full border border-zinc-200 bg-white px-3 text-[12px] font-mono text-zinc-900 placeholder:text-zinc-400 outline-none focus:border-zinc-300"
    />
    <span class="text-[10px] font-mono text-zinc-400">port {mysqlPort}</span>
  </div>

  <div class="overflow-hidden rounded-xl border border-zinc-200 bg-zinc-950">
    <textarea
      bind:value={sql}
      on:keydown={onKeydown}
      placeholder="Tulis SQL di sini — Ctrl+Enter untuk jalankan. Contoh: SHOW DATABASES;"
      class="min-h-[96px] w-full resize-y bg-transparent p-3 font-mono text-[12px] leading-relaxed text-zinc-100 placeholder:text-zinc-500 outline-none"
      spellcheck="false"
    ></textarea>
    <div class="flex items-center justify-between border-t border-zinc-800 bg-zinc-900 px-3 py-2">
      <span class="font-mono text-[10px] text-zinc-500">Ctrl+Enter • ; di akhir opsional</span>
      <button
        class="rounded-full bg-white px-4 py-1.5 text-[12px] font-medium text-black hover:bg-zinc-100 disabled:opacity-40"
        disabled={loading || !sql.trim()}
        on:click={run}
      >
        {loading ? '...' : 'Jalankan ▶'}
      </button>
    </div>
  </div>

  {#if err}
    <pre class="rounded-xl border border-red-200 bg-red-50 p-3 font-mono text-[11px] leading-relaxed text-red-700 whitespace-pre-wrap break-words">{err}</pre>
  {:else if out}
    <pre class="rounded-xl border border-zinc-200 bg-zinc-50 p-3 font-mono text-[11px] leading-relaxed text-zinc-800 whitespace-pre-wrap break-words max-h-[280px] overflow-auto">{out}</pre>
  {/if}

  {#if history.length}
    <div class="space-y-2">
      <div class="text-[10px] font-mono uppercase tracking-[0.14em] text-zinc-500">Riwayat</div>
      {#each history.slice().reverse() as h}
        <div class="rounded-xl border border-zinc-200 bg-white p-2.5">
          <div class="font-mono text-[11px] text-zinc-700 whitespace-pre-wrap break-words">{h.sql}</div>
          {#if h.err}
            <div class="mt-1.5 rounded-lg bg-red-50 border border-red-200 p-2 font-mono text-[10px] text-red-700 whitespace-pre-wrap break-words">{h.err}</div>
          {:else if h.out}
            <pre class="mt-1.5 rounded-lg bg-zinc-50 border border-zinc-200 p-2 font-mono text-[10px] text-zinc-600 whitespace-pre-wrap break-words max-h-[160px] overflow-auto">{h.out}</pre>
          {/if}
        </div>
      {/each}
      <button class="text-[11px] text-zinc-500 hover:text-zinc-700" on:click={() => (history = [])}>Hapus riwayat</button>
    </div>
  {/if}
</div>

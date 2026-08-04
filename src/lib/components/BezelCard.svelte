<script lang="ts">
  export let highlight = false
</script>
<div class="bg-white/[0.04] ring-1 ring-white/10 p-1.5 rounded-[1.75rem] {highlight ? 'ring-volt/30' : ''} transition-all">
  <div class="bg-zinc-900 rounded-[calc(1.75rem-6px)] shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)] p-5 md:p-6">
    <slot />
  </div>
</div>

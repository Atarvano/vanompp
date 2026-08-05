<script lang="ts">
  export let highlight = false
</script>

<div
  class="bg-white border border-zinc-200 rounded-[24px] p-5 md:p-6 shadow-[0_1px_2px_rgba(0,0,0,0.04)] transition-all {highlight
    ? 'ring-2 ring-[#E9FF70] ring-offset-2 ring-offset-[#fdfdf9]'
    : ''}"
>
  <slot />
</div>

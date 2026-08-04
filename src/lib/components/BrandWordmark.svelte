<script lang="ts">
  export let size: 'sm'|'md'|'lg' = 'md'
</script>
<div class="flex items-center gap-2.5">
  <div class="w-8 h-8 rounded-[0.65rem] bg-zinc-900 ring-1 ring-white/10 flex items-center justify-center relative" class:w-7={size==='sm'}  class:h-7={size==='sm'}  class:w-10={size==='lg'} class:h-10={size==='lg'}>
    <span class="font-mono font-bold text-[13px] tracking-tight text-white" class:text-[12px]={size==='sm'} class:text-[15px]={size==='lg'}>V/</span>
    <span class="absolute -bottom-0.5 -right-0.5 w-2 h-2 bg-volt rounded-full ring-2 ring-zinc-900"></span>
  </div>
  <span class="font-sans text-[15px] font-bold tracking-[-0.02em] text-white lowercase" class:text-[13px]={size==='sm'} class:text-[18px]={size==='lg'}>vanompp</span>
</div>

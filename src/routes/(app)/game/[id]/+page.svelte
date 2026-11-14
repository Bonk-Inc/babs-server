<script lang="ts">
  import * as Card from "#lib/components/ui/card/index.ts";
  import { Joystick } from "@lucide/svelte";

  let { data } = $props();
</script>

<div class="flex h-12 justify-start gap-3">
  <div class="flex items-center justify-center border rounded-xl ring-foreground/10 bg-card w-12 h-12">
    <Joystick />
  </div>
  <div class="flex gap-1 flex-col">
    <p class="leading-7 text-lg not-first:mt-6">
      {data.game?.name}
    </p>
    <p class="text-sm leading-none font-medium">Game overzicht</p>
  </div>
</div>
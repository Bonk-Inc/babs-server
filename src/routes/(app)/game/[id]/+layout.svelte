<script lang="ts">
  import * as NavigationMenu from "#lib/components/ui/navigation-menu/index.ts";
  import { House } from "@lucide/svelte";
  import type { LayoutProps } from "./$types";
  import { page } from "$app/state";
  import { cn } from "cn";

  let { children, data }: LayoutProps = $props();

  const navigationMenuLinkStyle = (active: boolean) => cn(
    'border-b-2 border-transparent rounded-none hover:border-foreground',
    { 'border-foreground': active }
  )

  let menuItems = [
    {
      path: `/game/${data.game.id}`,
      label: 'Home',
      icon: House
    }
  ]
</script>

<div class="border-b">
  <NavigationMenu.Root>
    <NavigationMenu.List>
      {#each menuItems as item}
        <NavigationMenu.Item>
          <NavigationMenu.Link href={item.path} class={navigationMenuLinkStyle(page.params.id == data.game.id)}>
            <item.icon /> {item.label}
          </NavigationMenu.Link>
        </NavigationMenu.Item>
      {/each}
    </NavigationMenu.List>
  </NavigationMenu.Root>
</div>
<div class="p-4">
  {@render children()}
</div>


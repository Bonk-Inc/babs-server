<script lang="ts">
  import { DatabaseZap, Joystick } from '@lucide/svelte';
  import * as Sidebar from '#lib/components/ui/sidebar/index.ts';
  import type { GameDTO } from '#lib/dtos/GameDTO.ts';

  interface Props {
    games: GameDTO[]
  }

  let { games }: Props = $props();
</script>

<Sidebar.Root>
  <Sidebar.Header class="border-sidebar-border h-16 border-b">
    <Sidebar.MenuItem>
      <Sidebar.MenuButton size="lg">
        {#snippet child({ props })}
          <a href="/dashboard" class={`${props.class} justify-center`}>
            <DatabaseZap size="48" class="size-12" />
            <span class="text-xl font-medium tracking-wide">BABS</span>
          </a>
        {/snippet}
      </Sidebar.MenuButton>
    </Sidebar.MenuItem>
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel class="justify-between text-sm">
        Games
      </Sidebar.GroupLabel>
      <Sidebar.Menu>
        {#each games as game (game.id)}
          <Sidebar.MenuItem>
            <Sidebar.MenuButton>
              {#snippet child({ props })}
                <a href={`/games/${game.id}`} {...props}>
                  <Joystick />
                  <span>{game.name}</span>
                </a>
              {/snippet}
            </Sidebar.MenuButton>
          </Sidebar.MenuItem>
        {/each}
      </Sidebar.Menu>
    </Sidebar.Group>
  </Sidebar.Content>
</Sidebar.Root>
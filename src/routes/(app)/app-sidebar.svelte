<script lang="ts">
  import { DatabaseZap, Joystick } from '@lucide/svelte';
  import * as Sidebar from '#lib/components/ui/sidebar/index';
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
        <Sidebar.MenuItem>
          <Sidebar.MenuButton>
            {#snippet child({ props })}
               <a href="/" {...props}>
                <Joystick />
                <span>Test</span>
              </a>
            {/snippet}
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Group>
  </Sidebar.Content>
</Sidebar.Root>
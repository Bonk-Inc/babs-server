<script lang="ts">
  import * as Sidebar from '#lib/components/ui/sidebar/index.ts';
	import type { LayoutProps } from './$types';
  import AppHeader from './app-header.svelte';
  import AppSidebar from './app-sidebar.svelte';

  let { children }: LayoutProps = $props();
</script>

<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset>
    <AppHeader />
    <main>
      {@render children()}
    </main>
  </Sidebar.Inset>
</Sidebar.Provider>
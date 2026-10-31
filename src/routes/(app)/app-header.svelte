<script lang="ts">
  import { useSession } from '#lib/auth-client.ts';
	import * as Avatar from '#lib/components/ui/avatar/index.ts';
  import * as DropdownMenu from '#lib/components/ui/dropdown-menu/index.ts';
  import { LogOut } from '@lucide/svelte';

  const session = useSession();

  const avatarFallback = (name: string) => name.split(' ')
    .map(s => s[0]?.toUpperCase())
    .join(); 

  const handleSignOut = () => {

  }
</script>

<header class="flex justify-end h-16 gap-2 border-b px-4 items-center">
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      <Avatar.Root>
        <Avatar.Image src={$session.data?.user.image} alt={$session.data?.user.name} />
        <Avatar.Fallback>{avatarFallback($session.data?.user.name ?? '')}</Avatar.Fallback>
      </Avatar.Root>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
      <DropdownMenu.Label>
        <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
          <div class="grid flex-1 text-left text-sm leading-tight">
            <span class="truncate font-medium">{$session.data?.user.name}</span>
            <span class="truncate text-xs">{$session.data?.user.email}</span>
          </div>
        </div>
      </DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item onSelect={handleSignOut}>
        <LogOut />
        Uitloggen
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</header>
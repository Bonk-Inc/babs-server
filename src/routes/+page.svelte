<script lang="ts">
  import * as Card from "#lib/components/ui/card/index.ts";
  import { DatabaseZap, KeySquare, LoaderCircle } from "@lucide/svelte";
  import { buttonVariants } from "#lib/components/ui/button";
  import { signIn, useSession } from "#lib/auth-client";
  import { goto } from "$app/navigation";

  const session = useSession();
  let loading = $state(false);

  if(session) {
    goto('/dashboard');
  }

  const handleSignIn = async () => {
    loading = true;
    await signIn.social({
      provider: "bonk-sso",
      callbackURL: "/",
    });
  };
</script>

<main class="flex h-screen w-screen items-center justify-center">
  <Card.Root class="w-lg">
    <Card.Header class="text-center">
      <Card.Title class="mb-3 flex items-center justify-center text-3xl">
        <DatabaseZap size="48" class="mr-2" /> BABS
      </Card.Title>
      <Card.Description class="text-primary text-bold mt-3 text-xl">
        Inloggen in Bonk Inc. Backend System
      </Card.Description>
    </Card.Header>
    <Card.Content class="my-6 flex items-center justify-center">
      <button
        class={`${buttonVariants({ variant: "default", size: "lg" })} w-full cursor-pointer`}
        onclick={async () => await handleSignIn()}
      >
        {#if loading}
          <LoaderCircle class="size-6 animate-spin" />
        {:else}
          <KeySquare class="mr-2 size-6" /> Inloggen via Authentik
        {/if}
      </button>
    </Card.Content>
  </Card.Root>
</main>

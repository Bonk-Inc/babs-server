<script lang="ts">
  import * as Card from "#lib/components/ui/card/index.ts";
  import * as Form from "#lib/components/ui/form/index.ts";
  import { Input } from "#lib/components/ui/sidebar/index.js";
  import { Save } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { untrack } from "svelte";
  import { superForm } from "sveltekit-superforms";
  import { zod4Client } from "sveltekit-superforms/adapters";
  import { toast } from "svelte-sonner";
  import { gameFormSchema } from "../schema";

  let { data } = $props();

  const form = superForm(untrack(() => data.form), {
    validators: zod4Client(gameFormSchema),
    onUpdated({ form }) {
      if(form.valid) {
        toast.success('Game toegevoegd');
       
        goto(`/game/${form.message.id}`);
      }
    }
  });

  const { form: formData, enhance } = form;
</script>

<div class="flex flex-col items-center justify-center w-full h-full p-4">
  <Card.Root class="w-full max-w-4xl">
    <Card.Header>
      <Card.Title class="text-xl">
        Maak een nieuw spel aan
      </Card.Title>
    </Card.Header>
    <Card.Content>
      <form method="POST" action="/game?/create" class="w-full space-y-6" use:enhance>
        <Form.Field {form} name="name">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Naam</Form.Label>
              <Input {...props} bind:value={$formData.name} />
            {/snippet}
          </Form.Control>
          <Form.Description>Naam van de toe te voegen game.</Form.Description>
          <Form.FieldErrors />
        </Form.Field>
        <Form.Button>
          <Save />
          Opslaan
        </Form.Button>
      </form>
    </Card.Content>
  </Card.Root>
</div>
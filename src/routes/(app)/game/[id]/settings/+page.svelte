<script lang="ts">
  import * as Form from "#lib/components/ui/form/index.ts";
  import { Input } from "#lib/components/ui/sidebar/index.js";
  import { Save } from "@lucide/svelte";
  import { untrack } from "svelte";
  import { toast } from "svelte-sonner";
  import { superForm } from "sveltekit-superforms";
  import { zod4Client } from "sveltekit-superforms/adapters";
  import { gameFormSchema } from "../../schema";

  let { data } = $props();

  const form = superForm(
    untrack(() => data.form),
    {
      resetForm: false,
      validators: zod4Client(gameFormSchema),
      onUpdated({ form }) {
        if (form.valid) {
          toast.success("Game geupdate");
        }
      },
    },
  );

  const { form: formData, enhance } = form;
</script>

<h1 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Instellingen
</h1>
<form method="POST" action="/game?/update&id={data.game.id}" class="flex flex-wrap justify-between" use:enhance>
  <div class="w-full">
    <Form.Field {form} name="name" class="m-0 w-1/3">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label>Naam</Form.Label>
          <Input {...props} bind:value={$formData.name} />
        {/snippet}
      </Form.Control>
      <Form.Description>Naam van de toe te voegen game.</Form.Description>
      <Form.FieldErrors />
    </Form.Field>
  </div>
  <div class="flex w-full items-end justify-start mt-4">
    <Form.Button>
      <Save />
      Opslaan
    </Form.Button>
  </div>
</form>
